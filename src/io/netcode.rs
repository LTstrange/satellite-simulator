//! Network code for the server
//!
//! read json request from network, and send request struct use channel
//! receive response struct from channel, and send json response to network

use async_channel::{Receiver, Sender};
use async_net::{TcpListener, TcpStream};
use bevy::tasks::futures_lite::prelude::*;

use super::*;

#[derive(Resource)]
pub struct MyNetChannel {
    pub rx_request: Receiver<NetRequest>,
    pub tx_response: Sender<NetResponse>,
}

pub async fn netcode(
    tx_controls: Sender<NetRequest>,
    rx_response: Receiver<NetResponse>,
    port: u16,
) {
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

        // Send request to bevy event
        tx_controls.send(request).await.unwrap();

        // Receive response from bevy event
        let response = rx_response.recv().await.unwrap();

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