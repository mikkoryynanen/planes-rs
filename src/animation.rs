use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::{player::Player, GameState};

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(loop_animated_frames)
                .with_system(animate_player)
                .into(),
        );
    }
}

#[derive(Component)]
pub struct FrameAnimation {
    timer: Timer,
    frames: Vec<usize>,
    current_frame: usize,
    is_looping: bool,
}

pub struct AnimationSheet {
    pub handle: Handle<TextureAtlas>,
    pub frames: Vec<usize>,
}

pub fn spawn_animated_entity_with_color(
    commands: &mut Commands,
    translation: Vec3,
    animation_sheet: &AnimationSheet,
    frame_duration: f32,
    is_looping: bool,
    color: Color,
) -> Entity {
    return build_animated_entity(
        commands,
        translation,
        animation_sheet,
        frame_duration,
        is_looping,
        color,
    );
}
pub fn spawn_animated_entity(
    commands: &mut Commands,
    translation: Vec3,
    animation_sheet: &AnimationSheet,
    frame_duration: f32,
    is_looping: bool,
) -> Entity {
    return build_animated_entity(
        commands,
        translation,
        animation_sheet,
        frame_duration,
        is_looping,
        Color::WHITE,
    );
}

fn build_animated_entity(
    commands: &mut Commands,
    translation: Vec3,
    animation_sheet: &AnimationSheet,
    frame_duration: f32,
    is_looping: bool,
    color: Color,
) -> Entity {
    let mut animated_sprite = TextureAtlasSprite::new(animation_sheet.frames[0]);
    animated_sprite.color = color;

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
            timer: Timer::from_seconds(frame_duration, is_looping),
            frames: animation_sheet.frames.clone(),
            current_frame: 0,
            is_looping: is_looping,
        })
        .id();
}

fn loop_animated_frames(
    mut commands: Commands,
    mut sprites_query: Query<
        (Entity, &mut TextureAtlasSprite, &mut FrameAnimation),
        Without<Player>,
    >,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut animation) in sprites_query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            sprite.index = animation.frames[animation.current_frame];

            if !animation.is_looping && sprite.index == animation.frames[animation.frames.len() - 1]
            {
                commands.entity(entity).despawn();
            }

            animation.timer.reset();
        }
    }
}

// TODO Maybe make this usable to other animated entities
fn animate_player(
    mut sprites_query: Query<(&mut TextureAtlasSprite, &mut FrameAnimation, &Player), With<Player>>,
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
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}
