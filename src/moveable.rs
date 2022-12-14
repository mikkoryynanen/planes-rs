use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::{utils::load_config::ConfigData, GameState};
#[derive(Component)]
pub struct Moveable {
    pub direction: Vec3,
    pub speed: f32,
    pub auto_destroy: bool,
}

pub struct MoveablePlugin;

impl Plugin for MoveablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(update_moveables)
                .into(),
        );
    }
}

fn update_moveables(
    mut commands: Commands,
    mut moveable_query: Query<(Entity, &mut Transform, &Moveable), With<Moveable>>,
    time: Res<Time>,
    config: Res<ConfigData>,
) {
    for (entity, mut moveable_transform, moveable) in moveable_query.iter_mut() {
        moveable_transform.translation += Vec3::new(
            moveable.direction.x * moveable.speed * time.delta_seconds(),
            moveable.direction.y * moveable.speed * time.delta_seconds(),
            0.,
        );

        let angle = (moveable.direction - moveable_transform.translation)
            .angle_between(moveable_transform.translation);
        moveable_transform.rotation = Quat::from_rotation_z(angle);

        if moveable.auto_destroy {
            if moveable_transform.translation.y > config.general.screen_height
            //  || moveable_transform.translation.y < 0.
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
