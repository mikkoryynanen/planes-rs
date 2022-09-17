use bevy::{prelude::*, sprite::Anchor};

use crate::{
    entities::entity_loader::{spawn_entity, GameSheets},
    player::Player,
};

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(loop_animated_frames)
            .add_system(animate_player);
    }
}

#[derive(Component)]
pub struct FrameAnimation {
    timer: Timer,
    frames: Vec<usize>,
    current_frame: usize,
}

pub struct AnimationSheet {
    pub handle: Handle<TextureAtlas>,
    pub frames: [usize; 3],
}

pub fn spawn_animated_entity(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    translation: Vec3,
    animation_sheet: &AnimationSheet,
) -> Entity {
    let animated_sprite = TextureAtlasSprite::new(animation_sheet.frames[0]);

    return commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: animated_sprite,
            texture_atlas: animation_sheet.handle.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FrameAnimation {
            timer: Timer::from_seconds(0.2, true),
            frames: animation_sheet.frames.to_vec(),
            current_frame: 0,
        })
        .id();
}

fn loop_animated_frames(
    mut sprites_query: Query<(&mut TextureAtlasSprite, &mut FrameAnimation)>,
    time: Res<Time>,
) {
    //  for (mut sprite, mut animation) in sprites_query.iter_mut() {
    //      animation.timer.tick(time.delta());
    //      if animation.timer.just_finished() {
    //          animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
    //          sprite.index = animation.frames[animation.current_frame];
    //      }
    //  }
}

// TODO Maybe make this usable to other animated entities
fn animate_player(
    mut sprites_query: Query<(&mut TextureAtlasSprite, &mut FrameAnimation, &Player)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation, player) in sprites_query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if animation.current_frame != player.target_animation_frame {
                if player.target_animation_frame > animation.current_frame {
                    animation.current_frame += 1;
                } else {
                    animation.current_frame -= 1;
                }
                println!("animation.current_frame {}", animation.current_frame);
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}
