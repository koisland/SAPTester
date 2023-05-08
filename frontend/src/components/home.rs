use dioxus::prelude::*;

const SAP_LOGO: &str =
    "https://static.wikia.nocookie.net/superautopets/images/5/5d/Super_Auto_Pets_Header.jpg";

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w3-container w3-white",

            div {
                class: "w3-container",
                text_align: "center",
                h1 {
                    class: "w3-text-black",
                    "Welcome to SAPTester!"
                }
                h3 {
                    "A site to test teams from the game:\t"
                    img {
                        class: "w3-image w3-round",
                        src: "{SAP_LOGO}",
                        border: "2px solid"
                    }
                }

                div {
                    class: "w3-panel w3-large w3-text-red",
                    "This is an unofficial project not affliated with Team Wood Games."
                }
            }

            br {}

        }
    })
}
