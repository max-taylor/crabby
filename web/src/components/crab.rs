use common::types::position::Position;
use dioxus::prelude::*;

static Ferris: Asset = asset!("/assets/ferris.svg");

#[component]
pub fn Crab(position: Position) -> Element {
    let top = position.y.to_string() + "px";
    let left = position.x.to_string() + "px";

    rsx! {
        img { src: "{Ferris}", class: "absolute w-8 h-8", top: top, left: left }
    }
}
