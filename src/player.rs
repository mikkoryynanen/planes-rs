use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    moveable::Moveable, projectile::Projectile, ASPECT_RATIO, PROJECTILE_SPRITE, SCREEN_HEIGHT,
};

use self::sprite_spawner::spawn_entity;

mod sprite_spawner;

const TIMESTEP_1_PER_SECOND: f64 = 30.0 / 60.0;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(movement)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                    .with_system(shooting_system),
            );
    }
}

fn setup(mut commands: Commands, asset: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.)),
                ..Default::default()
            },
            texture: asset.load("ship.png"),
            transform: Transform::from_xyz(0., 0., 100.),
            ..Default::default()
        })
        .insert(Player { speed: 450. });
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

fn shooting_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let player_transform = player_query.single();
    let projectile = spawn_entity(
        &mut commands,
        &asset_server,
        PROJECTILE_SPRITE,
        Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y,
            100.,
        ),
        Vec3::splat(0.35),
    );

    commands
        .entity(projectile)
        .insert(Projectile)
        .insert(Moveable {
            direction: Vec2::new(0., 1.),
            speed: 250.,
            auto_destroy: true
        });
}
