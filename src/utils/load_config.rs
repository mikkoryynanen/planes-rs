use serde::Deserialize;
use std::{fs, process::exit};

// Data =========================================================
#[derive(serde::Deserialize)]
pub struct ConfigData {
    pub general: General,
    pub paths: Paths,
    pub sprites: Sprites,
    pub player: Player,
    pub animations: Animations,
    pub enemies: Enemies,
}

#[derive(Deserialize)]
pub struct General {
    pub base_aspect_ratio: f32,
    pub screen_height: f32,
    pub scroll_speed: f32,
}

#[derive(Deserialize)]
pub struct Paths {
    pub tiles_path: String,
    pub planes_path: String,
    pub background_path: String,
}

#[derive(Deserialize)]
pub struct Sprites {
    pub sprite_scale: f32,
    pub tile_size: f32,
    pub tile_padding: f32,
}

#[derive(Deserialize)]
pub struct Player {
    pub base_health: i32,
    pub movement_speed: f32,
    pub max_speed: f32,
}

#[derive(Deserialize)]
pub struct Animations {
    pub default_frame_duration: f32,
    pub explosion_frame_duratioon: f32,
}

#[derive(Deserialize)]
pub struct Enemies {
    pub movement_speed: f32,
}

// =============================================================

pub fn load_config() -> ConfigData {
    // TODO Replace with relative path
    let filename = "/Users/mikkoryynanen/Desktop/Projects/Rust/planes/config.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(error) => {
            eprintln!("Could not read file `{}`, error {}", filename, error);
            exit(1);
        }
    };

    let data: ConfigData = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(error) => {
            eprintln!(
                "Unable to load data from `{}`, error {}",
                filename,
                error.to_owned()
            );
            exit(1);
        }
    };

    return data;
}
