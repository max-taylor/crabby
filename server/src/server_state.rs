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

    pub fn add_connection(&mut self, connection: Connection) -> Uuid {
        let uuid = Uuid::new_v4();

        self.connections.insert(uuid, connection);

        uuid
    }

    pub fn new_threadsafe() -> Arc<Mutex<ServerState>> {
        Arc::new(Mutex::new(ServerState::new()))
    }
}
