use std::collections::HashMap;

use crate::{utils::extract_urls::SAPItem, SAP_ITEM_IMG_URLS};
use dioxus::prelude::*;
use sir::css;

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
pub struct StoredTeams<'a> {
    pub selected_team: &'a UseState<TeamType>,
    pub teams: &'a UseState<HashMap<TeamType, Vec<SAPItem>>>,
}

fn add_pet_to_team(cx: &Scoped<StoredTeams>, item_info: &SAPItem) {
    // Append to team if less than 6 pets.
    if cx.props.teams.len() != ALLOWED_TEAM_SIZE {
        // Get a handle to the selected team pets.
        cx.props.teams.with_mut(|teams| {
            if let Some(selected_team) = teams.get_mut(cx.props.selected_team) {
                selected_team.push(item_info.clone())
            };
        })
    }
}

pub fn GameItemsContainer<'a>(cx: Scope<'a, StoredTeams<'a>>) -> Element {
    let img_hover_css = css!("img:hover { opacity: 0.7 }");

    cx.render(rsx! {
        for (item_categ, items) in SAP_ITEM_IMG_URLS.iter() {
            div {
                class: "w3_container w3-padding",
                h2 {
                    "{item_categ}"
                }
                div {
                    class: "w3_container w3-padding {img_hover_css}",

                    for (_, item_info) in items.iter() {
                        img {
                            src: "{item_info.icon}",
                            onclick: move |_| { add_pet_to_team(cx, item_info) }
                        }
                    }
                }
            }
        }
    })
}

pub fn TeamContainer<'a>(cx: Scope<'a, StoredTeams<'a>>) -> Element {
    let teams = &cx.props.teams.get();
    let Some(selected_team_pets) = teams.get(cx.props.selected_team) else {
        return cx.render(rsx! { "Failed to get team pets for {cx.props.selected_team}"})
    };

    cx.render(rsx! {
        table {
            class: "w3-table w3-striped w3-bordered w3-border w3-hoverable w3-white",
            tr {
                for pet in selected_team_pets.iter() {
                    td {
                        img {
                            src: "{pet.icon}",
                            onclick: move |_| {
                                println!("I was clicked.")
                            }
                        }
                    }
                }
            }
        }
    })
}

#[derive(Props)]
pub struct TabsContent<'a> {
    pub selected_tab: &'a UseState<TeamType>,
    pub tabs: HashMap<TeamType, Element<'a>>,
}

pub fn TabContainer<'a>(cx: Scope<'a, TabsContent<'a>>) -> Element {
    let Some(selected_tab_contents) = cx.props.tabs.get(cx.props.selected_tab.get()) else {
        return cx.render(rsx! { "Failed to get tab for {cx.props.selected_tab}"})
    };
    cx.render(rsx! {
        div {
            class: "w3-container",
            for tab in cx.props.tabs.keys() {
                div {
                    class: "w3-container w3-padding ",

                }
                button {
                    class: "w3-button w3-padding",
                    id: "{tab}",
                    onclick: move |_| cx.props.selected_tab.set(tab.clone()) ,
                    "{tab}"
                }
            }
        }
        selected_tab_contents
    })
}

pub fn Battle(cx: Scope) -> Element {
    let selected_team = use_state(cx, TeamType::default);
    let team_pets = use_state(cx, || {
        let mut teams = HashMap::<TeamType, Vec<SAPItem>>::new();
        teams.insert(TeamType::Friend, Vec::with_capacity(ALLOWED_TEAM_SIZE));
        teams.insert(TeamType::Enemy, Vec::with_capacity(ALLOWED_TEAM_SIZE));
        teams
    });

    cx.render(rsx! {
        div {
            TabContainer {
                selected_tab: selected_team,
                tabs: HashMap::from_iter([
                    (
                        TeamType::Friend,
                        cx.render(rsx! {
                            TeamContainer {
                                selected_team: selected_team
                                teams: team_pets
                            }
                        })
                    ),
                    (
                        TeamType::Enemy,
                        cx.render(rsx! {
                            TeamContainer {
                                selected_team: selected_team
                                teams: team_pets
                            }
                        })
                    )
                ]),
            }

            GameItemsContainer {
                selected_team: selected_team,
                teams: team_pets
            }
        }
    })
}
