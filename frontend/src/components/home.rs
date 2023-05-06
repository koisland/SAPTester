use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello!"
            button {
                onclick: move |_| {

                },
                "",
            }
        }
    })
}
