use common::types::result::CrateResult;
use uuid::Uuid;

use futures_util::{StreamExt, stream::SplitStream};
use log::*;
use tokio::{
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::Message};

use crate::server_state::{Connection, ServerStateTS};

pub async fn spawn_websocket_server(
    server_state: ServerStateTS,
    port: u16,
) -> CrateResult<JoinHandle<CrateResult<()>>> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    let handler = tokio::spawn(async move {
        loop {
            let listener_value = listener.accept().await;

            if let Err(e) = listener_value {
                error!("Error accepting connection: {}", e);
                continue;
            }

            let (stream, socket_addr) = listener_value?;

            let server_state = server_state.clone();

            tokio::spawn(async move {
                println!("New connection from: {}", socket_addr);

                let ws_stream = accept_async(stream).await.expect("Failed to accept");

                let (ws_sender, ws_receiver) = ws_stream.split();
                let id = server_state.lock().await.add_connection(Connection {
                    ws_send: ws_sender,
                    socket_addr,
                });

                if let Err(e) = handle_connection(ws_receiver, server_state, id).await {
                    let custom_error = e.downcast_ref::<tokio_tungstenite::tungstenite::Error>();
                    match custom_error {
                        Some(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => {
                            info!("Connection closed: {}", socket_addr);
                        }
                        _ => error!("Error handling connection: {}", e),
                    }
                }
            });
        }
    });

    Ok(handler)
}

pub async fn handle_connection(
    mut ws_receiver: SplitStream<WebSocketStream<TcpStream>>,
    server_state: ServerStateTS,
    id: Uuid,
) -> CrateResult<()> {
    loop {
        if let Some(msg) = ws_receiver.next().await {
            let msg = msg?;
            match msg {
                Message::Text(text) => {
                    info!("Received text message: {}", text);
                    // Handle text message
                }
                Message::Binary(_) => {
                    info!("Received binary message");
                    // Handle binary message
                }
                Message::Ping(_) => {
                    info!("Received ping message");
                    // Handle ping message
                }
                Message::Pong(_) => {
                    info!("Received pong message");
                    // Handle pong message
                }
                Message::Close(_) => {
                    info!("Received close message");

                    return Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed.into());
                }
                _ => {}
            }
            // // Intentionally ignore errors here, as we don't want to drop the connection
            // if let Err(e) = handle_loop(msg, server_state.clone()).await {
            //     error!("Error handling message: {:?}", e);
            // }
        } else {
            return Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed.into());
        }
    }
}
