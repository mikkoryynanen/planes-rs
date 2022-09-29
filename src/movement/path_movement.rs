use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::GameState;

#[derive(Component)]
pub struct PathMoveable {
    pub next_path_index: usize,
}

pub struct PathMovementPlugin;

impl Plugin for PathMovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(movement)
                .into(),
        );
    }
}

fn movement(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut PathMoveable, Entity), With<PathMoveable>>,
    time: Res<Time>,
) {
    // TODO: Move these to correct places
    let array: [(f32, f32); 3] = [(-100., 0.), (100., 100.), (100., -100.)];
    let movement_speed = 3.;

    for (mut transform, mut path_moveable, entity) in query.iter_mut() {
        // Movement
        let target = Vec3::new(
            array[path_moveable.next_path_index].0,
            array[path_moveable.next_path_index].1,
            0.,
        );
        let difference = target - transform.translation;
        let dot = Vec3::dot(difference, difference);
        if dot < 1. + movement_speed && path_moveable.next_path_index < array.len() {
            path_moveable.next_path_index += 1;
        } else {
            transform.translation += difference.normalize() * movement_speed;
        }

        if path_moveable.next_path_index >= array.len() {
            commands.entity(entity).despawn();
        }

        // Rotation
        let angle = (target - transform.translation).angle_between(transform.translation);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}
