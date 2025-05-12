use common::user::user_state::UserState;
use futures_util::SinkExt;
use log::error;
use std::net::SocketAddr;
use std::{collections::HashMap, sync::Arc};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use futures_util::stream::SplitSink;

pub struct Connection {
    pub ws_send: SplitSink<WebSocketStream<TcpStream>, Message>,
    pub socket_addr: SocketAddr,
    pub user_state: UserState,
}

pub struct ServerState {
    connections: HashMap<Uuid, Connection>,
}

pub type ServerStateTS = Arc<Mutex<ServerState>>;

impl ServerState {
    pub fn new() -> ServerState {
        ServerState {
            connections: HashMap::new(),
        }
    }

    pub fn add_connection(
        &mut self,
        ws_send: SplitSink<WebSocketStream<TcpStream>, Message>,
        socket_addr: SocketAddr,
    ) -> Uuid {
        let uuid = Uuid::new_v4();

        self.connections.insert(
            uuid,
            Connection {
                ws_send,
                socket_addr,
                user_state: UserState::new(),
            },
        );

        uuid
    }

    pub async fn update(&mut self, time_diff_ms: u64) {
        // This will get way more complicated with checking collisions and updating
        for (_, connection) in self.connections.iter_mut() {
            connection.user_state.update(time_diff_ms);
        }

        for (uuid, connection) in self.connections.iter_mut() {
            let user_state_json = serde_json::to_string(&connection.user_state);
            // TODO: better error handling
            if let Err(e) = user_state_json {
                error!("Error serializing user state for {}: {}", uuid, e);
                continue;
            }

            let user_state_json = user_state_json.unwrap();
            let message = Message::Text(user_state_json.into());
            if let Err(e) = connection.ws_send.send(message).await {
                error!("Error sending message to {}: {}", uuid, e);
                continue;
            }
        }
    }

    pub fn new_threadsafe() -> Arc<Mutex<ServerState>> {
        Arc::new(Mutex::new(ServerState::new()))
    }
}
