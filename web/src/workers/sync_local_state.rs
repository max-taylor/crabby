use common::user::user_state::UserState;
use dioxus::{
    logger::tracing,
    prelude::spawn,
    signals::{Signal, Writable},
};
use futures::{stream::SplitStream, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};

// Listen to messages received from the websocket server and update the local user_state
pub fn spawn_sync_local_state(
    mut read: SplitStream<WebSocket>,
    mut user_state: Signal<Option<UserState>>,
) {
    spawn(async move {
        loop {
            let msg = read.next().await;
            match msg {
                Some(Ok(msg)) => match msg {
                    Message::Text(text) => {
                        let received_state = serde_json::from_str::<UserState>(&text);

                        match received_state {
                            Ok(received_state) => {
                                let updated_state = user_state();

                                if let Some(mut updated_state) = updated_state {
                                    updated_state.position = received_state.position;

                                    user_state.set(Some(updated_state));
                                } else {
                                    user_state.set(Some(received_state));
                                }
                            }
                            Err(e) => {
                                tracing::error!("Error parsing user state: {}", e);
                            }
                        }
                    }
                    args => tracing::warn!("Other message type received {:?}", args),
                },
                Some(Err(e)) => {
                    tracing::error!("Error receiving message: {}", e);
                    break;
                }
                None => {
                    tracing::error!("No message received");
                }
            }
        }
    });
}
