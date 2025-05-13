use common::user::user_state::UserState;
use dioxus::{
    hooks::{use_context, use_context_provider},
    prelude::use_hook,
    signals::Signal,
};
use futures::StreamExt;
use gloo_net::websocket::futures::WebSocket;

use crate::workers::sync_local_state::spawn_sync_local_state;

pub fn use_websocket() {
    use_context_provider::<Signal<Option<UserState>>>(|| Signal::new(None));

    let user_state = use_context::<Signal<Option<UserState>>>();

    use_hook(|| {
        let ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
        let (write, read) = ws.split();

        spawn_sync_local_state(read, user_state.clone());
    });
}
