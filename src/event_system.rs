use bevy::prelude::*;

use crate::enemy::Enemy;

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
    mut enemy_query: Query<&mut Enemy>,
) {
    for &DamageEvent { damage, target } in events.iter() {
        if let Ok(mut enemy) = enemy_query.get_mut(target) {
            enemy.take_damage(damage);
            if enemy.health - damage <= 0 {
                commands.entity(target).despawn();
            }
        }
    }
}
