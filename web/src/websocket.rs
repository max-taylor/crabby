use common::user::user_state::UserState;
use dioxus::{
    hooks::{use_context, use_context_provider},
    logger::tracing::{self},
    prelude::{rsx, spawn, use_hook},
    signals::{Signal, Writable},
};
use futures::StreamExt;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;

pub fn use_websocket() {
    use_context_provider::<Signal<Option<UserState>>>(|| Signal::new(None));

    let mut user_state = use_context::<Signal<Option<UserState>>>();

    use_hook(|| {
        // TODO: FOrmat correctlkjjqiy
        let ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
        let (mut write, mut read) = ws.split();

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
                        _ => {}
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
    });
}
