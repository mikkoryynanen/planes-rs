use bevy::{prelude::*, window::PresentMode, render::camera::ScalingMode};

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 9. / 16.;
pub const TILE_SIZE: f32 = 0.1;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

fn main() {
    let height = 800.;

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Planes".to_string(),
            height: height,
            width: height * ASPECT_RATIO,
            resizable: false,
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_plugins(DefaultPlugins)
        .run();
}

fn player_movement(
    mut player_query: Query<(&Player,&mut Transform), With<Player>>, 
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
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
}

fn setup(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(50.)),
            ..Default::default()
        },
        texture: asset.load("ship.png"),
        transform: Transform::from_xyz(0., 0., 100.),
        ..Default::default()
    })
    .insert(Player {
        speed: 250.
    });
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // camera.projection.scaling_mode = ScalingMode::None;
    camera.projection.top = 1.;
    camera.projection.bottom = -1.;
    camera.projection.left = 1. * ASPECT_RATIO;
    camera.projection.right = -1. * ASPECT_RATIO;

    commands.spawn_bundle(camera);
}
