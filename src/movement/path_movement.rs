use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::{spawners::enemy_wave_spawner::WaveData, GameState};

#[derive(Component)]
pub struct PathMoveable {
    pub next_path_index: usize,
    pub move_positions: Vec<Vec2>,
    pub movement_speed: f32,
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
    mut path_moveable_query: Query<(&mut Transform, &mut PathMoveable, Entity), With<PathMoveable>>,
) {
    for (mut transform, mut path_moveable, entity) in path_moveable_query.iter_mut() {
        // Movement
        let target = path_moveable.move_positions[path_moveable.next_path_index].extend(100.);
        let difference = target - transform.translation;
        let dot = Vec3::dot(difference, difference);
        if dot < 1. + path_moveable.movement_speed
            && path_moveable.next_path_index < path_moveable.move_positions.len()
        {
            path_moveable.next_path_index += 1;
        } else {
            transform.translation += difference.normalize() * path_moveable.movement_speed;
        }

        if path_moveable.next_path_index >= path_moveable.move_positions.len() {
            commands.entity(entity).despawn();
        }

        // Rotation
        let angle = (target - transform.translation).angle_between(transform.translation);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}
