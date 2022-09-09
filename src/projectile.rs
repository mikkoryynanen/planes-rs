use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct Projectile;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(movement);
    }
}

// fn movement(mut projectile_query: Query<&mut Transform, (With<Projectile>, Without<Player>)>) {
//     for mut projectile in projectile_query.iter_mut() {
//         projectile.translation.y += 1.;
//     }
// }
