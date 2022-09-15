use bevy::{prelude::*, sprite::Anchor, window::PresentMode};
use bevy_editor_pls::EditorPlugin;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use components::Background;
use enemy::EnemyPlugin;
use entities::entity_loader::{craete_entity_from_atlas, spawn_entity, GameSheets, TilemapPlugin};
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
pub const SCROLL_SPEED: f32 = 15.;

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
        .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelCameraPlugin)
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, sheets: Res<GameSheets>) {
    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        SCREEN_HEIGHT as i32,
        (SCREEN_HEIGHT * ASPECT_RATIO) as i32,
    ));

    let tower = craete_entity_from_atlas(
        &mut commands,
        &sheets.general,
        24,
        Vec3::new(100., 100., 10.),
    );
    commands.entity(tower).insert(Name::new("Tower"));

    let background = spawn_entity(
        &mut commands,
        &asset_server,
        BACKGROUND_SPRITE,
        Vec3::new(0., -50., 0.),
        Anchor::BottomCenter,
    );
    commands
        .entity(background)
        .insert(Name::new("Background"))
        .insert(Background)
        .push_children(&[tower]);
}

fn move_camera(mut moveable_query: Query<&mut Transform, With<Background>>, time: Res<Time>) {
    for mut moveable_transform in moveable_query.iter_mut() {
        moveable_transform.translation -= Vec3::new(0., SCROLL_SPEED * time.delta_seconds(), 0.);
    }
}
