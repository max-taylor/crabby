use common::user::user_state::UserState;
use dioxus::{logger::tracing::info, prelude::spawn, signals::Signal};
use futures::stream::SplitSink;
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_timers::future::TimeoutFuture;

pub fn spawn_sync_server_state(
    write: SplitSink<WebSocket, Message>,
    mut user_state: Signal<Option<UserState>>,
) {
    spawn(async move {
        loop {
            TimeoutFuture::new(1_000).await;

            // info!("Sending user state to server");
        }
    });
}
