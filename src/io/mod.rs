use bevy::tasks::IoTaskPool;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

mod netcode;
use netcode::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<NetRequest>().add_event::<NetResponse>();

        // Setup
        app.add_systems(Startup, (setup,));

        // Functionality
        app.add_systems(Update, (handle_request, handle_response));
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum RequestCmd {
    GetTopology,
}

#[derive(Debug, Deserialize, Event)]
pub struct NetRequest {
    pub cmd: RequestCmd,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Event, Clone)]
pub struct NetResponse {
    pub status: String,
    pub data: serde_json::Value,
}

fn setup(mut commands: Commands, config: Res<Config>) {
    let (tx_request, rx_request) = async_channel::unbounded();
    let (tx_response, rx_response) = async_channel::unbounded();

    let port = config.Network.port;
    IoTaskPool::get()
        .spawn(async move {
            netcode(tx_request, rx_response, port).await;
        })
        .detach();

    commands.insert_resource(MyNetChannel {
        rx_request,
        tx_response,
    });
}

fn handle_request(channel: ResMut<MyNetChannel>, mut events: EventWriter<NetRequest>) {
    while let Ok(request) = channel.rx_request.try_recv() {
        events.send(request);
    }
}

fn handle_response(channel: ResMut<MyNetChannel>, mut events: EventReader<NetResponse>) {
    for event in events.read() {
        channel.tx_response.try_send(event.clone()).unwrap();
    }
}
