use bevy::prelude::*;

use crate::components::Health;

// TODO Maybe move these to their own file
pub struct DamageEvent {
    pub damage: i32,
    pub target: Entity,
}

pub struct EventSystemPlugin;

impl Plugin for EventSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(process_damage_events)
            .add_event::<DamageEvent>();
    }
}

fn process_damage_events(
    mut commands: Commands,
    mut events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for &DamageEvent { damage, target } in events.iter() {
        if let Ok(mut health) = health_query.get_mut(target) {
            health.take_damage(damage);
            if (health.amount - damage) <= 0 {
                commands.entity(target).despawn();
            }
        }
    }
}
