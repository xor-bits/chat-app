#![allow(non_snake_case)]

//

use dioxus::prelude::*;

//

fn App(cx: Scope) -> Element {
    let mut state = use_state(cx, || 0);

    let ev = move |ev: Event<KeyboardData>| {
        state += 1;
    };

    cx.render(rsx! {
        div {
            padding_top: "10px",
            display: "flex",
            align_items: "center",
            justify_content: "center",
            "Hello, world!"
        }

        h1 {
            "counter: {state}"
        }

        form {
            input {
                prevent_default: "onkeydown",
                placeholder: "message",
                onkeydown: ev,
            }
        }
    })
}

//

fn main() {
    console_error_panic_hook::set_once();
    console_log::init().expect("error initializing logger");

    dioxus_web::launch(App);
}
