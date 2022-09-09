use bevy::{prelude::*, window::PresentMode};
use moveable::MoveablePlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 600.;

const PLAYER_SPRITE: &str = "ship.png";
const PROJECTILE_SPRITE: &str = "projectile.png";

mod moveable;
mod player;
mod projectile;

fn main() {
    let height = SCREEN_HEIGHT;
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Planes".to_string(),
            height: height,
            width: height * ASPECT_RATIO,
            resizable: false,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(MoveablePlugin)
        .run();
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
