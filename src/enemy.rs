use bevy::prelude::*;

use crate::{moveable::Moveable, shoot::Shootable, sprite_spawner::spawn_entity, ENEMY_SPRITE};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = spawn_entity(
        &mut commands,
        &asset_server,
        ENEMY_SPRITE,
        Vec3::new(0., 250., 0.),
        Vec3::splat(1.),
    );

    commands
        .entity(entity)
        .insert(Name::new("Enemy"))
        .insert(Enemy)
        .insert(Moveable {
            auto_destroy: false,
            direction: Vec3::new(0., -1., 0.),
            speed: 50.,
        })
        .insert(Shootable {
            direction: Vec3::new(0., -1., 0.),
        });
}
