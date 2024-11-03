use async_channel::{Receiver, Sender};
use async_net::TcpListener;
use bevy::tasks::{futures_lite::prelude::*, IoTaskPool};

use crate::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup,));
        app.add_systems(
            FixedUpdate,
            (handle_net_updates, tell_the_net_task_what_to_do),
        );
    }
}

#[derive(Resource)]
struct MyNetChannel {
    tx_control: Sender<NetControl>,
    rx_reponse: Receiver<NetResponse>,
}

/// Get control messages which change simulator's state
///
/// Example:
///     Connect two pecific Sats.
///     Get all sats locations.
///     Make GroundStation send data through sats.
enum NetControl {}

/// Messages send out
///
/// For example, let outside python code collect data.
enum NetResponse {}

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

fn handle_net_updates(my_channels: Res<MyNetChannel>) {
    // Non-blocking check for any new messages on the channel
    while let Ok(msg) = my_channels.rx_reponse.try_recv() {
        // TODO: do something with `msg`
    }
}

fn tell_the_net_task_what_to_do(my_channels: Res<MyNetChannel>) {
    // if let Err(e) = my_channels.tx_control.try_send(NetControl) {
    //     // TODO: handle errors. Maybe our task has
    //     // returned or panicked, and closed the channel?
    // }
}

async fn netcode(rx_control: Receiver<NetControl>, tx_response: Sender<NetResponse>, port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream.unwrap();

        println!("Connection Established!!");
    }
}
