mod state;

use common::types::result::CrateResult;

use log::error;
use state::ServerStateTS;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
    task::JoinHandle,
};
use tokio_tungstenite::accept_async;

pub async fn spawn_websocket_server(
    server_state: ServerStateTS,
    port: u16,
) -> CrateResult<(JoinHandle<CrateResult<()>>, u16)> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    let port = listener.local_addr().unwrap().port();
    let handler = tokio::spawn(async move {
        loop {
            let listener_value = listener.accept().await;

            if let Err(e) = listener_value {
                error!("Error accepting connection: {}", e);
                continue;
            }

            let (stream, socket_addr) = listener_value?;

            tokio::spawn(async move {
                println!("New connection from: {}", socket_addr);
                // if let Err(e) = handle_connection(socket_addr, stream, server_state).await {
                //     let custom_error = e.downcast_ref::<tokio_tungstenite::tungstenite::Error>();
                //     match custom_error {
                //         Some(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => {
                //             info!("Connection closed: {}", socket_addr);
                //         }
                //         _ => error!("Error handling connection: {}", e),
                //     }
                // }
            });
        }
    });

    Ok((handler, port))
}

// pub async fn handle_connection(
//     peer: SocketAddr,
//     stream: TcpStream,
//     server_state: Arc<Mutex<ServerState>>,
// ) -> CrateResult<()> {
//     let ws_stream = accept_async(stream).await.expect("Failed to accept");
//     info!("New WebSocket connection: {}", peer);
//     let (ws_sender, mut ws_receiver) = ws_stream.split();
//
//     let msg = ws_receiver
//         .next()
//         .await
//         .ok_or(anyhow!("Must send public key as first message"))?;
//
//     // Declare the guard here so that it is dropped when the function returns, which will remove the connection
//     let _guard: ConnectionGuard;
//
//     if let WsMessage::CAddConnection(public_key) = parse_ws_message(msg?)? {
//         info!(
//             "Received public key, adding connection: {:?}",
//             serde_json::to_string(&public_key)?
//         );
//
//         let connection = Connection {
//             public_key: public_key.clone(),
//             ws_send: ws_sender,
//         };
//         _guard = ConnectionGuard {
//             public_key: public_key.clone(),
//             server_state: server_state.clone(),
//         };
//
//         server_state.lock().await.add_connection(connection);
//     } else {
//         return Err(anyhow!("Must send public key as first message"));
//     }
//
//     loop {
//         if let Some(msg) = ws_receiver.next().await {
//             // Intentionally ignore errors here, as we don't want to drop the connection
//             if let Err(e) = handle_loop(msg, server_state.clone()).await {
//                 error!("Error handling message: {:?}", e);
//             }
//         } else {
//             return Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed.into());
//         }
//     }
// }
