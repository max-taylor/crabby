use dioxus::{logger::tracing, prelude::*};
use websocket::use_websocket;
mod websocket;

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
    let user_state = use_websocket();

    tracing::info!("User state: {:?}", user_state);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen bg-gray-100",
            div {
                min_height: 200,
                min_width: 200,
            height: 200,
            width: 200,
                background: "linear-gradient(to right, #4f46e5, #3b82f6)",
            }
            p {
                class: "mt-4 text-lg text-gray-600",
                "This is a simple example of using WebSockets in a Dioxus application."
            }
        }
    }
}
