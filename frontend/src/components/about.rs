use dioxus::prelude::*;

use crate::{SAPAI_URL, SAPTEST_URL};

pub fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w3-container",
            div {
                class: "w3-container",
                p {
                    "SAPTester, the frontend for the library "
                    a { href: SAPTEST_URL, "saptest" }
                    ", began as a project to learn Rust."
                }
                p {
                    "While other projects exist for emulating Super Auto Pets, namely ",
                    a { href: SAPAI_URL, "sapai" },
                    ", I found them unsatisfactory for a few reasons."
                }
                ul {
                    li {
                        "Stale data."
                    }
                    li {
                        "Inability to create new effects."
                    }
                    li {
                        "Fixed team size."
                    }
                }
                p {
                    "This site serves as a playground to test team battles. However, it is not nearly as flexible as the library it depends on."
                }
            }

            br {}
        }
    })
}
