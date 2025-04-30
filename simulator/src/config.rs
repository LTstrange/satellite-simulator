use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::prelude::*;

#[derive(Deserialize, Resource, Debug)]
pub struct Config {
    #[serde(rename = "Dataset")]
    pub dataset: Option<Dataset>,
    #[serde(rename = "Display")]
    pub display: Display,
    #[serde(rename = "Simulation")]
    pub simulation: Simulation,
    #[serde(rename = "Network")]
    pub network: Network,
}

#[derive(Deserialize, Debug)]
pub struct Dataset {
    constellation_file: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct Display {
    // #[serde(default)]
    pub orbit: bool,
    // #[serde(default)]
    pub connection: bool,
}

#[derive(Deserialize, Debug)]
pub struct Simulation {
    pub time_speed: f32,
    pub connection_distance: f32,
    pub connection_number: usize,
}

#[derive(Deserialize, Debug)]
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

mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_config() {
        let config_path = PathBuf::from("config.toml");
        let config = Config::load(&config_path).unwrap();
        println!("{:?}", config);
    }
}
