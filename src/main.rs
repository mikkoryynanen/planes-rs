use bevy::{prelude::*, window::PresentMode};
use bevy_editor_pls::EditorPlugin;
use bevy_retro_camera::{RetroCameraBundle, RetroCameraPlugin};
use enemy::EnemyPlugin;
use entities::entity_loader::{spawn_entity, TilemapPlugin};
use event_system::EventSystemPlugin;
use input_actions::InputAction;
use leafwing_input_manager::prelude::*;
use moveable::MoveablePlugin;
use player::PlayerPlugin;

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use shoot::ShootPlugin;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 180.;

// Texture Atlas settings =========================
pub const TILE_SIZE: f32 = 16.;
pub const TILE_PADDING: f32 = 1.;
pub const TILES_PATH: &str = "tiles.png";
pub const PLANES_PATH: &str = "Sheets/Ships/Variation 1/JetFighter-Var1-Spritesheet.png";
// ================================================

// Sprite settings ================================
pub const SPRITE_SCALE: f32 = 1.;
const BACKGROUND_SPRITE: &str = "background_0.png";
// ================================================

mod enemy;
mod entities;
mod event_system;
mod input_actions;
mod moveable;
mod player;
mod projectile;
mod shoot;
// TODO Generic file for components, maybe replace this in the future
mod components;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Planes".to_string(),
            resizable: false,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_plugin(RetroCameraPlugin)
        .add_plugins(DefaultPlugins)
        // Development =============================================
        .add_plugin(EditorPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // ==========================================================
        // Game plugins =============================================
        .add_plugin(TilemapPlugin)
        .add_plugin(EventSystemPlugin)
        .add_plugin(InputManagerPlugin::<InputAction>::default())
        // ==========================================================
        // Gameplay plugins =========================================
        .add_plugin(PlayerPlugin)
        .add_plugin(MoveablePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ShootPlugin)
        // ==========================================================
        .add_startup_system(setup)
        .add_system(move_camera)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scale: f32 = 2.; // Viewport scaling factor
    commands.spawn_bundle(RetroCameraBundle::fixed_height(SCREEN_HEIGHT, scale));

    let background = spawn_entity(
        &mut commands,
        &asset_server,
        BACKGROUND_SPRITE,
        Vec3::new(0., 7108.0, 0.),
    );
    commands.entity(background).insert(Name::new("Background"));
}

fn move_camera(mut moveable_query: Query<&mut Transform, With<Camera2d>>, time: Res<Time>) {
    for mut moveable_transform in moveable_query.iter_mut() {
        moveable_transform.translation += Vec3::new(0., 5. * time.delta_seconds(), -1.);
    }
}
