use common::user::user_state::UserState;
use dioxus::{
    hooks::{use_context_provider, use_signal},
    logger::tracing::info,
    prelude::{spawn, use_hook},
    signals::Signal,
};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
// use wasm_bindgen_futures::spawn_local;

pub fn use_websocket() -> Signal<UserState> {
    let user_state = use_signal(|| UserState::new());

    use_hook(|| {
        // TODO: FOrmat correctly
        let mut ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
        let (mut write, mut read) = ws.split();

        spawn(async move {
            loop {
                let msg = read.next().await;
                match msg {
                    Some(Ok(msg)) => {
                        info!("Received message: {:?}", msg);
                        match msg {
                            Message::Text(text) => {
                                info!("Text message: {}", text);
                            }
                            _ => {}
                        }
                    }
                    Some(Err(e)) => {
                        println!("Error receiving message: {}", e);
                        break;
                    }
                    None => {
                        println!("No message received");
                        break;
                    }
                }
            }
        });
        // This is a placeholder for the actual implementation.
        // In a real-world scenario, this function would contain logic to handle WebSocket messages.
        println!("WebSocket message handler set up.");
    });

    return user_state;

    // use_context_provider(|| Signal::new(UserState::new()));
}
