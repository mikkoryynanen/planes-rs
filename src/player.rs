use bevy::{prelude::*, time::Stopwatch};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use crate::{
    animation::{spawn_animated_entity, spawn_animated_entity_with_color, AnimationSheet},
    collision::Collider,
    components::Health,
    input_actions::InputAction,
    shoot::Shootable,
    utils::load_config::ConfigData,
    CoreAssets, GameState,
};

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub max_speed: f32,
    pub target_animation_frame: usize,
    // TODO sould be private
    pub movement_direction: Vec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(movement)
                .with_system(shooting_system)
                .into(),
        );
    }
}

fn movement(
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    action_query: Query<&ActionState<InputAction>, With<Player>>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    let action_state = action_query.single();

    if action_state.pressed(InputAction::Move_Up) {
        player.movement_direction.y += player.movement_speed * time.delta_seconds();
    } else if action_state.pressed(InputAction::Move_Down) {
        player.movement_direction.y -= player.movement_speed * time.delta_seconds();
    } else if player.movement_direction != Vec2::ZERO {
        // No input but still moving
        let delta = player.movement_speed * time.delta_seconds();
        if player.movement_direction.y < 0. {
            player.movement_direction.y = (player.movement_direction.y + delta).min(0.);
        } else {
            player.movement_direction.y = (player.movement_direction.y - delta).max(0.);
        }
    };

    if action_state.pressed(InputAction::Move_Left) {
        player.movement_direction.x -= player.movement_speed * time.delta_seconds();
        // player.target_animation_frame = 0;
    } else if action_state.pressed(InputAction::Move_Right) {
        player.movement_direction.x += player.movement_speed * time.delta_seconds();
        // player.target_animation_frame = 2;
    } else if player.movement_direction != Vec2::ZERO {
        // player.target_animation_frame = 1;
        // No input but still moving
        let delta = player.movement_speed * time.delta_seconds();
        if player.movement_direction.x < 0. {
            player.movement_direction.x = (player.movement_direction.x + delta).min(0.);
        } else {
            player.movement_direction.x = (player.movement_direction.x - delta).max(0.);
        }
    };

    // Apply movment
    player.movement_direction.x = player
        .movement_direction
        .x
        .clamp(-player.max_speed, player.max_speed);
    player.movement_direction.y = player
        .movement_direction
        .y
        .clamp(-player.max_speed, player.max_speed);
    player_transform.translation +=
        Vec3::new(player.movement_direction.x, player.movement_direction.y, 0.)
            * time.delta_seconds();

    // Clamp movement to screen
    // TODO: Enable
    // if player_transform.translation.x
    //     > config.general.screen_height * config.general.base_aspect_ratio
    // {
    //     player_transform.translation.x =
    //         config.general.screen_height * config.general.base_aspect_ratio;
    // }
    // if player_transform.translation.x
    //     <= -config.general.screen_height * config.general.base_aspect_ratio
    // {
    //     player_transform.translation.x =
    //         -config.general.screen_height * config.general.base_aspect_ratio;
    // }

    // if player_transform.translation.y > config.general.screen_height {
    //     player_transform.translation.y = config.general.screen_height;
    // }
    // if player_transform.translation.y <= -config.general.screen_height {
    //     player_transform.translation.y = -config.general.screen_height;
    // }
}

fn shooting_system(
    mut shooter_query: Query<&mut Shootable, (With<Shootable>, With<Player>)>,
    action_query: Query<&ActionState<InputAction>, With<Player>>,
) {
    let action_state = action_query.single();
    let mut shootable = shooter_query.single_mut();

    shootable.is_shooting = action_state.pressed(InputAction::Shoot);
}
