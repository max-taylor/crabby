use common::user::user_state::UserState;
use futures_util::SinkExt;
use log::{error, info, warn};
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

    pub fn remove_connection(&mut self, uuid: Uuid) {
        self.connections.remove(&uuid);
    }

    pub fn update_connection_direction(&mut self, uuid: Uuid, direction_deg: u64) -> Option<()> {
        if let Some(connection) = self.connections.get_mut(&uuid) {
            info!(
                "Updating connection direction for {}: {}",
                uuid, direction_deg
            );
            connection.user_state.direction_deg = direction_deg;
            Some(())
        } else {
            // TODO: How to handle this, reconnect, or simply remove?
            warn!("Connection not found for UUID: {}", uuid);

            None
        }
    }

    pub async fn update(&mut self, time_diff_ms: u64) {
        // This will get way more complicated with checking collisions and updating
        for (_, connection) in self.connections.iter_mut() {
            connection.user_state.update(time_diff_ms);
        }

        let mut ids_to_remove: Vec<Uuid> = vec![];

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
                // TODO: Remove connection
                error!("Error sending message to {}: {}", uuid, e);

                ids_to_remove.push(*uuid);
                continue;
            }
        }

        for uuid in ids_to_remove {
            self.remove_connection(uuid);
        }
    }

    pub fn new_threadsafe() -> Arc<Mutex<ServerState>> {
        Arc::new(Mutex::new(ServerState::new()))
    }
}
