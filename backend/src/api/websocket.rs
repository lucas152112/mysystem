use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, State},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;
use tracing::{info, warn};

use crate::{database::DatabasePool, models};

// 全局狀態廣播通道
static CHANNEL: tokio::sync::OnceCell<broadcast::Sender<models::StatusLight>> = tokio::sync::OnceCell::const_new();

pub async fn handler(
    ws: WebSocketUpgrade,
    State(_pool): State<DatabasePool>,
) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    info!("WebSocket connection established");
    
    // 獲取或創建廣播通道
    let channel = CHANNEL.get_or_init(|| {
        broadcast::channel::<models::StatusLight>(100).0
    });
    
    let mut rx = channel.subscribe();
    
    // 發送歡迎消息
    if let Err(e) = socket.send(axum::extract::ws::Message::Text(
        serde_json::json!({
            "type": "connected",
            "message": "Connected to mysystem status updates"
        }).to_string()
    )).await {
        warn!("Failed to send welcome message: {}", e);
        return;
    }
    
    // 處理消息循環
    loop {
        tokio::select! {
            // 接收來自客戶端的消息
            msg = socket.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let axum::extract::ws::Message::Text(text) = msg {
                            info!("Received message from client: {}", text);
                            // 可以處理客戶端發送的命令
                        }
                    }
                    Some(Err(e)) => {
                        warn!("WebSocket error: {}", e);
                        break;
                    }
                    None => {
                        info!("WebSocket connection closed by client");
                        break;
                    }
                }
            }
            
            // 接收來自廣播通道的狀態更新
            Ok(status_light) = rx.recv() => {
                let message = serde_json::json!({
                    "type": "status_update",
                    "data": status_light
                }).to_string();
                
                if let Err(e) = socket.send(axum::extract::ws::Message::Text(message)).await {
                    warn!("Failed to send status update: {}", e);
                    break;
                }
            }
        }
    }
    
    info!("WebSocket connection closed");
}

// 發送狀態更新到所有連接的客戶端
pub async fn broadcast_status_update(status_light: models::StatusLight) {
    if let Some(channel) = CHANNEL.get() {
        if let Err(e) = channel.send(status_light) {
            warn!("Failed to broadcast status update: {}", e);
        }
    }
}