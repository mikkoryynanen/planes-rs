use bevy::{prelude::*, time::Stopwatch};
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    collision::Collider, components::Health, entities::entity_loader::craete_entity_from_atlas,
    moveable::Moveable, shoot::Shootable, CoreAssets, GameState,
};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, setup);
    }
}

fn setup(mut commands: Commands, core_asssets: Res<CoreAssets>) {
    let enemy_entity = craete_entity_from_atlas(
        &mut commands,
        &core_asssets.plane,
        0,
        Vec3::new(0., 50., 100.),
    );

    commands
        .entity(enemy_entity)
        .insert(Name::new(format!("Enemy_{}", enemy_entity.id())))
        .insert(Enemy)
        .insert(Health { amount: 100 })
        .insert(Collider)
        .insert(Moveable {
            auto_destroy: false,
            direction: Vec3::new(0., -1., 0.),
            speed: 0.,
        })
        .insert(Shootable {
            direction: Vec3::new(0., -1., 0.),
            source: enemy_entity,
            shoot_speed_per_ms: 500,
            time: Stopwatch::new(),
            is_shooting: true,
        });
}
