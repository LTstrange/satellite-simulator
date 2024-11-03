use async_net::TcpListener;

use crate::prelude::*;
use async_channel::{Receiver, Sender};
use bevy::tasks::{futures_lite::prelude::*, IoTaskPool};
pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup,));
    }
}

#[derive(Resource)]
struct MyNetChannel {
    tx_control: Sender<NetControl>,
    rx_reponse: Receiver<NetResponse>,
}

enum NetControl {
    GetAllPositions,
}

enum NetResponse {
    Positions(Vec<Vec3>),
    ControlResult(bool),
}

fn setup(mut commands: Commands, config: Res<Config>) {
    let (tx_control, rx_control) = async_channel::unbounded();
    let (tx_response, rx_response) = async_channel::unbounded();

    let port = config.Network.port;
    IoTaskPool::get()
        .spawn(async move {
            netcode(rx_control, tx_response, port).await;
        })
        .detach();

    commands.insert_resource(MyNetChannel {
        tx_control,
        rx_reponse: rx_response,
    });
}

async fn netcode(rx_control: Receiver<NetControl>, tx_response: Sender<NetResponse>, port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream.unwrap();

        println!("Connection Established!!");
    }
}
