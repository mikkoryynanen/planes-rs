use crate::{
    components::Collider,
    entities::entity_loader::{craete_entity_from_atlas, GameSheets},
    event_system::DamageEvent,
    moveable::Moveable,
    projectile::Projectile,
};
use bevy::{prelude::*, sprite::collide_aabb::collide, time::Stopwatch};

#[derive(Component)]
pub struct Shootable {
    pub direction: Vec3,
    pub source: Entity,

    pub is_shooting: bool,
    pub shoot_speed_per_ms: u128,
    pub time: Stopwatch,
}
pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_check).add_system(shooting_system);
    }
}

fn collision_check(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    colliders_query: Query<(Entity, &Transform), With<Collider>>,
    projectiles_query: Query<(Entity, &Projectile, &Transform), With<Projectile>>,
) {
    for (collider_entity, collider_transform) in colliders_query.iter() {
        for (projectile_entity, projectile, projectile_tranform) in projectiles_query.iter() {
            if projectile.source != collider_entity {
                let collision = collide(
                    collider_transform.translation,
                    Vec2::splat(16.),
                    projectile_tranform.translation,
                    Vec2::splat(16.),
                );
                if collision.is_some() {
                    commands.entity(projectile_entity).despawn();

                    damage_events.send(DamageEvent {
                        damage: 15,
                        target: collider_entity,
                        translation: projectile_tranform.translation,
                    });
                }
            }
        }
    }
}

fn shooting_system(
    mut commands: Commands,
    mut shooter_query: Query<(&mut Transform, &mut Shootable), With<Shootable>>,
    sheet: Res<GameSheets>,
    time: Res<Time>,
) {
    for (shooter_transform, mut shootable) in shooter_query.iter_mut() {
        if shootable.is_shooting {
            shootable.time.tick(time.delta());

            let pressed_millis = shootable.time.elapsed().as_millis();

            if pressed_millis > shootable.shoot_speed_per_ms {
                shootable.time.reset();

                let projectile = craete_entity_from_atlas(
                    &mut commands,
                    &sheet.general,
                    0,
                    Vec3::new(
                        shooter_transform.translation.x,
                        shooter_transform.translation.y,
                        100.,
                    ),
                );

                commands
                    .entity(projectile)
                    .insert(Name::new(format!("Projectile_{}", projectile.id())))
                    .insert(Projectile {
                        source: shootable.source,
                    })
                    .insert(Moveable {
                        direction: shootable.direction,
                        speed: 450.,
                        auto_destroy: true,
                    });
            }
        }
    }
}
