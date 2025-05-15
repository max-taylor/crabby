use common::{types::position::Position, user::user_state::UserState};
use dioxus::{logger::tracing::info, prelude::*};
use websocket::use_websocket;
mod hooks;
mod websocket;
mod workers;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_websocket();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let user_state = use_context::<Signal<Option<UserState>>>();
    let mut cursor = use_signal(|| Position { x: 0, y: 0 });

    let mut top = "0px".to_string();
    let mut left = "0px".to_string();
    if let Some(user_state) = user_state() {
        top = user_state.position.y.to_string() + "px";
        left = user_state.position.x.to_string() + "px";
    }

    info!("Cursor position: {:?}", cursor);

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
