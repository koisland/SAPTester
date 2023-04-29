use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(Props)]
pub struct TabState<'a> {
    pub selected_tab: &'a UseState<String>,
    pub desc: &'a str,
    pub tabs: IndexMap<String, Element<'a>>,
}

pub fn TabContainer<'a>(cx: Scope<'a, TabState<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w3-container",
            div {
                class: "w3-dropdown-hover",
                button {
                    class: "w3-button",
                    "{cx.props.desc}"
                }
                div {
                    class: "w3-dropdown-content",
                    for tab in cx.props.tabs.keys() {
                        button {
                            class: "w3-button",
                            onclick: move |_| cx.props.selected_tab.set(tab.clone()),
                            "{tab}"
                        }
                        br {}
                    }

                }
            }
            if let Some(selected_tab_contents) = cx.props.tabs.get(cx.props.selected_tab.get()) {
                rsx! { selected_tab_contents }
            } else {
                rsx! {"Failed to get tab for {cx.props.selected_tab}"}
            }
        }
    })
}
