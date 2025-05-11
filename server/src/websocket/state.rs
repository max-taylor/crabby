use std::sync::Arc;

use tokio::sync::Mutex;

pub struct ServerState {}

pub type ServerStateTS = Arc<Mutex<ServerState>>;

impl ServerState {
    pub fn new() -> ServerState {
        ServerState {
                // Initialize your server state here
            }
    }

    pub fn new_threadsafe() -> Arc<Mutex<ServerState>> {
        Arc::new(Mutex::new(ServerState::new()))
    }
}
