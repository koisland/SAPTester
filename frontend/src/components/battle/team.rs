use dioxus::prelude::*;
use log::info;

use crate::{
    components::battle::{
        ui::BattleUIState,
        utils::{assign_pet_property, remove_pet_from_team, swap_pet_on_team, PetProperty},
    },
    RECORDS,
};

pub fn TeamContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    cx.props.teams.with(|teams| {
        if let Some(selected_team_pets) = teams.get(cx.props.selected_team.get()) {
            cx.render(rsx! {
                table {
                    class: "w3-table w3-responsive w3-white",
                    style: "display: inline-block;",
                    tr {
                        if selected_team_pets.is_empty() {
                            cx.render(rsx! {
                                h2 {
                                    "Click a pet to add it to a team!"
                                }
                                h3 {
                                    "Empty slots located in \"Unknown\" pack at tier 0."
                                }
                            })
                        } else {
                            None
                        }
                        // Pets are added in reverse order to keep frontmost pet at rightside of table row.
                        selected_team_pets.iter().enumerate().map(|(i, (pet_img_url, pet))| {
                            let title = pet.as_ref().map_or("None", |pet| &pet.name);
                            let is_selected = Some(i) == **cx.props.selected_pet_idx;
                            cx.render(rsx! {
                                td {
                                    class: if is_selected { "w3-red" } else { "" },

                                    PetItemIcon(cx, i)

                                    img {
                                        class: "w3-image",
                                        src: "{pet_img_url}",
                                        title: "{title}",
                                        // Starting pet.
                                        ondragstart: move |_| cx.props.selected_pet_idx.set(Some(i)),
                                        // Assign item to pet.
                                        ondragenter: move |_| {
                                            // If user is dragging an item.
                                            if cx.props.selected_item.get().is_some() {
                                                if let Err(err) = assign_pet_property(cx, i, PetProperty::Food(cx.props.selected_item.get().clone())) {
                                                    info!("{err}")
                                                }
                                            } else {
                                                // Otherwise, move pets.
                                                if let Some(from_idx) = cx.props.selected_pet_idx.get() {
                                                    if let Err(err) = swap_pet_on_team(cx, *from_idx, i) {
                                                        info!("{err}")
                                                    }
                                                }
                                            }
                                        },
                                        // Remove pet.
                                        ondblclick: move |_| {
                                            remove_pet_from_team(cx, i)
                                        },
                                        // Set pet as current.
                                        onclick: move |_| cx.props.selected_pet_idx.set(Some(i))
                                    }
                                }
                            })
                        })
                    }
                }
            })
        } else {
            cx.render(rsx! { "Failed to get team pets for {cx.props.selected_team}"})
        }
    })
}

fn PetItemIcon<'a>(cx: Scope<'a, BattleUIState<'a>>, pet_idx: usize) -> Element<'a> {
    let Some(pet_item) = cx.props.teams.with(|teams| {
        teams
            .get(cx.props.selected_team.get())
            .and_then(|team| team.get(pet_idx))
            .and_then(|(_, pet)| pet.as_ref().and_then(|pet| pet.item.clone()))
    }) else {
        return None
    };

    let Some(img_url) = RECORDS.get()
        .and_then(|records| records.get("Foods"))
        .and_then(|foods|
            foods.get(&pet_item).and_then(|food| food.get("img_url")).and_then(|url| url.as_str())
        ) else
    {
        return None
    };

    cx.render(rsx! {
        img {
            class: "w3-image",
            style: "width: 15%;height: 15%;float: left;",
            src: "{img_url}",
            title: "{pet_item}",
            // On item double click, remove item.
            ondblclick: move |_| {
                if let Err(err) = assign_pet_property(cx, pet_idx, PetProperty::Food(None)) {
                    info!("{err}")
                }
            }
        }
    })
}
