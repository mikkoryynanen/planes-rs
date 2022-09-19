use crate::{
    enemy::EnemyPlugin, moveable::MoveablePlugin, player::PlayerPlugin, shoot::ShootPlugin,
};
use bevy::{app::PluginGroupBuilder, prelude::*};
pub struct CorePlugins;
impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PlayerPlugin)
            .add(MoveablePlugin)
            .add(EnemyPlugin)
            .add(ShootPlugin);
    }
}
