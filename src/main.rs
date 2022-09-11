use bevy::{prelude::*, window::PresentMode};
use bevy_editor_pls::EditorPlugin;
use enemy::EnemyPlugin;
use entities::entity_loader::{spawn_entity, TilemapPlugin};
use moveable::{Moveable, MoveablePlugin};
use player::PlayerPlugin;
use projectile::ProjectilePlugin;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use shoot::ShootPlugin;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 600.;

pub const TILE_SIZE: f32 = 16.;
pub const TILE_PADDING: f32 = 1.;
pub const TILES_PATH: &str = "tiles.png";

pub const SPRITE_SCALE: f32 = 50.;
const PLAYER_SPRITE: &str = "ship.png";
const PROJECTILE_SPRITE: &str = "projectile.png";
const ENEMY_SPRITE: &str = "enemy.png";
const BACKGROUND_SPRITE: &str = "background.png";

mod enemy;
mod entities;
mod moveable;
mod player;
mod projectile;
mod shoot;

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
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // ==========================================================
        // Game plugins =============================================
        .add_plugin(TilemapPlugin)
        // ==========================================================
        // Gameplay plugins =========================================
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(MoveablePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ShootPlugin)
        // ==========================================================
        .add_system(move_camera)
        .run();
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    // camera.projection.scaling_mode = ScalingMode::None;
    camera.projection.top = 1.;
    camera.projection.bottom = -1.;
    camera.projection.left = 1. * ASPECT_RATIO;
    camera.projection.right = -1. * ASPECT_RATIO;

    commands.spawn_bundle(camera);

    let background = spawn_entity(
        &mut commands,
        &asset_server,
        BACKGROUND_SPRITE,
        Vec3::splat(0.),
        Vec3::splat(1.),
    );
    commands.entity(background);
}

fn move_camera(mut moveable_query: Query<&mut Transform, With<Camera2d>>, time: Res<Time>) {
    for mut moveable_transform in moveable_query.iter_mut() {
        moveable_transform.translation += Vec3::new(0., 5. * time.delta_seconds(), 0.);
    }
}
