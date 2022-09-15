use bevy::{prelude::*, time::Stopwatch};

use crate::{
    components::{Collider, Health},
    entities::entity_loader::{craete_entity_from_atlas, GameSheets},
    moveable::Moveable,
    shoot::Shootable,
};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, sheets: Res<GameSheets>) {
    let enemy_entity =
        craete_entity_from_atlas(&mut commands, &sheets.planes, 0, Vec3::new(0., 50., 100.));

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
