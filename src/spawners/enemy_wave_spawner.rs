use std::time::Duration;

use bevy::prelude::*;
use bevy_pixel_camera::PixelProjection;
use iyes_loopless::prelude::{ConditionSet, FixedTimestepStage};

use crate::{enemy::spawn_enemy, CoreAssets, GameState};

struct Wave {
    enemies_to_spawn: usize,
    enemies_spawned: usize,
    wave_position_y: f32,
}

pub struct WaveData {
    waves: Vec<Wave>,
}

impl WaveData {
    fn has_waves(&self) -> bool {
        // TODO This can be optimized?
        return self
            .waves
            .iter()
            .filter(|&wave| wave.enemies_spawned < wave.enemies_to_spawn)
            .collect::<Vec<&Wave>>()
            .len()
            > 0;
    }
}

pub struct EnemyWaveSpawnerPlugin;

impl Plugin for EnemyWaveSpawnerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut spawn_update = SystemStage::parallel();
        spawn_update.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(spawn_wave)
                .into(),
        );

        app.add_stage_before(
            CoreStage::PreUpdate,
            "EnemySpawnPreUpdate",
            FixedTimestepStage::from_stage(Duration::from_millis(500), spawn_update),
        )
        // TODO theese should be loaded from file
        .insert_resource(WaveData {
            waves: vec![
                Wave {
                    enemies_spawned: 0,
                    enemies_to_spawn: 5,
                    wave_position_y: 5.,
                },
                Wave {
                    enemies_spawned: 0,
                    enemies_to_spawn: 5,
                    wave_position_y: 50.,
                },
            ],
        });
    }
}

fn spawn_wave(
    mut commands: Commands,
    core_asssets: Res<CoreAssets>,
    mut wave_data: ResMut<WaveData>,
    camera_query: Query<&Transform, With<PixelProjection>>,
) {
    // Check if it's time to spawn current wave
    // We do this by looking at the cameras Y translation

    if wave_data.has_waves() {
        let camera_transform = camera_query.single();

        for wave in wave_data.waves.iter_mut() {
            if camera_transform.translation.y >= wave.wave_position_y {
                if wave.enemies_spawned < wave.enemies_to_spawn {
                    spawn_enemy(&mut commands, &core_asssets);
                    wave.enemies_spawned += 1;
                    println!("spawning enemy");
                    return;
                }
            }
        }
    } else {
        println!("level completed");
    }
}
