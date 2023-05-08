use dioxus::prelude::*;
use indexmap::IndexMap;
use itertools::Itertools;
use log::info;

use crate::{
    components::{
        battle::{
            ui::{BattleUIState, FILTER_FIELDS},
            utils::{add_pet_to_team, assign_pet_property, PetProperty},
            MAX_PET_TIER,
        },
        tabs::TabContainer,
    },
    EMPTY_SLOT_IMG, RECORDS,
};

pub fn PetsContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let Some(records) = RECORDS.get() else {
        return cx.render(rsx! {
            div {
                class: "w3-container w3-responsive",
                "Click an item type to display its contents."
            }
        })
    };

    let Some(pets) = records.get("Pets") else {
        return cx.render(rsx! {
            div {
                class: "w3-container w3-responsive",
                "Unable to retrieve pet information."
            }
        })
    };

    // TODO: Filter query.
    cx.render(rsx! {
        div {
            class: "w3-table w3-striped w3-responsive w3-white",
            pets.iter()
            // Only show one level of pet.
            .filter(|(_, item_info)| item_info.get("lvl").and_then(|lvl| lvl.as_u64()).eq(&Some(1)))
            .map(|(pet, item_info)| {
                let url = item_info.get("img_url").and_then(|url| url.as_str()).unwrap_or(EMPTY_SLOT_IMG);
                let title = pet.trim_end_matches("_1");
                rsx! {
                    img {
                        class: "w3-image w3-hover-opacity",
                        src: "{url}",
                        title: "{title}",
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
            })
        }
    })
}

pub fn FoodsContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let Some(records) = RECORDS.get() else {
        return cx.render(rsx! {
            div {
                class: "w3-container w3-responsive",
                "Click an item type to display its contents."
            }
        })
    };

    let Some(foods) = records.get("Foods") else {
        return cx.render(rsx! {
            div {
                class: "w3-container w3-responsive",
                "Unable to retrieve food information."
            }
        })
    };

    cx.render(rsx! {
        div {
            class: "w3-table w3-striped w3-responsive w3-white",

            foods.iter().map(|(name, icon)| {
                let title = name.to_string();
                let url = icon.get("img_url").and_then(|url| url.as_str()).unwrap_or(EMPTY_SLOT_IMG);
                rsx! {
                    img {
                        class: "w3-image w3-hover-opacity",
                        src: "{url}",
                        title: "{title}",
                        draggable: "true",
                        // Dragging an item icon selects it; dropping it deselects it.
                        ondragend: move |_| cx.props.selected_item.set(None),
                        ondragstart: move |_| cx.props.selected_item.set(Some(name.to_string())),
                        // On item click, assign to current pet if any.
                        onclick: move |_| {
                            if let Some(Err(err)) = cx.props.selected_pet_idx.get().map(|idx| {
                                // Set selected item.
                                assign_pet_property(cx, idx, PetProperty::Food(Some(name.to_string())))
                            }) {
                                info!("{err}")
                            }
                        }
                    }
                }
            })
        }
    })
}

pub fn GameItemsContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let selected_item_tab = use_state(cx, || String::from("Pet"));

    cx.render(rsx! {
        TabContainer {
            desc: "Item",
            selected_tab: selected_item_tab,
            tabs: IndexMap::from_iter([
                (
                    String::from("Pet"),
                    PetsContainer(cx)
                ),
                (
                    String::from("Food"),
                    FoodsContainer(cx)
                ),
            ])
        },
    })
}

pub fn GameItemsFilterContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let Some((Some(selected_name), Some(selected_tier), Some(selected_pack))) = FILTER_FIELDS
        .into_iter()
        .map(|field| cx.props.filters.with(|fields| fields.get(field).cloned())).collect_tuple() else
    {
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
                    class: "w3-input",
                    name: "Tier",
                    "type": "number",
                    value: "{selected_tier}",
                    min: "0",
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
                    [String::from("Turtle"), String::from("Puppy"), String::from("Star"), String::from("Weekly"),String::from("Other")].into_iter().map(|pack| {
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
