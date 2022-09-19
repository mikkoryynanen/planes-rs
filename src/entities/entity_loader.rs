use bevy::{prelude::*, sprite::Anchor};

pub fn spawn_entity(
    commands: &mut Commands,
    texture: Handle<Image>,
    translation: Vec3,
    anchor_override: Anchor,
) -> Entity {
    return commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: anchor_override,
                ..Default::default()
            },
            texture: texture,
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
