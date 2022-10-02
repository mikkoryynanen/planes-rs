use crate::{
    animation::{spawn_animated_entity, AnimationSheet},
    collision::Collider,
    components::{Collectable, Health},
    CoreAssets, GameState, Score, UIScore,
};
use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use rand::Rng;

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
    mut damage_events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
    core_assets: Res<CoreAssets>,
    mut score: ResMut<Score>,
    mut score_query: Query<&mut Text, With<UIScore>>,
) {
    for &DamageEvent {
        damage,
        target,
        translation,
    } in damage_events.iter()
    {
        if let Ok(mut health) = health_query.get_mut(target) {
            health.take_damage(damage);
            if (health.amount - damage) <= 0 {
                commands.entity(target).despawn();

                let random_number = rand::thread_rng().gen_range(0..5);
                for index in 0..random_number {
                    let random_position = rand::thread_rng().gen_range(0..50);
                    let collectable = spawn_animated_entity(
                        &mut commands,
                        translation + Vec3::new(random_position as f32, random_position as f32, 0.),
                        &AnimationSheet {
                            handle: core_assets.collectable.clone(),
                            frames: vec![0, 1, 2, 3, 4],
                        },
                        0.2,
                        true,
                    );

                    commands
                        .entity(collectable)
                        .insert(Name::new("Collectable"))
                        .insert(Collectable)
                        .insert(Collider);
                }
            }

            let animation_sheet = AnimationSheet {
                handle: core_assets.general.clone(),
                frames: vec![4, 8, 9],
            };

            let _ = spawn_animated_entity(&mut commands, translation, &animation_sheet, 0.1, false);

            score.amount += 100;
            let mut scoreboard = score_query.single_mut();
            scoreboard.sections[0].value = score.amount.to_string();
        }
    }
}

fn process_collection_events(mut commands: Commands, mut events: EventReader<CollectionEvent>) {
    for &CollectionEvent in events.iter() {
        println!("collection event");
    }
}
