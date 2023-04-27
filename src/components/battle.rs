use dioxus::prelude::*;
use log::info;
use saptest::{db::record::SAPRecord, Food, FoodName, Pet};
use sir::css;
use std::{collections::HashMap, error::Error, str::FromStr};

use crate::{utils::extract_urls::SAPItem, SAP_ITEM_IMG_URLS};

const ALLOWED_TEAM_SIZE: usize = 6;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub enum TeamType {
    #[default]
    Friend,
    Enemy,
}

impl std::fmt::Display for TeamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamType::Friend => write!(f, "Friend"),
            TeamType::Enemy => write!(f, "Enemy"),
        }
    }
}

#[derive(Props)]
pub struct TeamState<'a> {
    pub selected_team: &'a UseState<String>,
    pub selected_item: &'a UseState<Option<String>>,
    pub teams: &'a UseState<HashMap<String, Vec<(String, Pet)>>>,
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
            selected_team.push((item_info.icon.to_string(), pet))
        };
    });

    Ok(())
}

fn remove_pet_from_team(cx: &Scoped<TeamState>, pet_idx: usize) {
    cx.props.teams.with_mut(|teams| {
        if let Some(selected_team_pets) = teams.get_mut(cx.props.selected_team.get()) {
            selected_team_pets.remove(pet_idx);
        }
    })
}

fn assign_item_to_pet(cx: &Scoped<TeamState>, pet_idx: usize) -> Result<(), Box<dyn Error>> {
    let Some(item_name) = cx.props.selected_item.get() else {
        return Ok(());
    };

    // Convert name to food.
    let food = Food::try_from(FoodName::from_str(item_name)?)?;

    cx.props.teams.with_mut(|teams| {
        if let Some((_, pet)) = teams
            .get_mut(cx.props.selected_team.get())
            .and_then(|team| team.get_mut(pet_idx))
        {
            // Assign pet food.
            pet.item = Some(food);
        }
    });

    Ok(())
}

pub fn GameItemsContainer<'a>(cx: Scope<'a, TeamState<'a>>) -> Element {
    let img_hover_css = css!("img:hover { opacity: 0.7 }");
    let selected_item_tab = use_state(cx, || saptest::Entity::Pet.to_string());

    let (Some(pets), Some(foods)) = (SAP_ITEM_IMG_URLS.get("Pets"), SAP_ITEM_IMG_URLS.get("Foods")) else {
        return cx.render(rsx! { "Unable to retrieve pet/food information."})
    };
    cx.render(rsx! {
        TabContainer {
            desc: "Item",
            selected_tab: selected_item_tab,
            tabs: HashMap::from_iter([
                (
                    saptest::Entity::Pet.to_string(),
                    cx.render(rsx! {
                        div {
                            class: "w3_container w3-padding {img_hover_css}",
                            for (name, item_info) in pets.iter() {
                                img {
                                    src: "{item_info.icon}",
                                    title: "{name}",
                                    onclick: move |_| {
                                        if let Err(err) = add_pet_to_team(cx, item_info) {
                                            info!("{err}")
                                        }
                                    }
                                }
                            }
                        }
                    })
                ),
                (
                    saptest::Entity::Food.to_string(),
                    cx.render(rsx! {
                        div {
                            class: "w3_container w3-padding {img_hover_css}",
                            for (name, item_info) in foods.iter() {
                                img {
                                    src: "{item_info.icon}",
                                    title: "{name}",
                                    draggable: "true",
                                    // Dragging an item icon selects it; dropping it deselects it.
                                    ondragstart: move |_| cx.props.selected_item.set(Some(name.to_owned())),
                                    ondragend: move |_| cx.props.selected_item.set(None)
                                }
                            }
                        }
                    })
                ),
            ])
        },
    })
}

fn PetItemIcon<'a>(cx: Scope<'a, TeamState<'a>>, pet: &'a Pet) -> Element<'a> {
    // Safe to unwrap as assertion at init ensures foods and pets exist.
    if let Some(Some(item)) = pet.item.as_ref().map(|item| {
        SAP_ITEM_IMG_URLS
            .get("Foods")
            .unwrap()
            .get(&item.name.to_string())
    }) {
        cx.render(rsx! {
            img {
                src: "{item.icon}",
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

    cx.render(rsx! {
        table {
            class: "w3-table w3-striped w3-bordered w3-border w3-hoverable w3-white",
            tr {
                for (i, (pet_img_url, pet)) in selected_team_pets.iter().enumerate() {
                    td {
                        class: "{img_hover_css}",
                        // Include image of item icon.
                        PetItemIcon(cx, pet)

                        img {
                            src: "{pet_img_url}",
                            title: "{i}_{pet.name}",
                            ondragover: move |_| {
                                if let Err(err) = assign_item_to_pet(cx, i) {
                                    info!("{err}")
                                }
                            },
                            onclick: move |_| remove_pet_from_team(cx, i)
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
                    class: "w3-dropdown-content w3-card",
                    for tab in cx.props.tabs.keys() {
                        button {
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
    let selected_team = use_state(cx, || TeamType::default().to_string());
    let selected_item = use_state(cx, || None);
    let team_pets = use_state(cx, || {
        let mut teams = HashMap::<String, Vec<(String, Pet)>>::new();
        teams.insert(
            TeamType::Friend.to_string(),
            Vec::with_capacity(ALLOWED_TEAM_SIZE),
        );
        teams.insert(
            TeamType::Enemy.to_string(),
            Vec::with_capacity(ALLOWED_TEAM_SIZE),
        );
        teams
    });

    cx.render(rsx! {
        div {
            TabContainer {
                desc: "Team",
                selected_tab: selected_team,
                tabs: HashMap::from_iter([
                    (
                        TeamType::Friend.to_string(),
                        cx.render(rsx! {
                            TeamContainer {
                                selected_team: selected_team,
                                selected_item: selected_item,
                                teams: team_pets
                            }
                        })
                    ),
                    (
                        TeamType::Enemy.to_string(),
                        cx.render(rsx! {
                            TeamContainer {
                                selected_team: selected_team,
                                selected_item: selected_item,
                                teams: team_pets
                            }
                        })
                    )
                ]),
            }

            hr {}

            GameItemsContainer {
                selected_team: selected_team,
                selected_item: selected_item,
                teams: team_pets
            }
        }
    })
}
