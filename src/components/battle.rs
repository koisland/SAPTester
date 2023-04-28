use dioxus::prelude::*;
use log::info;
use saptest::{db::record::SAPRecord, Food, FoodName, Pet};
use sir::css;
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

use crate::{utils::extract_urls::SAPItem, SAP_ITEM_IMG_URLS};

const ALLOWED_TEAM_SIZE: usize = 6;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub enum TeamType {
    #[default]
    Friend,
    Enemy,
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub enum PetProperty {
    #[default]
    Stats,
    Effect,
}

impl std::fmt::Display for TeamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamType::Friend => write!(f, "Friend"),
            TeamType::Enemy => write!(f, "Enemy"),
        }
    }
}

impl std::fmt::Display for PetProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PetProperty::Stats => write!(f, "Stats"),
            PetProperty::Effect => write!(f, "Effect"),
        }
    }
}

#[derive(Props)]
pub struct TeamState<'a> {
    pub selected_team: &'a UseState<String>,
    pub selected_pet_idx: &'a UseState<Option<usize>>,
    pub selected_item: &'a UseState<Option<String>>,
    pub teams: &'a UseState<HashMap<String, VecDeque<(String, Pet)>>>,
}

fn swap_pet_on_team(cx: &Scoped<TeamState>, from: usize, to: usize) -> Result<(), Box<dyn Error>> {
    if from != to {
        let selected_team = cx.props.selected_team.get();
        cx.props.teams.with_mut(|teams| {
            if let Some(selected_team) = teams.get_mut(selected_team) {
                selected_team.swap(from, to);
                // Keep swapped pet as selected pet.
                cx.props.selected_pet_idx.set(Some(to));
            };
        });
    }
    Ok(())
}

fn add_pet_to_team(cx: &Scoped<TeamState>, item_info: &SAPItem) -> Result<(), Box<dyn Error>> {
    let SAPRecord::Pet(pet_record) = &item_info.record else {
        return Err("Got a food record. Cannot add item to team.".into());
    };
    // Create pet only if selected team has less than 6 pets.
    let selected_team = cx.props.selected_team.get();
    if let Some(team) = cx.props.teams.get().get(selected_team) {
        if team.len() >= ALLOWED_TEAM_SIZE {
            // Allow silent failure.
            return Ok(());
        }
    } else {
        return Err("Cannot retrieve team from selected teams.".into());
    }
    let pet = Pet::new(pet_record.name.clone(), None, 1)?;

    // Get a mut handle to the selected team pets.
    cx.props.teams.with_mut(|teams| {
        if let Some(selected_team) = teams.get_mut(selected_team) {
            selected_team.push_front((item_info.icon.to_string(), pet))
        };
    });

    Ok(())
}

fn remove_pet_from_team(cx: &Scoped<TeamState>, pet_idx: usize) {
    cx.props.teams.with_mut(|teams| {
        if let Some(selected_team_pets) = teams.get_mut(cx.props.selected_team.get()) {
            // Remove pet from pets.
            // Should never panic.
            selected_team_pets.remove(pet_idx);

            // Reset selected pet.
            cx.props.selected_pet_idx.set(None)
        }
    })
}

fn assign_food_to_pet(
    cx: &Scoped<TeamState>,
    pet_idx: usize,
    item_name: Option<&String>,
) -> Result<(), Box<dyn Error>> {
    // Convert name to food.
    let food = if let Some(item_name) = item_name {
        Some(Food::try_from(FoodName::from_str(item_name)?)?)
    } else {
        None
    };

    cx.props.teams.with_mut(|teams| {
        if let Some((_, pet)) = teams
            .get_mut(cx.props.selected_team.get())
            .and_then(|team| team.get_mut(pet_idx))
        {
            // Assign pet food or remove food.
            pet.item = food;
        }
    });

    Ok(())
}

pub fn PetsContainer<'a>(cx: Scope<'a, TeamState<'a>>) -> Element {
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

pub fn FoodsContainer<'a>(cx: Scope<'a, TeamState<'a>>) -> Element {
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

pub fn GameItemsContainer<'a>(cx: Scope<'a, TeamState<'a>>) -> Element {
    let selected_item_tab = use_state(cx, || saptest::Entity::Pet.to_string());

    cx.render(rsx! {
        TabContainer {
            desc: "Item",
            selected_tab: selected_item_tab,
            tabs: HashMap::from_iter([
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

fn PetAttrContainer<'a>(cx: Scope<'a, TeamState<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            "{cx.props.selected_pet_idx:?}"
        }
    })
}

fn PetItemIcon<'a>(cx: Scope<'a, TeamState<'a>>, pet_idx: usize) -> Element<'a> {
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

pub fn TeamContainer<'a>(cx: Scope<'a, TeamState<'a>>) -> Element {
    let img_hover_css = css!("img:hover { opacity: 0.7 }");
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
                        class: "w3-border {img_hover_css}",
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
                                        println!("Moving pet {from_idx} to {i}.");
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

#[derive(Props)]
pub struct TabState<'a> {
    pub selected_tab: &'a UseState<String>,
    pub desc: &'a str,
    pub tabs: HashMap<String, Element<'a>>,
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

pub fn Battle(cx: Scope) -> Element {
    // Selected team.
    let selected_team = use_state(cx, || TeamType::default().to_string());
    // Selected item.
    let selected_item = use_state(cx, || None);
    // Selected pet.
    let selected_pet_idx = use_state(cx, || None);
    // Selected pet property.
    let selected_pet_property = use_state(cx, || PetProperty::default().to_string());
    // Stored state for pets.
    let team_pets = use_state(cx, || {
        let mut teams = HashMap::<String, VecDeque<(String, Pet)>>::new();
        teams.insert(
            TeamType::Friend.to_string(),
            VecDeque::with_capacity(ALLOWED_TEAM_SIZE),
        );
        teams.insert(
            TeamType::Enemy.to_string(),
            VecDeque::with_capacity(ALLOWED_TEAM_SIZE),
        );
        teams
    });

    let team_container_component = || {
        cx.render(rsx! {
            TeamContainer {
                selected_team: selected_team,
                selected_item: selected_item,
                selected_pet_idx: selected_pet_idx,
                teams: team_pets
            }
        })
    };
    let pet_attr_component = || {
        cx.render(rsx! {
            PetAttrContainer {
                selected_team: selected_team,
                selected_item: selected_item,
                selected_pet_idx: selected_pet_idx,
                teams: team_pets
            }
        })
    };

    cx.render(rsx! {
        div {
            TabContainer {
                desc: "Team",
                selected_tab: selected_team,
                tabs: HashMap::from_iter([
                    (
                        TeamType::Friend.to_string(),
                        team_container_component()
                    ),
                    (
                        TeamType::Enemy.to_string(),
                        team_container_component()
                    )
                ]),
            }

            hr {}

            TabContainer {
                desc: "Current Pet",
                selected_tab: selected_pet_property,
                tabs: HashMap::from_iter([
                    (
                        PetProperty::Stats.to_string(),
                        pet_attr_component()
                    ),
                    (
                        PetProperty::Effect.to_string(),
                        pet_attr_component()
                    ),
                ])
            }

            hr {}

            GameItemsContainer {
                selected_team: selected_team,
                selected_item: selected_item,
                selected_pet_idx: selected_pet_idx,
                teams: team_pets
            }
        }
    })
}
