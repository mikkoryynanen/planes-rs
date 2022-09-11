use bevy::prelude::*;

use crate::SPRITE_SCALE;

pub fn spawn_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    asset_path: &str,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    return commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SPRITE_SCALE)),
                ..Default::default()
            },
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
