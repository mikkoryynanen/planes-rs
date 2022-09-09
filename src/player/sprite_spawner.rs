use bevy::prelude::*;

pub fn spawn_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entity_name: &str,
    translation: Vec3,
    scale: Vec3
) -> Entity {
    return commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.)),
                ..Default::default()
            },
            texture: asset_server.load(entity_name),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
}
