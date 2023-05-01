use dioxus::prelude::*;
use log::info;
use saptest::PetName;
use sir::css;

use crate::{
    components::battle::{
        ui::BattleUIState,
        utils::{assign_food_to_pet, remove_pet_from_team, swap_pet_on_team},
    },
    SAP_ITEM_IMG_URLS,
};

pub fn TeamContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let _img_hover_css = css!("img:hover { opacity: 0.7 }");
    let teams = &cx.props.teams.read();
    let Some(selected_team_pets) = teams.get(cx.props.selected_team.get()) else {
        return cx.render(rsx! { "Failed to get team pets for {cx.props.selected_team}"})
    };
    let empty_slot_pet = PetName::Custom("Empty".to_owned());

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
                    })
                }
                // Pets are added in reverse order to keep frontmost pet at rightside of table row.
                for (i, (pet_img_url, pet)) in selected_team_pets.iter().enumerate() {
                    td {
                        class: if Some(i) == **cx.props.selected_pet_idx {
                            " w3-red {_img_hover_css}"
                        } else {
                            "{img_hover_css}"
                        },
                        // Include image of item icon.
                        PetItemIcon(cx, i)

                        img {
                            class: "w3-image",
                            src: "{pet_img_url}",
                            title: "{pet.as_ref().map_or(&empty_slot_pet, |pet| &pet.name)}",
                            // Starting pet.
                            ondragstart: move |_| cx.props.selected_pet_idx.set(Some(i)),
                            // Assign item to pet.
                            ondragenter: move |_| {
                                // If user is dragging an item.
                                if cx.props.selected_item.get().is_some() {
                                    if let Err(err) = assign_food_to_pet(cx, i, cx.props.selected_item.get().as_ref()) {
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
                            ondblclick: move |_| remove_pet_from_team(cx, i),
                            // Set pet as current.
                            onclick: move |_| cx.props.selected_pet_idx.set(Some(i))
                        }
                    }
                }
            }
        }
    })
}

fn PetItemIcon<'a>(cx: Scope<'a, BattleUIState<'a>>, pet_idx: usize) -> Element<'a> {
    let item_icon_css = css!(
        "
        width: 15%;
        height: 15%;
        float: left;
    "
    );

    let pet_item = cx.props.teams.with(|teams| {
        teams
            .get(cx.props.selected_team.get())
            .and_then(|team| team.get(pet_idx))
            .and_then(|(_, pet)| {
                if let Some(pet) = pet.as_ref() {
                    pet.item.as_ref().map(|item| {
                        // Safe to access as assertion at init ensures foods and pets exist.
                        SAP_ITEM_IMG_URLS["Foods"].get(&item.name.to_string())
                    })
                } else {
                    None
                }
            })
    });

    if let Some(Some(item)) = pet_item {
        cx.render(rsx! {
            img {
                class: "w3-image {item_icon_css}",
                src: "{item.icon}",
                // On item double click, remove item.
                ondblclick: move |_| {
                    // And remove item.
                    if let Err(err) = assign_food_to_pet(cx, pet_idx, None) {
                        info!("{err}")
                    }
                }
            }
        })
    } else {
        None
    }
}
