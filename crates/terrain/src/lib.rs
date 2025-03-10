mod collider;
mod marker;
mod plugin;
mod shader;
mod terrain;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use collider::TerrainCollider;
pub use marker::CircleMarker;
use marker::MarkerPlugin;
use plugin::TerrainPlugin;
pub use terrain::TerrainBundle;

pub struct TerrainPluginGroup;

impl PluginGroup for TerrainPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(TerrainPlugin).add(MarkerPlugin);
    }
}
