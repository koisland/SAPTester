use dioxus::prelude::*;
use crate::SAP_ITEM_IMG_URLS;

#[derive(PartialEq, Props)]
struct FilteredItems {}


pub fn TeamContainer(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w3-container team_container",
            ""
        }
    })
}

#[derive(Props)]
pub struct TabContent<'a> {
    pub tabs: Vec<&'a str>,
    pub tabs_content: Vec<Element<'a>>
}

pub fn TabContainer<'a>(cx: Scope<'a, TabContent<'a>>) -> Element {
    cx.render(rsx! {
        div {
            div {
                class: "tab",
                button {
                    class: "tablinks",
                    onclick: move |_| println!("Hey")
                }
            }

        }
    })
}
pub fn Battle(cx: Scope) -> Element {
    let team_1_pets = use_state(cx, || Vec::<String>::new());
    let team_2_pets = use_state(cx, || Vec::<String>::new());


    cx.render(rsx! {
        div {
            TabContainer {
                tabs: Vec::from_iter(["Team 1", "Team 2"]),
                tabs_content: vec![]
            }

            for (item_categ, items) in SAP_ITEM_IMG_URLS.iter() {
                div {
                    class: "{item_categ}_group",
                    h2 {
                        "{item_categ}"
                    }
                    div {
                        class: "{item_categ}_container",

                        for (_, item_info) in items.iter() {
                            
                            img {
                                src: "{item_info.icon}"
                            }
                        }
                    }
                }
            }
        }
    })
}
