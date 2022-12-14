use bevy::{prelude::*, time::Stopwatch};
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    collision::Collider,
    components::Health,
    entities::entity_loader::craete_entity_from_atlas,
    moveable::Moveable,
    movement::{self, path_movement::PathMoveable},
    shoot::Shootable,
    CoreAssets, GameState,
};

#[derive(Component)]
pub struct Enemy;

// pub struct EnemyPlugin;

// impl Plugin for EnemyPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_enter_system(GameState::InGame, setup);
//     }
// }

pub fn spawn_enemy(
    commands: &mut Commands,
    core_asssets: &Res<CoreAssets>,
    move_positions_array: Vec<Vec2>,
    movement_speed: f32,
) {
    let enemy_entity = craete_entity_from_atlas(
        commands,
        &core_asssets.plane,
        0,
        move_positions_array[0].extend(100.),
    );

    commands
        .entity(enemy_entity)
        .insert(Name::new(format!("Enemy_{}", enemy_entity.id())))
        .insert(Enemy)
        .insert(Health { amount: 1 })
        .insert(Collider)
        .insert(PathMoveable { next_path_index: 0, move_positions: move_positions_array, movement_speed: movement_speed })
        // .insert(Shootable {
        //     direction: Vec3::new(0., -1., 0.),
        //     source: enemy_entity,
        //     shoot_speed_per_ms: 500,
        //     time: Stopwatch::new(),
        //     is_shooting: true,
        // })
        ;
}
