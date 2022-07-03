use std::sync::Arc;

use ahash::AHashMap;
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    transform::TransformSystem,
};
use de_core::{
    objects::{MovableSolid, ObjectType, StaticSolid},
    projection::ToFlat,
    state::GameState,
};
use de_map::size::MapBounds;
use de_objects::{IchnographyCache, ObjectCache};
use futures_lite::future;
use iyes_loopless::prelude::*;

use crate::{
    exclusion::ExclusionArea, finder::PathFinder, path::PathResult, triangulation::triangulate,
    PathQueryProps, PathTarget,
};

pub struct PathingPlugin;

impl Plugin for PathingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UpdateFinderState>()
            .init_resource::<UpdatePathsState>()
            .add_event::<UpdateEntityPath>()
            .add_event::<PathFinderUpdated>()
            .add_enter_system(GameState::Playing, setup)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                check_update_result.run_in_state(GameState::Playing),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                check_removed.run_in_state(GameState::Playing),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                check_updated
                    .run_in_state(GameState::Playing)
                    .label("check_updated")
                    .after(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update
                    .run_in_state(GameState::Playing)
                    .after("check_updated"),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_existing_paths
                    .run_in_state(GameState::Playing)
                    .label("update_existing_paths")
                    .after(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_requested_paths
                    .run_in_state(GameState::Playing)
                    .after(TransformSystem::TransformPropagate)
                    .after("update_existing_paths"),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                check_path_results.run_in_state(GameState::Playing),
            );
    }
}

/// This event triggers computation of shortest path to a target and
/// replacement / insertion of this path to the entity.
pub struct UpdateEntityPath {
    entity: Entity,
    target: PathTarget,
}

impl UpdateEntityPath {
    /// # Arguments
    ///
    /// * `entity` - entity whose path should be updated / inserted.
    ///
    /// * `target` - desired path target & path searching query configuration.
    pub fn new(entity: Entity, target: PathTarget) -> Self {
        Self { entity, target }
    }

    fn entity(&self) -> Entity {
        self.entity
    }

    fn target(&self) -> PathTarget {
        self.target
    }
}

/// This event is sent whenever the path finder is updated.
///
/// Paths found before the event was sent may no longer be optimal or may lead
/// through non-accessible area.
struct PathFinderUpdated;

struct UpdateFinderState {
    invalid: bool,
    task: Option<Task<PathFinder>>,
}

impl UpdateFinderState {
    fn invalidate(&mut self) {
        self.invalid = true;
    }

    fn should_update(&self) -> bool {
        self.invalid && self.task.is_none()
    }

    fn spawn_update<'a, T>(
        &mut self,
        pool: &AsyncComputeTaskPool,
        cache: ObjectCache,
        bounds: MapBounds,
        entities: T,
    ) where
        T: Iterator<Item = (&'a GlobalTransform, &'a ObjectType)>,
    {
        debug_assert!(self.task.is_none());
        let entities: Vec<(GlobalTransform, ObjectType)> = entities
            .map(|(transform, object_type)| (*transform, *object_type))
            .collect();
        self.task = Some(pool.spawn(async move { create_finder(cache, bounds, entities) }));
        self.invalid = false;
    }

    fn check_result(&mut self) -> Option<PathFinder> {
        let finder = self
            .task
            .as_mut()
            .and_then(|task| future::block_on(future::poll_once(task)));
        if finder.is_some() {
            self.task = None;
        }
        finder
    }
}

impl Default for UpdateFinderState {
    fn default() -> Self {
        Self {
            invalid: true,
            task: None,
        }
    }
}

struct UpdatePathsState {
    tasks: AHashMap<Entity, Task<Option<PathResult>>>,
}

impl UpdatePathsState {
    fn spawn_new(
        &mut self,
        pool: &AsyncComputeTaskPool,
        finder: Arc<PathFinder>,
        entity: Entity,
        source: Vec2,
        target: PathTarget,
    ) {
        let task = pool.spawn(async move { finder.find_path(source, target) });
        self.tasks.insert(entity, task);
    }

    fn check_results(&mut self) -> Vec<(Entity, Option<PathResult>)> {
        let mut results = Vec::new();
        self.tasks.retain(
            |&entity, task| match future::block_on(future::poll_once(task)) {
                Some(path) => {
                    results.push((entity, path));
                    false
                }
                None => true,
            },
        );

        results
    }
}

impl Default for UpdatePathsState {
    fn default() -> Self {
        Self {
            tasks: AHashMap::new(),
        }
    }
}

type ChangedQuery<'world, 'state> =
    Query<'world, 'state, Entity, (With<StaticSolid>, Changed<GlobalTransform>)>;

fn setup(mut commands: Commands, bounds: Res<MapBounds>) {
    commands.insert_resource(Arc::new(PathFinder::new(bounds.as_ref())));
}

fn check_removed(mut state: ResMut<UpdateFinderState>, removed: RemovedComponents<StaticSolid>) {
    if removed.iter().next().is_some() {
        state.invalidate();
    }
}

fn check_updated(mut state: ResMut<UpdateFinderState>, changed: ChangedQuery) {
    if changed.iter().next().is_some() {
        state.invalidate();
    }
}

fn update(
    mut state: ResMut<UpdateFinderState>,
    pool: Res<AsyncComputeTaskPool>,
    bounds: Res<MapBounds>,
    cache: Res<ObjectCache>,
    entities: Query<(&GlobalTransform, &ObjectType), With<StaticSolid>>,
) {
    if state.should_update() {
        info!("Spawning path finder update task");
        state.spawn_update(pool.as_ref(), cache.clone(), *bounds, entities.iter());
    }
}

fn check_update_result(
    mut commands: Commands,
    mut state: ResMut<UpdateFinderState>,
    mut pf_updated: EventWriter<PathFinderUpdated>,
) {
    if let Some(finder) = state.check_result() {
        info!("Inserting updated path finder");
        commands.insert_resource(Arc::new(finder));
        pf_updated.send(PathFinderUpdated);
    }
}

/// Creates a new path finder by triangulating accessible area on the map.
// This function has to be public due to its benchmark.
pub fn create_finder(
    cache: impl IchnographyCache,
    bounds: MapBounds,
    entities: Vec<(GlobalTransform, ObjectType)>,
) -> PathFinder {
    debug!(
        "Going to create a new path finder from {} entities",
        entities.len()
    );
    let exclusions = ExclusionArea::build(cache, entities.as_slice());
    let triangles = triangulate(&bounds, exclusions.as_slice());
    PathFinder::from_triangles(triangles, exclusions)
}

fn update_existing_paths(
    pool: Res<AsyncComputeTaskPool>,
    finder: Res<Arc<PathFinder>>,
    mut state: ResMut<UpdatePathsState>,
    mut events: EventReader<PathFinderUpdated>,
    entities: Query<(Entity, &GlobalTransform, &PathResult)>,
) {
    if events.iter().count() == 0 {
        // consume the iterator
        return;
    }

    for (entity, transform, path) in entities.iter() {
        let new_target = PathTarget::new(
            path.target().location(),
            PathQueryProps::new(path.target().properties().distance(), f32::INFINITY),
        );

        state.spawn_new(
            pool.as_ref(),
            finder.clone(),
            entity,
            transform.translation.to_flat(),
            new_target,
        );
    }
}

fn update_requested_paths(
    pool: Res<AsyncComputeTaskPool>,
    finder: Res<Arc<PathFinder>>,
    mut state: ResMut<UpdatePathsState>,
    mut events: EventReader<UpdateEntityPath>,
    entities: Query<&GlobalTransform, With<MovableSolid>>,
) {
    for event in events.iter() {
        if let Ok(transform) = entities.get(event.entity()) {
            state.spawn_new(
                pool.as_ref(),
                finder.clone(),
                event.entity(),
                transform.translation.to_flat(),
                event.target(),
            );
        }
    }
}

fn check_path_results(mut commands: Commands, mut state: ResMut<UpdatePathsState>) {
    for (entity, path) in state.check_results() {
        let mut entity_commands = commands.entity(entity);
        match path {
            Some(path) => {
                entity_commands.insert(path);
            }
            None => {
                entity_commands.remove::<PathResult>();
            }
        }
    }
}
