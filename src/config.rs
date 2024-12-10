use std::{io::Read, path::Path};

use crate::prelude::*;

#[allow(non_snake_case)]
#[derive(Deserialize, Resource)]
pub struct Config {
    pub Dataset: Dataset,
    #[serde(default)]
    pub Display: Display,
    pub Simulation: Simulation,
    pub Network: Network,
}

#[derive(Deserialize)]
pub struct Dataset {
    pub constellation_file: String,
}

#[derive(Deserialize, Default)]
pub struct Display {
    #[serde(default)]
    pub orbit: bool,
    #[serde(default)]
    pub connection: bool,
}

#[derive(Deserialize)]
pub struct Simulation {
    pub time_speed: f32,
    pub connection_distance: f32,
    pub connection_number: usize,
}

#[derive(Deserialize)]
pub struct Network {
    pub port: u16,
}

impl Config {
    pub fn load(file_path: &Path) -> Result<Self> {
        let mut file = std::fs::File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
