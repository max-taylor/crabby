use common::types::result::CrateResult;
use log::info;
use tokio::{task::JoinHandle, time};

use crate::{constants::WEBSOCKET_UPDATE_RATE_MS, server_state::ServerStateTS};

pub fn spawn_server_state_dispatcher(
    server_state: ServerStateTS,
    websocket_port: u16,
) -> CrateResult<JoinHandle<()>> {
    let server_state = server_state.clone();

    let mut last_update = time::Instant::now();

    let handler = tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(WEBSOCKET_UPDATE_RATE_MS)).await;

            let mut server_state = server_state.lock().await;
            let time_diff_ms = last_update.elapsed().as_millis() as u64;

            server_state.update(WEBSOCKET_UPDATE_RATE_MS).await;
        }
        //
        // // TODO: This isn't setting the time properly
        // last_update = time::Instant::now();
    });

    Ok(handler)
}
