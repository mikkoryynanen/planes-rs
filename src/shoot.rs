use crate::{
    collision::Collider, entities::entity_loader::craete_entity_from_atlas, moveable::Moveable,
    projectile::Projectile, CoreAssets, GameState,
};
use bevy::{prelude::*, time::Stopwatch};
use iyes_loopless::prelude::ConditionSet;

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
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(shooting_system)
                .into(),
        );
    }
}

fn shooting_system(
    mut commands: Commands,
    mut shooter_query: Query<(&mut Transform, &mut Shootable), With<Shootable>>,
    core_asssets: Res<CoreAssets>,
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
                    &core_asssets.general,
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
                    .insert(Collider)
                    .insert(Moveable {
                        direction: shootable.direction,
                        speed: 450.,
                        auto_destroy: false,
                    });
            }
        }
    }
}
