use bevy::{prelude::*, window::PresentMode};
use bevy_editor_pls::EditorPlugin;
use enemy::EnemyPlugin;
use moveable::MoveablePlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use shoot::ShootPlugin;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 600.;

pub const SPRITE_SCALE: f32 = 50.;
const PLAYER_SPRITE: &str = "ship.png";
const PROJECTILE_SPRITE: &str = "projectile.png";
const ENEMY_SPRITE: &str = "enemy.png";

mod enemy;
mod moveable;
mod player;
mod projectile;
mod shoot;
mod sprite_spawner;

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
        // Development =============================================
        .add_plugin(EditorPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // ==========================================================
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(MoveablePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ShootPlugin)
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
