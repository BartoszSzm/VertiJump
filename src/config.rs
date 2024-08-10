use crate::extensions::Square;
use serde;
use std::fs;
use toml;

#[derive(serde::Deserialize)]
pub struct Config {
    pub window_title: String,
    pub fullscreen: bool,
    pub window_resizable: bool,
    pub window_dims: Square<i32>,
    pub player_tile_dims: Square<u32>,
    pub camera_zoom_x: i32,
    pub camera_zoom_y: i32,
    pub max_fps: u32,
    pub paths: Paths,
    pub spawn_point_x: f32,
    pub spawn_point_y: f32,
}

#[derive(serde::Deserialize)]
pub struct Paths {
    pub player_texture_filepath: String,
    pub map_filepath: String,
    pub map_terrain_texture: String,
}

pub fn get_config(path: &str) -> Config {
    let config_str =
        fs::read_to_string(path).expect(&format!("Error loading config file {}", path));
    let config: Config = toml::from_str(&config_str).expect("Error parsing config file");
    config
}
