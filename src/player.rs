use bevy::{prelude::*, time::Stopwatch};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use crate::{
    entities::entity_loader::{craete_entity_from_atlas, GameSheets},
    input_actions::InputAction,
    shoot::Shootable,
    ASPECT_RATIO, SCREEN_HEIGHT, SPRITE_SCALE,
};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(movement)
            .add_system(shooting_system);
    }
}

fn setup(mut commands: Commands, sheet: Res<GameSheets>) {
    let player_entity = craete_entity_from_atlas(
        &mut commands,
        &sheet.planes,
        2,
        Vec3::new(0., 0., 100.),
        SPRITE_SCALE,
    );

    commands
        .entity(player_entity)
        .insert(Name::new("Player"))
        .insert(Player { speed: 450. })
        .insert(Shootable {
            direction: Vec3::new(0., 1., 0.),
            shoot_speed_per_ms: 500,
            time: Stopwatch::new(),
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<InputAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::Space, InputAction::Shoot)]),
        });
}

fn movement(
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut player_transform) = player_query.single_mut();

    let mut y_delta = 0.;
    if keyboard.pressed(KeyCode::W) {
        y_delta += 1. * time.delta_seconds() * player.speed;
    }
    if keyboard.pressed(KeyCode::S) {
        y_delta -= 1. * time.delta_seconds() * player.speed;
    }

    let mut x_delta = 0.;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= 1. * time.delta_seconds() * player.speed;
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += 1. * time.delta_seconds() * player.speed;
    }

    player_transform.translation += Vec3::new(x_delta, y_delta, 0.);

    if player_transform.translation.x > SCREEN_HEIGHT * ASPECT_RATIO / 2. {
        player_transform.translation.x = SCREEN_HEIGHT * ASPECT_RATIO / 2.;
    }
    if player_transform.translation.x <= -SCREEN_HEIGHT * ASPECT_RATIO / 2. {
        player_transform.translation.x = -SCREEN_HEIGHT * ASPECT_RATIO / 2.;
    }

    if player_transform.translation.y > SCREEN_HEIGHT / 2. {
        player_transform.translation.y = SCREEN_HEIGHT / 2.;
    }
    if player_transform.translation.y <= -SCREEN_HEIGHT / 2. {
        player_transform.translation.y = -SCREEN_HEIGHT / 2.;
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
