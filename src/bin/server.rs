use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

type UsersMap = Arc<Mutex<HashMap<String, String>>>;

async fn handle_connection(
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: UsersMap,
    client_id: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    let welcome = WebSocketMessage {
        message_type: MsgTypes::Message,
        data_array: None,
        data: Some("{\"from\": \"System\", \"message\": \"Welcome to chat!\"}".to_string()),
    };
    ws_stream.send(Message::text(serde_json::to_string(&welcome)?)).await?;

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            if let Ok(incoming_msg) = serde_json::from_str::<WebSocketMessage>(text) {
                                match incoming_msg.message_type {
                                    MsgTypes::Register => {
                                        let username = incoming_msg.data.unwrap_or(client_id.clone());
                                        users.lock().unwrap().insert(client_id.clone(), username.clone());

                                        let user_list: Vec<String> = users.lock().unwrap().values().cloned().collect();
                                        let response = WebSocketMessage {
                                            message_type: MsgTypes::Users,
                                            data_array: Some(user_list),
                                            data: None,
                                        };
                                        let json = serde_json::to_string(&response)?;
                                        bcast_tx.send(json)?;
                                    },
                                    MsgTypes::Message => {
                                        let from = users.lock().unwrap().get(&client_id).unwrap_or(&client_id).clone();
                                        let message_text = incoming_msg.data.unwrap_or_default();
                                        let message_data = format!("{{\"from\": \"{}\", \"message\": \"{}\"}}", from, message_text);
                                        let response = WebSocketMessage {
                                            message_type: MsgTypes::Message,
                                            data_array: None,
                                            data: Some(message_data),
                                        };
                                        let json = serde_json::to_string(&response)?;
                                        bcast_tx.send(json)?;
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                    Some(Err(e)) => return Err(e.into()),
                    None => return Ok(()),
                }
            }
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (bcast_tx, _) = channel(100);
    let users: UsersMap = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running at ws://127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        let bcast_tx = bcast_tx.clone();
        let users = users.clone();
        let client_id = Uuid::new_v4().to_string();

        tokio::spawn(async move {
            let ws_stream = ServerBuilder::new().accept(socket).await?;
            handle_connection(ws_stream, bcast_tx, users, client_id).await
        });
    }
}
