use bevy::prelude::*;

#[derive(Component)]
pub struct Moveable {
    pub direction: Vec2,
    pub speed: f32,
}

pub struct MoveablePlugin;

impl Plugin for MoveablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_moveables);
    }
}

fn update_moveables(
    mut moveable_query: Query<(&mut Transform, &Moveable), With<Moveable>>,
    time: Res<Time>,
) {
    for (mut moveable_transform, moveable) in moveable_query.iter_mut() {
        moveable_transform.translation += Vec3::new(
            moveable.direction.x * moveable.speed * time.delta_seconds(),
            moveable.direction.y * moveable.speed * time.delta_seconds(),
            0.,
        );
    }
}
