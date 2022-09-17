use bevy::prelude::*;

use crate::{
    components::Health,
    entities::entity_loader::{craete_entity_from_atlas, GameSheets},
};

// TODO Maybe move these to their own file
pub struct DamageEvent {
    pub damage: i32,
    pub target: Entity,
    pub translation: Vec3,
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
    sheets: Res<GameSheets>,
) {
    for &DamageEvent {
        damage,
        target,
        translation,
    } in events.iter()
    {
        if let Ok(mut health) = health_query.get_mut(target) {
            health.take_damage(damage);
            if (health.amount - damage) <= 0 {
                commands.entity(target).despawn();
            }

            let splash = craete_entity_from_atlas(&mut commands, &sheets.general, 6, translation);
        }
    }
}
