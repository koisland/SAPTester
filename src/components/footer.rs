use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        footer {
            class: "w3-container w3-black",
            p {
                "Generated with Dioxus and Axum."
            }
            p {
                "© 2023 Keisuke K. Oshima"
            }
        }
    })
}
