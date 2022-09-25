use bevy::{
    prelude::{Commands, Component, Entity, EventWriter, Plugin, Query, Transform, Vec2, With},
    sprite::collide_aabb::collide,
};
use iyes_loopless::prelude::ConditionSet;

use crate::{
    components::Collectable,
    event_system::{CollectionEvent, DamageEvent},
    GameState,
};

#[derive(Component)]
pub struct Collider;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(collision_check_damageables)
                .with_system(collision_check_collectables)
                .into(),
        );
    }
}

fn collision_check_damageables(
    colliders_query: Query<(Entity, &Transform), With<Collider>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let (collision, entity, transform) = has_collision(&colliders_query);
    if collision {
        println!("There is colllision");
        damage_events.send(DamageEvent {
            damage: 15,
            target: entity.unwrap(),
            translation: transform.unwrap().translation,
        });
    }
}

fn collision_check_collectables(
    mut commands: Commands,
    colliders_query: Query<(Entity, &Transform), With<Collider>>,
    collectables_query: Query<(Entity, &Transform), With<Collectable>>,
    mut collection_events: EventWriter<CollectionEvent>,
) {
    for (collectable_entity, collectable_transform) in collectables_query.iter() {
        for (collider_entity, collider_transform) in colliders_query.iter() {
            if collectable_entity != collider_entity {
                let collision = collide(
                    collectable_transform.translation,
                    Vec2::splat(16.),
                    collider_transform.translation,
                    Vec2::splat(16.),
                );

                if collision.is_some() {
                    commands.entity(collectable_entity).despawn();
                    collection_events.send(CollectionEvent);
                }
            }
        }
    }
}

// Base implementation of the collision, should not be used in the system
fn has_collision(
    colliders_query: &Query<(Entity, &Transform), With<Collider>>,
) -> (bool, Option<Entity>, Option<Transform>) {
    for (a_collider_entity, a_collider_transform) in colliders_query.iter() {
        for (b_collider_entity, b_collider_transform) in colliders_query.iter() {
            if a_collider_entity != b_collider_entity {
                let collision = collide(
                    a_collider_transform.translation,
                    Vec2::splat(16.),
                    b_collider_transform.translation,
                    Vec2::splat(16.),
                );

                return (
                    collision.is_some(),
                    Some(a_collider_entity),
                    Some(*a_collider_transform),
                );
            }
        }
    }
    return (false, None, None);
}
