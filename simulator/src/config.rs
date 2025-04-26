use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::prelude::*;

#[allow(non_snake_case)]
#[derive(Deserialize, Resource)]
pub struct Config {
    pub Dataset: Option<Dataset>,
    #[serde(default)]
    pub Display: Display,
    pub Simulation: Simulation,
    pub Network: Network,
}

#[derive(Deserialize)]
pub struct Dataset {
    constellation_file: String,
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
        let mut file = File::open(file_path).map_err(|_| "Config file not found.")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

impl Dataset {
    pub fn read_from_file(&self) -> Result<Vec<RawSatelliteData>> {
        let file = File::open(&self.constellation_file).map_err(|_| "Dataset file not found.")?;
        let reader = BufReader::new(file);
        let satellites_data: Vec<RawSatelliteData> = serde_json::from_reader(reader)?;
        Ok(satellites_data)
    }
}
