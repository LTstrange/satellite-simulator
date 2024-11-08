use async_channel::{Receiver, Sender};
use async_net::{TcpListener, TcpStream};
use bevy::tasks::{futures_lite::prelude::*, IoTaskPool};

use crate::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup,));
        app.add_systems(FixedUpdate, (handle_net_commands, handle_response_msg));
    }
}

#[derive(Resource)]
struct MyNetChannel {
    tx_response: Sender<NetResponse>,
    rx_controls: Receiver<NetControl>,
}

enum NetControl {
    GetEcho,
}

enum NetResponse {
    Echo,
}

fn setup(mut commands: Commands, config: Res<Config>) {
    let (tx_controls, rx_controls) = async_channel::unbounded();
    let (tx_response, rx_response) = async_channel::unbounded();

    let port = config.Network.port;
    IoTaskPool::get()
        .spawn(async move {
            netcode(tx_controls, rx_response, port).await;
        })
        .detach();

    commands.insert_resource(MyNetChannel {
        tx_response,
        rx_controls,
    });
}

fn handle_net_commands(my_channels: Res<MyNetChannel>) {
    // Non-blocking check for any new messages on the channel
    while let Ok(msg) = my_channels.rx_controls.try_recv() {
        match msg {
            NetControl::GetEcho => todo!(),
        }
    }
}

fn handle_response_msg(my_channels: Res<MyNetChannel>) {
    if let Err(e) = my_channels.tx_response.try_send(NetResponse::Echo) {}
}

async fn netcode(tx_controls: Sender<NetControl>, rx_response: Receiver<NetResponse>, port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    info!("Server started on port {}", port);

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from: {}", addr);

        handle_connection(stream).await;
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut command = [0; 1024];
    while let Ok(n) = stream.read(&mut command).await {
        if n == 0 {
            break;
        }
        let command = &command[..n];
        let response = match command {
            b"get_topology" => "Topology",
            _ => "Unknown Command",
        };
        if let Err(e) = stream.write_all(response.as_bytes()).await {
            error!("Failed to write response: {}", e);
            break;
        }
    }
}
