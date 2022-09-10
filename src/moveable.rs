use bevy::prelude::*;

use crate::SCREEN_HEIGHT;

#[derive(Component)]
pub struct Moveable {
    pub direction: Vec2,
    pub speed: f32,
    pub auto_destroy: bool,
}

pub struct MoveablePlugin;

impl Plugin for MoveablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_moveables);
    }
}

fn update_moveables(
    mut commands: Commands,
    mut moveable_query: Query<(Entity, &mut Transform, &Moveable), With<Moveable>>,
    time: Res<Time>,
) {
    for (entity, mut moveable_transform, moveable) in moveable_query.iter_mut() {
        moveable_transform.translation += Vec3::new(
            moveable.direction.x * moveable.speed * time.delta_seconds(),
            moveable.direction.y * moveable.speed * time.delta_seconds(),
            0.,
        );

        if moveable.auto_destroy {
            if moveable_transform.translation.y > SCREEN_HEIGHT
               //  || moveable_transform.translation.y < 0.
            {
                commands.entity(entity).despawn();
                println!("moveale destroyed")
            }
        }
    }
}
