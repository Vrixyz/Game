//! This crate implements various entity behavior systems.

use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use chase::ChasePlugin;
pub use chase::ChaseTarget;

mod chase;

pub struct BehaviourPluginGroup;

impl PluginGroup for BehaviourPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(ChasePlugin);
    }
}
