// /src/ws.rs

use std::collections::HashMap;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use chrono::Utc;
use futures_util::{StreamExt, SinkExt};
use uuid::Uuid;

use crate::{models::ChatMessage, AppState};

/// Connect like:
///   ws://localhost:3000/ws?user=alice&channel=general&echo=false
///
/// Query params:
/// - user: optional, defaults to random UUID
/// - channel: optional, defaults to "default"
/// - echo: "false" to avoid sending your own messages back (default: true)
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let sender_id = params
        .get("user")
        .cloned()
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let channel = params
        .get("channel")
        .cloned()
        .unwrap_or_else(|| "default".to_string());
    let echo = params
        .get("echo")
        .map(|v| v != "false")
        .unwrap_or(true);

    ws.on_upgrade(move |socket| handle_socket(socket, state, sender_id, channel, echo))
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    sender_id: String,
    channel: String,
    echo: bool,
) {
    let mut rx = state.broker.subscribe();
    let (mut sink, mut stream) = socket.split();

    loop {
        tokio::select! {
            // Incoming message -> broadcast
            maybe_msg = stream.next() => {
                match maybe_msg {
                    Some(Ok(Message::Text(text))) => {
                        let chat_msg = ChatMessage {
                            id: Uuid::new_v4(),
                            channel: channel.clone(),
                            sender_id: sender_id.clone(),
                            content: text,
                            created_at: Utc::now(),
                        };
                        state.broker.send(chat_msg);
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => { /* ignore */ }
                    Some(Err(_)) => break,
                }
            }

            // Broadcast -> send to client
            Ok(bcast_msg) = rx.recv() => {
                if bcast_msg.channel != channel {
                    continue;
                }
                if !echo && bcast_msg.sender_id == sender_id {
                    continue;
                }

                // send plain text: "sender_id: content"
                let line = format!("{}: {}", bcast_msg.sender_id, bcast_msg.content);

                if sink.send(Message::Text(line)).await.is_err() {
                    break;
                }
            }
        }
    }
}
