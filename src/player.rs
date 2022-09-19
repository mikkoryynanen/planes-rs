use bevy::{prelude::*, time::Stopwatch};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use crate::{
    animation::{spawn_animated_entity, AnimationSheet},
    components::Health,
    input_actions::InputAction,
    shoot::Shootable,
    CoreAssets, GameState, ASPECT_RATIO, SCREEN_HEIGHT,
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
        )
        .add_enter_system(GameState::InGame, setup);
    }
}

fn setup(
    mut commands: Commands,
    core_asssets: Res<CoreAssets>,
    // config: Res<ConfigData>
) {
    let player_entity = spawn_animated_entity(
        &mut commands,
        Vec3::new(0., 0., 100.),
        &AnimationSheet {
            handle: core_asssets.plane.clone(),
            frames: vec![0, 1, 2],
        },
        0.2,
        true,
    );

    commands
        .entity(player_entity)
        .insert(Name::new(format!("Player_{}", player_entity.id())))
        .insert(Player {
            movement_speed: 450.,
            max_speed: 500.,
            movement_direction: Vec2::new(0., 0.),
            target_animation_frame: 1, // Default position
        })
        .insert(Health { amount: 100 })
        // .insert(Collider) // TODO Enable once game states are done
        .insert(Shootable {
            direction: Vec3::new(0., 1., 0.),
            source: player_entity,
            shoot_speed_per_ms: 500,
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
        });
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
        player.target_animation_frame = 0;
    } else if action_state.pressed(InputAction::Move_Right) {
        player.movement_direction.x += player.movement_speed * time.delta_seconds();
        player.target_animation_frame = 2;
    } else if player.movement_direction != Vec2::ZERO {
        player.target_animation_frame = 1;
        // No input but still moving
        let delta = player.movement_speed * time.delta_seconds();
        if player.movement_direction.x < 0. {
            player.movement_direction.x = (player.movement_direction.x + delta).min(0.);
        } else {
            player.movement_direction.x = (player.movement_direction.x - delta).max(0.);
        }
    };

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

    if player_transform.translation.x > SCREEN_HEIGHT * ASPECT_RATIO {
        player_transform.translation.x = SCREEN_HEIGHT * ASPECT_RATIO;
    }
    if player_transform.translation.x <= -SCREEN_HEIGHT * ASPECT_RATIO {
        player_transform.translation.x = -SCREEN_HEIGHT * ASPECT_RATIO;
    }

    if player_transform.translation.y > SCREEN_HEIGHT {
        player_transform.translation.y = SCREEN_HEIGHT;
    }
    if player_transform.translation.y <= -SCREEN_HEIGHT {
        player_transform.translation.y = -SCREEN_HEIGHT;
    }
}

fn shooting_system(
    mut shooter_query: Query<&mut Shootable, (With<Shootable>, With<Player>)>,
    action_query: Query<&ActionState<InputAction>, With<Player>>,
) {
    let action_state = action_query.single();
    let mut shootable = shooter_query.single_mut();

    shootable.is_shooting = action_state.pressed(InputAction::Shoot);
}
