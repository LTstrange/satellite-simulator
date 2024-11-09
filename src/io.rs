use async_channel::{Receiver, Sender};
use async_net::{TcpListener, TcpStream};
use bevy::tasks::{futures_lite::prelude::*, IoTaskPool};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<NetRequest>().add_event::<NetResponse>();

        // Setup
        app.add_systems(Startup, (setup,));
    }
}

#[derive(Resource)]
struct MyNetChannel {
    rx_controls: Receiver<NetRequest>,
    tx_response: Sender<NetResponse>,
}

#[derive(Debug, Deserialize, Event)]
struct NetRequest {
    cmd: String,
    params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Event)]
struct NetResponse {
    status: String,
    data: serde_json::Value,
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

async fn netcode(tx_controls: Sender<NetRequest>, rx_response: Receiver<NetResponse>, port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    info!("Server started on port {}", port);

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from: {}", addr);

        handle_connection(stream, tx_controls.clone(), rx_response.clone())
            .await
            .unwrap();
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    tx_controls: Sender<NetRequest>,
    rx_response: Receiver<NetResponse>,
) -> Result<()> {
    let mut length_buf = [0u8; 4];

    loop {
        // 读取消息长度
        if let Err(e) = stream.read_exact(&mut length_buf).await {
            warn!("Failed to read message length: {}", e);
            break;
        }
        let message_length = u32::from_be_bytes(length_buf) as usize;

        // 读取消息内容
        let mut message = vec![0u8; message_length];
        if let Err(e) = stream.read_exact(&mut message).await {
            warn!("Failed to read message: {}", e);
            break;
        }

        // 解析请求
        let request: NetRequest = match serde_json::from_slice(&message) {
            Ok(req) => req,
            Err(e) => {
                warn!("Failed to parse request: {}", e);
                continue;
            }
        };

        // 处理请求
        let response = match request.cmd.as_str() {
            "get_topology" => NetResponse {
                status: "success".to_string(),
                data: serde_json::json!({
                    "nodes": ["node1", "node2"],
                    "links": [{"source": "node1", "target": "node2"}]
                }),
            },
            _ => NetResponse {
                status: "error".to_string(),
                data: serde_json::json!({
                    "message": format!("Unknown command: {}", request.cmd)
                }),
            },
        };

        // 序列化响应
        let response_data = serde_json::to_vec(&response)?;
        let response_length = response_data.len() as u32;

        // 发送响应长度
        if let Err(e) = stream.write_all(&response_length.to_be_bytes()).await {
            warn!("Failed to send response length: {}", e);
            break;
        }

        // 发送响应内容
        if let Err(e) = stream.write_all(&response_data).await {
            warn!("Failed to send response: {}", e);
            break;
        }
    }

    Ok(())
}
