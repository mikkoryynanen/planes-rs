use bevy::{prelude::*, time::Stopwatch};

use crate::{
    components::Collider,
    entities::entity_loader::{craete_entity_from_atlas, GameSheets},
    moveable::Moveable,
    shoot::Shootable,
    SPRITE_SCALE,
};

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
}

impl Enemy {
    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, sheets: Res<GameSheets>) {
    let enemy_entity = craete_entity_from_atlas(
        &mut commands,
        &sheets.planes,
        3,
        Vec3::new(0., 250., 0.),
        SPRITE_SCALE,
    );

    commands
        .entity(enemy_entity)
        .insert(Name::new("Enemy"))
        .insert(Enemy { health: 100 })
        .insert(Collider)
        .insert(Moveable {
            auto_destroy: false,
            direction: Vec3::new(0., -1., 0.),
            speed: 50.,
        })
        .insert(Shootable {
            direction: Vec3::new(0., -1., 0.),
            shoot_speed_per_ms: 500,
            time: Stopwatch::new(),
            is_shooting: true,
        });
}
