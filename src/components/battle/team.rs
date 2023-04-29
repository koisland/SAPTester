use dioxus::prelude::*;
use log::info;
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
    let teams = &cx.props.teams.get();
    let Some(selected_team_pets) = teams.get(cx.props.selected_team.get()) else {
        return cx.render(rsx! { "Failed to get team pets for {cx.props.selected_team}"})
    };

    // TODO: Make position of tab button and its contents fixed so can always see when scrolling.
    cx.render(rsx! {
        table {
            class: "w3-table w3-striped w3-border w3-responsive w3-white",
            style: "overflow: scroll;",
            tr {
                // Pets are added in reverse order to keep frontmost pet at rightside of table row.
                for (i, (pet_img_url, pet)) in selected_team_pets.iter().enumerate() {
                    td {
                        class: if Some(i) == **cx.props.selected_pet_idx {
                            "w3-border w3-red {_img_hover_css}"
                        } else {
                            "{img_hover_css}"
                        },
                        // Include image of item icon.
                        PetItemIcon(cx, i)

                        img {
                            class: "w3-image",
                            src: "{pet_img_url}",
                            title: "{pet.name}",
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

    let pet = cx
        .props
        .teams
        .get()
        .get(cx.props.selected_team.get())
        .and_then(|team| team.get(pet_idx));

    // Safe to unwrap as assertion at init ensures foods and pets exist.
    if let Some(Some(item)) = pet.and_then(|(_, pet)| {
        pet.item.as_ref().map(|item| {
            SAP_ITEM_IMG_URLS
                .get("Foods")
                .unwrap()
                .get(&item.name.to_string())
        })
    }) {
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
        cx.render(rsx! {
            p {
                "None"
            }
        })
    }
}
