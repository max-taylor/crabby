use dioxus::prelude::MouseEvent;
use dioxus::signals::Writable;
use dioxus::{
    hooks::{use_effect, use_signal},
    signals::Signal,
};
use wasm_bindgen::prelude::*;

// Create a struct to hold cursor position
#[derive(Clone, Debug, PartialEq)]
struct CursorPosition {
    x: i32,
    y: i32,
}

// JavaScript interop to get cursor position
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn addEventListener(event: &str, callback: &Closure<dyn FnMut(JsValue)>);
}

// Alternative implementation using a hook
fn use_cursor_position() -> Signal<CursorPosition> {
    let mut position = use_signal(|| CursorPosition { x: 0, y: 0 });

    // use_effect(move || {
    //     let pos = position.clone();
    //
    //     let closure = Closure::wrap(Box::new(move |event: JsValue| {
    //         let event = MouseEvent::from(event);
    //         pos.set(CursorPosition {
    //             x: event.client_x(),
    //             y: event.client_y(),
    //         });
    //     }) as Box<dyn FnMut(JsValue)>);
    //
    //     let window = web_sys::window().unwrap();
    //     let document = window.document().unwrap();
    //     document
    //         .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
    //         .unwrap();
    //
    //     move || {
    //         document
    //             .remove_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
    //             .unwrap();
    //     }
    // });

    position
}
