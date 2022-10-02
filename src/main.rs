use animation::AnimatorPlugin;
use bevy::{
    prelude::*,
    sprite::{Anchor, Rect},
    time::Stopwatch,
    ui::FocusPolicy,
    window::{PresentMode, WindowMode},
};
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use collision::CollisionPlugin;
use components::Background;
use entities::entity_loader::spawn_entity;
use event_system::EventSystemPlugin;
use input_actions::InputAction;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use moveable::MoveablePlugin;
use movement::path_movement::{PathMoveable, PathMovementPlugin};
use player::PlayerPlugin;

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use shoot::ShootPlugin;
use spawners::enemy_wave_spawner::EnemyWaveSpawnerPlugin;
use utils::load_config::ConfigData;

use crate::{
    animation::{spawn_animated_entity, spawn_animated_entity_with_color, AnimationSheet},
    collision::Collider,
    components::{Collectable, Health},
    player::Player,
    shoot::Shootable,
    utils::load_config::load_config,
};

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

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
mod collision;
mod movement;
mod spawners;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    LoadingMainMenu,
    MainMenu,
    LoadingInGame,
    InGame,
    GameOver,
}

#[derive(AssetCollection)]
struct MenuAssets {
    #[asset(path = "fonts/FFFFORWA.ttf")]
    pub font: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct CoreAssets {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 12, rows = 10))]
    #[asset(path = "tiles.png")]
    pub general: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 2, rows = 1))]
    #[asset(path = "player.png")]
    pub plane: Handle<TextureAtlas>,

    // TODO implement this by copying the player sprite and setting it to black
    // #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 2, rows = 1))]
    // #[asset(path = "player_shadow.png")]
    // pub plane_shadow: Handle<TextureAtlas>,
    #[asset(path = "background_0.png")]
    pub background: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 2))]
    #[asset(path = "coin.png")]
    pub collectable: Handle<TextureAtlas>,

    // UI assets
    #[asset(path = "fonts/FFFFORWA.ttf")]
    pub font: Handle<Font>,
}

pub struct Score {
    amount: i64,
}

#[derive(Component)]
pub struct UIScore;

fn main() {
    let config = load_config();

    App::new()
        .insert_resource(config)
        .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelCameraPlugin)
        .insert_resource(WindowDescriptor {
            title: "Planes".to_string(),
            resizable: false,
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::BorderlessFullscreen,
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
        // .add_plugin(EditorPlugin)
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
        .add_plugin(ShootPlugin)
        .add_plugin(MoveablePlugin)
        .add_plugin(PathMovementPlugin) // TODO: this should be in enemy wave spawner?
        // .add_plugin(EnemyPlugin)
        .add_plugin(AnimatorPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(EnemyWaveSpawnerPlugin)
        // ==========================================================
        .insert_resource(Score { amount: 0 })
        // ==========================================================
        .add_enter_system(GameState::MainMenu, setup_main_menu)
        .add_enter_system(GameState::InGame, setup_in_game)
        .add_enter_system(GameState::GameOver, setup_game_over)
        .add_system(move_camera.run_in_state(GameState::InGame))
        .add_system(main_menu_system.run_in_state(GameState::MainMenu))
        .run();
}

fn setup_main_menu(mut commands: Commands, config: Res<ConfigData>, menu_assets: Res<MenuAssets>) {
    println!("setting up main menu...");

    commands
        .spawn_bundle(PixelCameraBundle::from_resolution(
            config.general.screen_height as i32,
            (config.general.screen_height * config.general.base_aspect_ratio) as i32,
        ))
        .insert(UiCameraConfig { show_ui: true });

    // Root
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::GRAY.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Main Menu",
                            TextStyle {
                                font: menu_assets.font.clone(),
                                font_size: 60.,
                                color: Color::WHITE,
                            },
                        )
                        .with_text_alignment(TextAlignment::TOP_CENTER)
                        .with_style(Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::all(Val::Px(10.)),
                            ..default()
                        }),
                    );
                });

            // Buttons
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.), Val::Px(65.)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: menu_assets.font.clone(),
                            font_size: 32.,
                            color: Color::WHITE,
                        },
                    ));
                });
        })
        .insert(UIScore);

    println!("setting up main menu. Main menu NYI, moving straight to core...");
    // TODO Control this from ManiMenu system
    // commands.insert_resource(NextState(GameState::LoadingInGame));
}

fn main_menu_system(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::LoadingInGame));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_in_game(
    mut commands: Commands,
    core_assets: ResMut<CoreAssets>,
    config: Res<ConfigData>,
    score: Res<Score>,
    menu_entities_query: Query<Entity, With<Transform>>,
) {
    println!("unloading menu assets");
    for entity in menu_entities_query.iter() {
        commands.entity(entity).despawn();
    }

    println!("Setting up in-game...");

    // Pixel Camera
    commands
        .spawn_bundle(PixelCameraBundle::from_resolution(
            config.general.screen_height as i32,
            (config.general.screen_height * config.general.base_aspect_ratio) as i32,
        ))
        .insert(UiCameraConfig { show_ui: true });

    // UI
    commands
        .spawn_bundle(
            TextBundle::from_section(
                score.amount.to_string(),
                TextStyle {
                    font: core_assets.font.clone(),
                    font_size: 60.,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                margin: UiRect::all(Val::Px(10.)),
                ..default()
            }),
        )
        .insert(UIScore);

    // let tower = craete_entity_from_atlas(
    //     &mut commands,
    //     &core_asssets.general,
    //     24,
    //     Vec3::new(100., 100., 10.),
    // );
    // commands.entity(tower).insert(Name::new("Tower"));

    // Background
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
        // .push_children(&[collectable])
        ;

    // Player
    let player_entity = spawn_animated_entity(
        &mut commands,
        Vec3::new(0., 0., 100.),
        &AnimationSheet {
            handle: core_assets.plane.clone(),
            frames: vec![0, 1],
        },
        0.2,
        true,
    );

    let player_shadow = spawn_animated_entity_with_color(
        &mut commands,
        Vec3::new(0., 0., 100.),
        &AnimationSheet {
            handle: core_assets.plane.clone(),
            frames: vec![0],
        },
        0.2,
        true,
        Color::rgba(0., 0., 0., 0.5),
    );

    commands
        .entity(player_shadow)
        .insert(Transform::from_xyz(-15., -15., -1.));

    commands
        .entity(player_entity)
        .insert(Name::new(format!("Player_{}", player_entity.id())))
        .insert(Player {
            movement_speed: config.player.movement_speed,
            max_speed: config.player.max_speed,
            movement_direction: Vec2::new(0., 0.),
            target_animation_frame: 0, // Default position
        })
        .insert(Health {
            // TODO: calcuate total value from upgrades
            amount: config.player.base_health,
        })
        .insert(Collider)
        .insert(Shootable {
            direction: Vec3::new(0., 1., 0.),
            source: player_entity,
            shoot_speed_per_ms: 500, // TODO To be calculated from upgrades
            time: Stopwatch::new(),
            is_shooting: false,
        })
        .insert_bundle(InputManagerBundle::<InputAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::Space, InputAction::Shoot),
                (KeyCode::W, InputAction::Move_Up),
                (KeyCode::S, InputAction::Move_Down),
                (KeyCode::A, InputAction::Move_Left),
                (KeyCode::D, InputAction::Move_Right),
            ]),
        })
        .add_child(player_shadow);
}

fn setup_game_over(mut commands: Commands, entities_query: Query<Entity, With<Transform>>) {
    println!("game over");

    for entity in entities_query.iter() {
        commands.entity(entity).despawn();
    }

    commands.insert_resource(NextState(GameState::LoadingMainMenu));
}

fn move_camera(
    mut moveable_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    config: Res<ConfigData>,
) {
    for mut moveable_transform in moveable_query.iter_mut() {
        moveable_transform.translation -=
            Vec3::new(0., -config.general.scroll_speed * time.delta_seconds(), 0.);
    }
}
