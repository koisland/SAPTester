use dioxus::prelude::*;
use dioxus_router::Link;

const SAPTEST_URL: &str = "https://github.com/koisland/SuperAutoTest";

pub fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w3-bar w3-top w3-black w3-large",
            div {
                Link {
                    class: "w3-bar-item w3-button w3-hide-small w3-hover-white",
                    to: "/home",
                    "Home"
                }
                Link {
                    class: "w3-bar-item w3-button w3-hide-small w3-hover-white",
                    to: "/battle",
                    "Battle"
                }
                Link {
                    class: "w3-bar-item w3-button w3-hide-small w3-hover-white",
                    to: "/about",
                    "About"
                }
                Link {
                    to: SAPTEST_URL,
                    external: true,
                    class: "w3-bar-item w3-button w3-hide-small w3-hover-white",
                    "GitHub"
                }
            }
        }
    })
}
