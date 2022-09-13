use crate::{
    components::Collider,
    entities::entity_loader::{craete_entity_from_atlas, GameSheets},
    event_system::DamageEvent,
    moveable::Moveable,
    projectile::Projectile,
    SPRITE_SCALE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide, time::Stopwatch};

#[derive(Component)]
pub struct Shootable {
    pub direction: Vec3,

    pub is_shooting: bool,
    pub shoot_speed_per_ms: u128,
    pub time: Stopwatch,
}

impl Default for Shootable {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            shoot_speed_per_ms: Default::default(),
            time: Default::default(),
            is_shooting: false,
        }
    }
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
                    Vec2::splat(SPRITE_SCALE),
                    projectile_tranform.translation,
                    Vec2::splat(SPRITE_SCALE),
                );
                if collision.is_some() {
                    commands.entity(projectile_entity).despawn();

                    damage_events.send(DamageEvent {
                        damage: 50,
                        target: collider_entity,
                    });
                }
            }
        }
    }
}

fn shooting_system(
    mut commands: Commands,
    mut shooter_query: Query<(Entity, &mut Transform, &mut Shootable), With<Shootable>>,
    sheet: Res<GameSheets>,
    time: Res<Time>,
) {
    for (shooter_entity, shooter_transform, mut shootable) in shooter_query.iter_mut() {
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
                        shooter_transform.translation.y + 15.,
                        100.,
                    ),
                    5.,
                );

                commands
                    .entity(projectile)
                    .insert(Name::new(format!("Projectile_{}", projectile.id())))
                    .insert(Projectile {
                        source: shooter_entity,
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
