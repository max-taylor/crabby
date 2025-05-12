use common::{constants::websocket::WEBSOCKET_PORT, types::result::CrateResult};
use server_state::ServerState;
use websocket::spawn_websocket_server;

mod server_state;
mod websocket;

#[tokio::main]
async fn main() -> CrateResult<()> {
    let server_state = ServerState::new_threadsafe();
    println!("Hello, world!");

    let ws_server = spawn_websocket_server(server_state, WEBSOCKET_PORT).await?;

    let ws_result = tokio::try_join!(ws_server);

    if let Err(e) = ws_result {
        eprintln!("Error in WebSocket server: {}", e);
    }

    println!("WebSocket server stopped.");

    Ok(())
}
