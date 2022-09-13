use crate::{
    enemy::Enemy,
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
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
    projectiles_query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (enemy_entity, enemy_transform) in enemies_query.iter() {
        for (projectile_entity, projectile_tranform) in projectiles_query.iter() {
            let collision = collide(
                enemy_transform.translation,
                Vec2::splat(SPRITE_SCALE),
                projectile_tranform.translation,
                Vec2::splat(SPRITE_SCALE),
            );
            if collision.is_some() {
                commands.entity(projectile_entity).despawn();

                damage_events.send(DamageEvent {
                    damage: 10,
                    target: enemy_entity,
                });
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
                        shooter_transform.translation.y + 15.,
                        100.,
                    ),
                    5.,
                );

                commands
                    .entity(projectile)
                    .insert(Name::new("Projectile"))
                    .insert(Projectile)
                    .insert(Moveable {
                        direction: shootable.direction,
                        speed: 450.,
                        auto_destroy: true,
                    });
            }
        }
    }
}
