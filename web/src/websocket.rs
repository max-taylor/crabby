use common::user::user_state::UserState;
use dioxus::{
    hooks::{use_context, use_context_provider},
    prelude::{rsx, use_hook},
    signals::Signal,
};
use futures::StreamExt;
use gloo_net::websocket::futures::WebSocket;

use crate::workers::{
    sync_local_state::spawn_sync_local_state, sync_server_state::spawn_sync_server_state,
};

pub fn use_websocket() {
    use_context_provider::<Signal<Option<UserState>>>(|| Signal::new(None));

    let user_state = use_context::<Signal<Option<UserState>>>();

    use_hook(|| {
        let ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
        let (write, read) = ws.split();

        spawn_sync_local_state(read, user_state.clone());
        spawn_sync_server_state(write, user_state.clone());
    });

    rsx! {
        div {
            class: "h-screen w-screen",
            onpointermove: move |event| {
                    // // Get element-relative coordinates
                    // let element_coords = event.element_coordinates();
                    // element_x.set(element_coords.x as i32);
                    // element_y.set(element_coords.y as i32);

                    // If we wanted to update the global position too:
                    let client_coords = event.client_coordinates();
                    cursor.set(Position {
                        x: client_coords.x as i64,
                        y: client_coords.y as i64,
                    });
                },
            div {
                class: "absolute min-w-8 min-h-8 bg-gray-100 rounded-full",
                top: top,
                left: left,
            }
        }
    }
}
