use bevy::prelude::*;

use crate::{TILES_PATH, TILE_PADDING, TILE_SIZE};

pub struct TilemapPlugin;
pub struct GameSheet(Handle<TextureAtlas>);

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_atlas);
    }
}

pub fn spawn_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    asset_path: &str,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    return commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(asset_path),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
}

pub fn craete_entity_from_atlas(
    commands: &mut Commands,
    sheet: &GameSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = Color::WHITE;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    return commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: sheet.0.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
}

fn load_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = asset_server.load(TILES_PATH);
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(TILE_SIZE),
        10,
        12,
        Vec2::splat(TILE_PADDING),
        Vec2::ZERO,
    );

    let atlas_bundle = texture_atlases.add(atlas);
    commands.insert_resource(GameSheet(atlas_bundle));
}
