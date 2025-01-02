use crate::prelude::*;

use bevy::remote::{http::RemoteHttpPlugin, RemotePlugin};

pub struct IOPlugin;

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RemoteHttpPlugin::default(), RemotePlugin::default()));
    }
}
