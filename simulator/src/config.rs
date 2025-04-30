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
        info!("Reading dataset from file: {}", self.constellation_file);
        let file = File::open(&self.constellation_file).map_err(|_| "Dataset file not found.")?;
        let reader = BufReader::new(file);
        let satellites_data: Vec<RawSatelliteData> = serde_json::from_reader(reader)?;

        Ok(satellites_data)
    }
}

/// Example of SatelliteData:
/// {
///     "OBJECT_NAME":"STARLINK-1008",
///     "OBJECT_ID":"2019-074B",
///     "EPOCH":"2024-10-27T04:10:58.101312",
///     "MEAN_MOTION":15.06400535,
///     "ECCENTRICITY":0.0001539,
///     "INCLINATION":53.0535,
///     "RA_OF_ASC_NODE":264.5418,
///     "ARG_OF_PERICENTER":101.0513,
///     "MEAN_ANOMALY":259.0649,
///     "EPHEMERIS_TYPE":0,
///     "CLASSIFICATION_TYPE":"U",
///     "NORAD_CAT_ID":44714,
///     "ELEMENT_SET_NO":999,
///     "REV_AT_EPOCH":27361,
///     "BSTAR":0.00030439,
///     "MEAN_MOTION_DOT":4.255e-5,
///     "MEAN_MOTION_DDOT":0
/// }
#[derive(Deserialize, Debug, Clone)]
pub struct RawSatelliteData {
    #[serde(rename = "OBJECT_ID")]
    pub object_id: String,
    #[serde(rename = "EPOCH")]
    pub epoch: String,
    #[serde(rename = "MEAN_MOTION")]
    pub mean_motion: f32, // (rev/day)
    #[serde(rename = "ECCENTRICITY")]
    pub eccentricity: f32,
    #[serde(rename = "INCLINATION")]
    pub inclination: f32, // (degrees)
    #[serde(rename = "RA_OF_ASC_NODE")]
    pub ra_of_asc_node: f32, // (degrees)
    #[serde(rename = "ARG_OF_PERICENTER")]
    pub arg_of_pericenter: f32, // (degrees)
    #[serde(rename = "MEAN_ANOMALY")]
    pub mean_anomaly: f32, // (degrees)
}
