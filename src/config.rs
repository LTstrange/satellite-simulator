use std::{io::Read, path::Path};

use crate::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Resource)]
pub struct Config {
    pub satellite_json: String,
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
