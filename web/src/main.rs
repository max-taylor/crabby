use common::types::position::Position;
use components::crab::Crab;
use dioxus::prelude::*;
use websocket::use_websocket;
mod components;
mod hooks;
mod websocket;
mod workers;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(Layout);
}

#[component]
fn Layout() -> Element {
    rsx!(
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        App { }
    )
}

#[component]
fn App() -> Element {
    let (user_state, mut cursor_position) = use_websocket();

    if user_state().is_none() {
        return rsx!(
            div {
                class: "h-screen w-screen flex items-center justify-center",
                "Loading..."
            }
        );
    }

    let user_state = user_state().unwrap();

    rsx! {
        div {
            class: "h-screen w-screen",
            onkeydown: move |event| {
                // Handle key down events
                match event.key() {
                    Key::ArrowUp => key_event.set(Some(ControlEvent::ArrowUp)),
                    Key::ArrowDown => key_event.set(Some(ControlEvent::ArrowDown)),
                    Key::ArrowLeft => key_event.set(Some(ControlEvent::ArrowLeft)),
                    Key::ArrowRight => key_event.set(Some(ControlEvent::ArrowRight)),
                    Key::Character(val) => {
                        if val == " " {
                            key_event.set(Some(ControlEvent::Space));
                        }
                    },
                    _ => {}
                }
            },
            onkeyup: move |_| {
                key_event.set(None);
            },
            onpointermove: move |event| {
                    // Get global coordinates
                    let client_coords = event.client_coordinates();
                    cursor_position.set(Position {
                        x: client_coords.x as i64,
                        y: client_coords.y as i64,
                    });
                },

            Crab {position: user_state.position}
        }
    }
}
