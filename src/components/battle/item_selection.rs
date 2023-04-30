use dioxus::prelude::*;
use indexmap::IndexMap;
use itertools::Itertools;
use log::info;
use saptest::db::pack::Pack;
use sir::css;

use crate::{
    components::{
        battle::{
            ui::{BattleUIState, FILTER_FIELDS},
            utils::{add_pet_to_team, assign_food_to_pet},
            MAX_PET_TIER,
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
            class: "w3-table w3-striped  w3-responsive w3-white {img_hover_css}",
            for (name, item_info) in pets.iter().filter(|(_, item_info)| item_info.is_valid_item(cx)) {
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
            class: "w3-table w3-striped  w3-responsive w3-white {img_hover_css}",
            for (name, item_info) in foods.iter().filter(|(_, item_info)| item_info.is_holdable() && item_info.is_valid_item(cx)) {
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
    })
}

pub fn GameItemsFilterContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let Some((Some(selected_name), Some(selected_tier), Some(selected_pack))) = FILTER_FIELDS.into_iter().map(|field| cx.props.filters.with(|fields| fields.get(field).cloned())).collect_tuple() else {
        return cx.render(rsx! {
            div {
                class: "w3-container",
                h2 {
                    "Unable to get selected field."
                }
            }
        })
    };
    let is_valid_state = use_state(cx, || true);

    cx.render(rsx! {
        div {
            class: "w3-container w3-cell-middle",
            div {
                class: "w3-container",
                h2 { "Filter" }
                h3 { "Name"}
                input {
                    class: "w3-input",
                    name: "Name",
                    "type": "search",
                    value: "{selected_name}",
                    oninput: move |evt| {
                        cx.props.filters.with_mut(|filters| {
                            filters.entry("Name").and_modify(|field| {
                                *field = evt.data.value.clone()
                            });
                        });
                    }
                }
                h3 { "Tier" }
                input {
                    class: if **is_valid_state { "w3-input"} else { "w3-input w3-pale-red"},
                    name: "Tier",
                    "type": "number",
                    value: "{selected_tier}",
                    min: 0,
                    max: "{MAX_PET_TIER}",
                    onchange: move |evt| {
                        if let Ok(tier) = evt.data.value.parse::<usize>().map(|tier| tier.clamp(0, MAX_PET_TIER)) {
                            cx.props.filters.with_mut(|filters| {
                                filters.entry("Tier").and_modify(|field| {
                                    *field = tier.to_string()
                                });
                            });
                        } else {
                            is_valid_state.set(false)
                        }
                    }
                }
                h3 { "Pack" }
                select {
                    class: "w3-select",
                    name: "Pack",
                    value: "{selected_pack}",
                    onchange: move |evt| {
                        cx.props.filters.with_mut(|filters| {
                            filters.entry("Pack").and_modify(|field| {
                                *field = evt.data.value.clone()
                            });
                        });
                    },
                    [Pack::Turtle, Pack::Puppy, Pack::Star, Pack::Weekly].into_iter().map(|pack| {
                        let pack = pack.to_string();
                        cx.render(rsx! {
                            option {
                                value: "{pack}",
                                "{pack}"
                            }
                        })
                    })
                }
            }
        }
    })
}
