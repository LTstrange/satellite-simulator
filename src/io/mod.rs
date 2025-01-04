use crate::prelude::*;

use bevy::remote::{http::RemoteHttpPlugin, BrpResult, RemotePlugin};
use serde_json::Value;

pub struct IOPlugin;

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        let remote_plugin = RemotePlugin::default().with_method("add_satellite", add_satellite);
        app.add_plugins((RemoteHttpPlugin::default(), remote_plugin));
    }
}

fn add_satellite(param: In<Option<Value>>, mut commands: Commands) -> BrpResult<Value> {
    todo!()
}
