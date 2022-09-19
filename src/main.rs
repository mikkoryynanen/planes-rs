use animation::AnimatorPlugin;
use bevy::{prelude::*, sprite::Anchor, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use components::Background;
use enemy::EnemyPlugin;
use entities::entity_loader::spawn_entity;
use event_system::EventSystemPlugin;
use input_actions::InputAction;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use moveable::MoveablePlugin;
use player::PlayerPlugin;

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use shoot::ShootPlugin;
use utils::load_config::load_config;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 180.;
pub const SCROLL_SPEED: f32 = 15.;

// Texture Atlas settings =========================
pub const TILE_SIZE: f32 = 16.;
pub const TILE_PADDING: f32 = 1.;
pub const TILES_PATH: &str = "tiles.png";
pub const PLANES_PATH: &str = "player_plane.png";
// ================================================

// Sprite settings ================================
pub const SPRITE_SCALE: f32 = 1.;
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
mod animation;
mod components;
mod plugins;
mod utils;
// mod asset_collections;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    LoadingMainMenu,
    MainMenu,
    LoadingInGame,
    InGame,
}

#[derive(AssetCollection)]
struct MenuAssets {}

#[derive(AssetCollection)]
struct CoreAssets {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 12, rows = 10))]
    #[asset(path = "tiles.png")]
    pub general: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 3, rows = 1))]
    #[asset(path = "player_plane.png")]
    pub plane: Handle<TextureAtlas>,

    #[asset(path = "background_0.png")]
    pub background: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelCameraPlugin)
        .insert_resource(WindowDescriptor {
            title: "Planes".to_string(),
            resizable: false,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        // Initial state ===========================================
        .add_loopless_state(GameState::LoadingMainMenu)
        .add_loading_state(
            LoadingState::new(GameState::LoadingMainMenu)
                .continue_to_state(GameState::MainMenu)
                // .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                //     "dynamic_assets.assets",
                // ])
                .with_collection::<MenuAssets>(),
        )
        .add_loading_state(
            LoadingState::new(GameState::LoadingInGame)
                .continue_to_state(GameState::InGame)
                // .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                //     "dynamic_assets.assets",
                // ])
                .with_collection::<CoreAssets>(),
        )
        // =========================================================
        // Development =============================================
        .add_plugin(EditorPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // ==========================================================
        // Game plugins =============================================
        .add_plugin(EventSystemPlugin)
        .add_plugin(InputManagerPlugin::<InputAction>::default())
        // ==========================================================
        // Gameplay plugins =========================================
        .add_plugin(PlayerPlugin)
        .add_plugin(MoveablePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ShootPlugin)
        .add_plugin(AnimatorPlugin)
        // ==========================================================
        .add_enter_system(GameState::MainMenu, setup_main_menu)
        .add_enter_system(GameState::InGame, setup_in_game)
        .add_system(move_camera)
        .run();
}

fn setup_main_menu(mut commands: Commands, menu_assets: Res<MenuAssets>) {
    println!("setting up main menu. Main menu NYI, moving straight to core...");
    commands.insert_resource(NextState(GameState::LoadingInGame));
}

fn setup_in_game(mut commands: Commands, core_assets: Res<CoreAssets>) {
    println!("Setting up in-game...");

    // let data = load_config();
    // commands.insert_resource(data);

    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        SCREEN_HEIGHT as i32,
        (SCREEN_HEIGHT * ASPECT_RATIO) as i32,
    ));

    // let tower = craete_entity_from_atlas(
    //     &mut commands,
    //     &core_asssets.general,
    //     24,
    //     Vec3::new(100., 100., 10.),
    // );
    // commands.entity(tower).insert(Name::new("Tower"));

    let background = spawn_entity(
        &mut commands,
        core_assets.background.clone(),
        Vec3::new(0., -50., 0.),
        Anchor::BottomCenter,
    );
    commands
        .entity(background)
        .insert(Name::new("Background"))
        .insert(Background)
        // .push_children(&[tower])
        ;
}

fn move_camera(mut moveable_query: Query<&mut Transform, With<Background>>, time: Res<Time>) {
    for mut moveable_transform in moveable_query.iter_mut() {
        moveable_transform.translation -= Vec3::new(0., SCROLL_SPEED * time.delta_seconds(), 0.);
    }
}
