use crate::prelude::*;

use bevy::remote::{http::RemoteHttpPlugin, BrpResult, RemotePlugin};
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

/// Add a satellite.
///
/// # Parameters
/// - ID: String - The ID of the satellite.
/// - orbitial elements: [Number, .. ] - The data of the satellite.
fn add_satellite(param: In<Option<Value>>, mut commands: Commands) -> BrpResult<Value> {
    // Example
    // commands.spawn((
    //     satellite,
    //     Name::new(satellite_data.OBJECT_ID),
    //     Mesh3d(satellite_mesh.clone()),
    //     MeshMaterial3d(satellite_material.clone()),
    //     Transform::from_translation(pos),
    // ));

    // What we need:
    // - ID: String                         -- from parameter
    // - orbitial elements: [Number, .. ]   -- from parameter
    // - mesh: Mesh                         -- from a resource
    // - material: StandardMaterial         -- from a resource

    // commands.spawn((
    //     Satellite {
    //         // fill in satellite data
    //     },
    //     Name::new("satellite"),
    //     // fill in satellite mesh and material
    // ));
    todo!()
}
