use common::types::position::Position;
use dioxus::prelude::*;
use websocket::use_websocket;
mod hooks;
mod websocket;
mod workers;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let (user_state, mut cursor_position) = use_websocket();

    let mut top = "0px".to_string();
    let mut left = "0px".to_string();
    if let Some(user_state) = user_state() {
        top = user_state.position.y.to_string() + "px";
        left = user_state.position.x.to_string() + "px";
    }

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "h-screen w-screen",
            onpointermove: move |event| {
                    // Get global coordinates
                    let client_coords = event.client_coordinates();
                    cursor_position.set(Position {
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
