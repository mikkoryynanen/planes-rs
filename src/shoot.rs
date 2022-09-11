use bevy::{prelude::*, sprite::collide_aabb::collide, time::FixedTimestep};

use crate::{
    enemy::Enemy, entities::entity_loader::spawn_entity, moveable::Moveable, player::Player,
    projectile::Projectile, PROJECTILE_SPRITE, SPRITE_SCALE,
};

const TIMESTEP_1_PER_SECOND: f64 = 30.0 / 60.0;

#[derive(Component)]
pub struct Shootable {
    pub direction: Vec3,
}

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_check).add_system_set(
            // TODO: Placeholder implemention
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(shooting_system),
        );
    }
}

fn collision_check(
    mut commands: Commands,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
    projectiles_query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (enemy_entity, enemy_transform) in enemies_query.iter() {
        for (projectile_entity, projectile_tranform) in projectiles_query.iter() {
            let collision = collide(
                enemy_transform.translation,
                Vec2::splat(SPRITE_SCALE),
                projectile_tranform.translation,
                Vec2::splat(SPRITE_SCALE * 0.5),
            );
            if collision.is_some() {
                commands.entity(projectile_entity).despawn();
                // TODO reduce enemy health and despawn once it reaches 0
                // commands.entity(enemy_entity).despawn();
            }
        }
    }
}

fn shooting_system(
    mut commands: Commands,
    shooter_query: Query<(&Transform, &Shootable), With<Shootable>>,
    asset_server: Res<AssetServer>,
) {
    for (shooter_transform, shootable) in shooter_query.iter() {
        let projectile = spawn_entity(
            &mut commands,
            &asset_server,
            PROJECTILE_SPRITE,
            Vec3::new(
                shooter_transform.translation.x,
                shooter_transform.translation.y + 15.,
                100.,
            ),
            Vec3::splat(0.35),
        );

        commands
            .entity(projectile)
            .insert(Name::new("Projectile"))
            .insert(Projectile)
            .insert(Moveable {
                direction: shootable.direction,
                speed: 250.,
                auto_destroy: true,
            });
    }
}
