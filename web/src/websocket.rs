use common::{
    types::position::Position,
    user::user_state::{ControlEvent, UserState},
};
use dioxus::{
    hooks::{use_context_provider, use_signal},
    prelude::use_hook,
    signals::Signal,
};
use futures::StreamExt;
use gloo_net::websocket::futures::WebSocket;

use crate::workers::{
    sync_local_state::spawn_sync_local_state, sync_server_state::spawn_sync_server_state,
};

pub fn use_websocket() -> (Signal<Option<UserState>>, Signal<Position>) {
    use_context_provider::<Signal<Option<UserState>>>(|| Signal::new(None));

    let user_state = use_signal::<Option<UserState>>(|| None);
    let mut key_event: Signal<Option<ControlEvent>> = use_signal(|| None);

    use_hook(|| {
        let ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
        let (write, read) = ws.split();

        spawn_sync_local_state(read, user_state.clone());
        spawn_sync_server_state(write, user_state.clone(), cursor_position.clone());
    });

    (user_state, cursor_position)
}
