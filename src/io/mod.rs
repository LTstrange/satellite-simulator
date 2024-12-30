use crate::prelude::*;

use bevy::remote::{RemotePlugin, RemoteServer};

pub struct IOPlugin;

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RemotePlugin,));
    }
}
