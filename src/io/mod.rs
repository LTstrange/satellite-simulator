use crate::prelude::*;
use serde::{Deserialize, Serialize};

use bevy::remote::{error_codes, http::RemoteHttpPlugin, BrpError, BrpResult, RemotePlugin};
use serde_json::Value;

pub struct IOPlugin {
    port: u16,
}

impl IOPlugin {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        let remote_http_plugin = RemoteHttpPlugin::default().with_port(self.port);

        let remote_plugin = RemotePlugin::default().with_method("add_satellite", add_satellite);

        app.add_plugins((remote_plugin, remote_http_plugin));
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AddSatelliteParams {
    id: String,
    elements: [f32; 6],
}

/// A helper function used to parse a `serde_json::Value`.
fn parse<T: for<'de> Deserialize<'de>>(value: Value) -> Result<T, BrpError> {
    serde_json::from_value(value).map_err(|err| BrpError {
        code: error_codes::INVALID_PARAMS,
        message: err.to_string(),
        data: None,
    })
}

/// A helper function used to parse a `serde_json::Value` wrapped in an `Option`.
fn parse_some<T: for<'de> Deserialize<'de>>(value: Option<Value>) -> Result<T, BrpError> {
    match value {
        Some(value) => parse(value),
        None => Err(BrpError {
            code: error_codes::INVALID_PARAMS,
            message: String::from("Params not provided"),
            data: None,
        }),
    }
}

/// Add a satellite.
///
/// # Parameters
/// - ID: String - The ID of the satellite.
/// - orbitial elements: [Number, .. ] - The data of the satellite.
fn add_satellite(
    In(params): In<Option<Value>>,
    mut event: EventWriter<SpawnSatellite>,
) -> BrpResult<Value> {
    let AddSatelliteParams { id, elements } = parse_some(params)?;

    let data = Satellite::from_slice(&elements);
    event.send(SpawnSatellite { id, data });

    BrpResult::Ok(Value::Null)
}
