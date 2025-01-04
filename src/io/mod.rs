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

fn add_satellite(param: In<Option<Value>>, mut commands: Commands) -> BrpResult<Value> {
    todo!()
}
