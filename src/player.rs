use bevy::{math::vec3, prelude::*, time::Stopwatch};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use crate::{
    components::{Collider, Health},
    entities::entity_loader::{craete_entity_from_atlas, GameSheets},
    input_actions::InputAction,
    shoot::Shootable,
    ASPECT_RATIO, SCREEN_HEIGHT, SPRITE_SCALE,
};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    // TODO sould be private
    pub movement_direction: Vec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(movement)
            .add_system(shooting_system);
    }
}

fn setup(mut commands: Commands, sheets: Res<GameSheets>) {
    let player_entity =
        craete_entity_from_atlas(&mut commands, &sheets.planes, 0, Vec3::new(0., 0., 100.));

    commands
        .entity(player_entity)
        .insert(Name::new(format!("Player_{}", player_entity.id())))
        .insert(Player {
            speed: 450.,
            movement_direction: Vec2::new(0., 0.),
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
                (KeyCode::D, InputAction::Move_Left),
                (KeyCode::A, InputAction::Move_Right),
            ]),
        });
}

fn movement(
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    action_query: Query<&ActionState<InputAction>, With<Player>>,
    time: Res<Time>,
) {
    // TODO Make editable from settings file
    const MAX_SPEED: f32 = 500.;
    const ACCELERATION: f32 = 1700.;

    let (mut player, mut player_transform) = player_query.single_mut();
    let action_state = action_query.single();

    if action_state.pressed(InputAction::Move_Up) {
        player.movement_direction.y += ACCELERATION * time.delta_seconds();
    } else if action_state.pressed(InputAction::Move_Down) {
        player.movement_direction.y -= ACCELERATION * time.delta_seconds();
    } else if player.movement_direction != Vec2::ZERO {
        // No input but still moving
        let delta = ACCELERATION * time.delta_seconds();
        if player.movement_direction.y < 0. {
            player.movement_direction.y = (player.movement_direction.y + delta).min(0.);
        } else {
            player.movement_direction.y = (player.movement_direction.y - delta).max(0.);
        }
    };

    if action_state.pressed(InputAction::Move_Left) {
        player.movement_direction.x += ACCELERATION * time.delta_seconds();
    } else if action_state.pressed(InputAction::Move_Right) {
        player.movement_direction.x -= ACCELERATION * time.delta_seconds();
    } else if player.movement_direction != Vec2::ZERO {
        // No input but still moving
        let delta = ACCELERATION * time.delta_seconds();
        if player.movement_direction.x < 0. {
            player.movement_direction.x = (player.movement_direction.x + delta).min(0.);
        } else {
            player.movement_direction.x = (player.movement_direction.x - delta).max(0.);
        }
    };

    player.movement_direction.x = player.movement_direction.x.clamp(-MAX_SPEED, MAX_SPEED);
    player.movement_direction.y = player.movement_direction.y.clamp(-MAX_SPEED, MAX_SPEED);
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
