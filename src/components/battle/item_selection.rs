use dioxus::prelude::*;
use indexmap::IndexMap;
use log::info;
use sir::css;

use crate::{
    components::{
        battle::{
            ui::BattleUIState,
            utils::{add_pet_to_team, assign_food_to_pet},
        },
        tabs::TabContainer,
    },
    SAP_ITEM_IMG_URLS,
};

pub fn PetsContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let img_hover_css = css!("img:hover { opacity: 0.7 }");
    let Some(pets) = SAP_ITEM_IMG_URLS.get("Pets") else {
        return cx.render(rsx! { "Unable to retrieve pet information."})
    };
    cx.render(rsx! {
        div {
            class: "w3-table w3-striped w3-border w3-responsive w3-white {img_hover_css}",
            for (name, item_info) in pets.iter() {
                img {
                    class: "w3-image",
                    src: "{item_info.icon}",
                    title: "{name}",
                    // Add pet on click.
                    onclick: move |_| {
                        if let Err(err) = add_pet_to_team(cx, item_info) {
                            info!("{err}")
                        }
                    },
                    // Add pet on drag.
                    ondragstart: move |_| {
                        if let Err(err) = add_pet_to_team(cx, item_info) {
                            info!("{err}")
                        } else {
                            // If successful, set as current pet.
                            cx.props.selected_pet_idx.set(Some(0));
                        }
                    }
                }
            }
        }
    })
}

pub fn FoodsContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let img_hover_css = css!("img:hover { opacity: 0.7 }");
    let Some(foods) = SAP_ITEM_IMG_URLS.get("Foods") else {
        return cx.render(rsx! { "Unable to retrieve food information."})
    };
    cx.render(rsx! {
        div {
            class: "w3-table w3-striped w3-border w3-responsive w3-white {img_hover_css}",
            for (name, item_info) in foods.iter().filter(|(_, item_info)| item_info.is_holdable()) {
                img {
                    class: "w3-image",
                    src: "{item_info.icon}",
                    title: "{name}",
                    draggable: "true",
                    // Dragging an item icon selects it; dropping it deselects it.
                    ondragstart: move |_| cx.props.selected_item.set(Some(name.to_owned())),
                    ondragend: move |_| cx.props.selected_item.set(None),
                    // On item click, assign to current pet if any.
                    onclick: move |_| {
                        if let Some(Err(err)) = cx.props.selected_pet_idx.get().map(|idx| {
                            // Set selected item.
                            assign_food_to_pet(cx, idx, Some(name))
                        }) {
                            info!("{err}")
                        }
                    }
                }
            }
        }
    })
}

pub fn GameItemsContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let selected_item_tab = use_state(cx, || saptest::Entity::Pet.to_string());

    cx.render(rsx! {
        div {
            class: "w3-container",
            style: "overflow: scroll;",
            TabContainer {
                desc: "Item",
                selected_tab: selected_item_tab,
                tabs: IndexMap::from_iter([
                    (
                        saptest::Entity::Pet.to_string(),
                        cx.render(rsx! {
                            PetsContainer(cx)
                        })
                    ),
                    (
                        saptest::Entity::Food.to_string(),
                        cx.render(rsx! {
                            FoodsContainer(cx)
                        })
                    ),
                ])
            },
        }
    })
}
