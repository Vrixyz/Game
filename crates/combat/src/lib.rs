pub use attack::AttackEvent;
use attack::AttackPlugin;
use bevy::{
    app::PluginGroupBuilder,
    prelude::{PluginGroup, SystemLabel},
};
use laser::LaserPlugin;

mod attack;
mod laser;
mod sightline;

pub struct CombatPluginGroup;

impl PluginGroup for CombatPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(LaserPlugin).add(AttackPlugin);
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
enum AttackingLabels {
    Update,
    Fire,
}
