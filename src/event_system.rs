use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::{
    animation::{spawn_animated_entity, AnimationSheet},
    components::Health,
    CoreAssets, GameState, Score, UIScore,
};

// Events ========================================
// TODO Maybe move these to their own file
pub struct DamageEvent {
    pub damage: i32,
    pub target: Entity,
    pub translation: Vec3,
}
pub struct CollectionEvent;
// ================================================

pub struct EventSystemPlugin;

impl Plugin for EventSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(process_damage_events)
                .with_system(process_collection_events)
                .into(),
        )
        .add_event::<DamageEvent>()
        .add_event::<CollectionEvent>();
    }
}

fn process_damage_events(
    mut commands: Commands,
    mut events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
    core_asssets: Res<CoreAssets>,
    mut score: ResMut<Score>,
    mut score_query: Query<&mut Text, With<UIScore>>,
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

                // TODO Spawn X amount of collectables once enemy is destroyed

            }

            let animation_sheet = AnimationSheet {
                handle: core_asssets.general.clone(),
                frames: vec![4, 8, 9],
            };

            let _ = spawn_animated_entity(&mut commands, translation, &animation_sheet, 0.1, false);

            score.amount += 100;
            let mut scoreboard = score_query.single_mut();
            scoreboard.sections[0].value = score.amount.to_string();
        }
    }
}

fn process_collection_events(
    mut commands: Commands,
    mut events: EventReader<CollectionEvent>,
) {
    for &CollectionEvent in events.iter() {
        println!("collection event");

        
    }
}
