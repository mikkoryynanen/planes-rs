use bevy::prelude::*;

use crate::{
    shoot::Shootable, sprite_spawner::spawn_entity, ASPECT_RATIO, PLAYER_SPRITE, SCREEN_HEIGHT,
};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(movement);
    }
}

fn setup(mut commands: Commands, asset: Res<AssetServer>) {
    let player_entity = spawn_entity(
        &mut commands,
        &asset,
        PLAYER_SPRITE,
        Vec3::new(0., 0., 100.),
        Vec3::splat(1.),
    );

    commands
        .entity(player_entity)
        .insert(Name::new("Player"))
        .insert(Player { speed: 450. })
        .insert(Shootable);
}

fn movement(
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut player_transform) = player_query.single_mut();

    let mut y_delta = 0.;
    if keyboard.pressed(KeyCode::W) {
        y_delta += 1. * time.delta_seconds() * player.speed;
    }
    if keyboard.pressed(KeyCode::S) {
        y_delta -= 1. * time.delta_seconds() * player.speed;
    }

    let mut x_delta = 0.;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= 1. * time.delta_seconds() * player.speed;
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += 1. * time.delta_seconds() * player.speed;
    }

    player_transform.translation += Vec3::new(x_delta, y_delta, 0.);

    if player_transform.translation.x > SCREEN_HEIGHT * ASPECT_RATIO / 2. {
        player_transform.translation.x = SCREEN_HEIGHT * ASPECT_RATIO / 2.;
    }
    if player_transform.translation.x <= -SCREEN_HEIGHT * ASPECT_RATIO / 2. {
        player_transform.translation.x = -SCREEN_HEIGHT * ASPECT_RATIO / 2.;
    }

    if player_transform.translation.y > SCREEN_HEIGHT / 2. {
        player_transform.translation.y = SCREEN_HEIGHT / 2.;
    }
    if player_transform.translation.y <= -SCREEN_HEIGHT / 2. {
        player_transform.translation.y = -SCREEN_HEIGHT / 2.;
    }
}
