use common::types::position::Position;
use dioxus::prelude::*;
use dioxus::{
    hooks::use_effect,
    signals::{Signal, Writable},
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::js_sys::Function;

/// A hook that tracks cursor position within the browser window
pub fn use_cursor_position() -> Signal<Position> {
    let position = use_signal(Position::default);

    use_effect(move || {
        // Create a stable reference to the signal
        let mut position = position.clone();

        // Create a closure that updates the position when the mouse moves
        let callback = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            position.set(Position {
                x: event.client_x() as i64,
                y: event.client_y() as i64,
            });
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);

        // Get window and document objects
        let window = web_sys::window().expect("No global window exists");
        let document = window.document().expect("No document exists on window");

        // Properly cast the closure to a JS function and add the event listener
        let listener: &Function = callback.as_ref().unchecked_ref();
        document
            .add_event_listener_with_callback("mousemove", listener)
            .expect("Failed to add event listener");

        // Store the callback so it won't be dropped
        let callback = callback.forget();

        // Return cleanup function - note: no move closure here, just a direct return
        // || {
        //     if let Some(window) = web_sys::window() {
        //         if let Some(document) = window.document() {
        //             // When unmounting, remove the event listener to prevent memory leaks
        //             let _ = document.remove_event_listener_with_callback("mousemove", listener);
        //         }
        //     }
        // }

        // // Return cleanup function that properly removes the event listener
        // move || {
        //     if let Ok(document) = window.document() {
        //         // When unmounting, remove the event listener to prevent memory leaks
        //         let _ = document.remove_event_listener_with_callback("mousemove", listener);
        //     }
        //     // Explicitly drop the closure to free memory
        //     drop(callback);
        // }
    });

    position
}
