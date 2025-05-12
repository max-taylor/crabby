use common::{constants::websocket::WEBSOCKET_PORT, types::result::CrateResult};
use server_state::ServerState;
use websocket::spawn_websocket_server;

mod constants;
mod positions;
mod server_state;
mod websocket;

#[tokio::main]
async fn main() -> CrateResult<()> {
    env_logger::init();

    let server_state = ServerState::new_threadsafe();
    println!("Hello, world!");

    let ws_server = spawn_websocket_server(server_state.clone(), WEBSOCKET_PORT).await?;
    let dispatcher =
        positions::spawn_server_state_dispatcher(server_state.clone(), WEBSOCKET_PORT)?;

    let ws_result = tokio::try_join!(ws_server, dispatcher);

    if let Err(e) = ws_result {
        eprintln!("Error in WebSocket server: {}", e);
    }

    println!("WebSocket server stopped.");

    Ok(())
}
