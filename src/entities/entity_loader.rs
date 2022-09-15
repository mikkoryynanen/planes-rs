use bevy::{prelude::*, sprite::Anchor};

use crate::{PLANES_PATH, TILES_PATH, TILE_PADDING, TILE_SIZE};

pub struct TilemapPlugin;

pub struct GameSheets {
    pub general: Handle<TextureAtlas>,
    pub planes: Handle<TextureAtlas>,
}

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
    anchor_override: Anchor,
) -> Entity {
    return commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: anchor_override,
                ..Default::default()
            },
            texture: asset_server.load(asset_path),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
}

pub fn craete_entity_from_atlas(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>,
    index: usize,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = Color::WHITE;
    // sprite.anchor = Anchor::BottomCenter;

    return commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: atlas.clone(),
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
    let tiles_atlas = generate_atlas(&asset_server, TILES_PATH, 10, 12, TILE_SIZE);
    let tiles_atlas_bundle = texture_atlases.add(tiles_atlas);

    let planes_atlas = generate_atlas(&asset_server, PLANES_PATH, 6, 4, 32.);
    let planes_atlas_bundle = texture_atlases.add(planes_atlas);

    let atlases = GameSheets {
        general: tiles_atlas_bundle,
        planes: planes_atlas_bundle,
    };
    commands.insert_resource(atlases);
}

fn generate_atlas(
    asset_server: &Res<AssetServer>,
    path: &str,
    columns: usize,
    rows: usize,
    tile_size: f32,
) -> TextureAtlas {
    return TextureAtlas::from_grid_with_padding(
        asset_server.load(path),
        Vec2::splat(tile_size),
        columns,
        rows,
        Vec2::splat(TILE_PADDING),
        Vec2::ZERO,
    );
}
