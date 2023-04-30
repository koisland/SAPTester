use dioxus::prelude::*;
use indexmap::IndexMap;
use saptest::Pet;
use std::collections::{HashMap, VecDeque};

use crate::components::{
    battle::{
        fight::FightSummary,
        item_selection::{GameItemsContainer, GameItemsFilterContainer},
        selected_pet::PetAttrContainer,
        team::TeamContainer,
        utils::TeamType,
        ALLOWED_TEAM_SIZE,
    },
    tabs::TabContainer,
};

pub const FILTER_FIELDS: [&str; 3] = ["Name", "Tier", "Pack"];
pub const FILTER_FIELD_DEFAULTS: [&str; 3] = ["", "1", "Turtle"];

#[derive(Props)]
pub struct BattleUIState<'a> {
    pub selected_team: &'a UseState<String>,
    pub selected_pet_idx: &'a UseState<Option<usize>>,
    pub selected_item: &'a UseState<Option<String>>,
    pub selected_pet_attr: &'a UseState<String>,
    pub filters: &'a UseRef<HashMap<&'static str, String>>,
    pub teams: &'a UseRef<IndexMap<String, VecDeque<(String, Pet)>>>,
}

pub fn Battle(cx: Scope) -> Element {
    // Selected team.
    let selected_team = use_state(cx, || TeamType::default().to_string());
    // Selected item.
    let selected_item = use_state(cx, || None);
    // Selected pet.
    let selected_pet_idx = use_state(cx, || None);
    // Selected pet property.
    let selected_pet_property = use_state(cx, || String::from("Stats"));
    // Item filters.
    let selected_filters = use_ref(cx, || {
        let field_values = FILTER_FIELD_DEFAULTS.map(|field| field.to_owned());
        HashMap::<&str, String>::from_iter(FILTER_FIELDS.into_iter().zip(field_values.into_iter()))
    });
    // Stored state for pets.
    let team_pets = use_ref(cx, || {
        let mut teams = IndexMap::<String, VecDeque<(String, Pet)>>::new();
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

    let team_container_component = cx.render(rsx! {
        TeamContainer {
            selected_team: selected_team,
            selected_item: selected_item,
            selected_pet_idx: selected_pet_idx,
            selected_pet_attr: selected_pet_property,
            filters: selected_filters,
            teams: team_pets
        }
    });
    let pet_attr_component = cx.render(rsx! {
        PetAttrContainer {
            selected_team: selected_team,
            selected_item: selected_item,
            selected_pet_idx: selected_pet_idx,
            selected_pet_attr: selected_pet_property,
            filters: selected_filters,
            teams: team_pets
        }
    });

    cx.render(rsx! {
        div {
            class: "w3-container",
            div {
                class: "w3-container w3-half",
                TabContainer {
                    desc: "Team",
                    selected_tab: selected_team,
                    tabs: IndexMap::from_iter([
                        (
                            TeamType::Friend.to_string(),
                            team_container_component.clone()
                        ),
                        (
                            TeamType::Enemy.to_string(),
                            team_container_component
                        )
                    ]),
                }
            }
            div {
                class: "w3-container w3-half",
                TabContainer {
                    desc: "Current Pet",
                    selected_tab: selected_pet_property,
                    tabs: IndexMap::from_iter([
                        (
                            String::from("Stats"),
                            pet_attr_component.clone()
                        ),
                        (
                            String::from("Effect"),
                            pet_attr_component.clone()
                        ),
                        (
                            String::from("Food"),
                            pet_attr_component
                        ),
                    ])
                }
            }
        }

        br {}

        div {
            class: "w3-container",
            div {
                class: "w3-container w3-threequarter",
                GameItemsContainer {
                    selected_team: selected_team,
                    selected_item: selected_item,
                    selected_pet_idx: selected_pet_idx,
                    selected_pet_attr: selected_pet_property,
                    filters: selected_filters,
                    teams: team_pets
                }
            }
            div {
                class: "w3-container w3-quarter w3-leftbar",
                GameItemsFilterContainer {
                    selected_team: selected_team,
                    selected_item: selected_item,
                    selected_pet_idx: selected_pet_idx,
                    selected_pet_attr: selected_pet_property,
                    filters: selected_filters,
                    teams: team_pets
                },
            }
        }

        br {}

        FightSummary {
            selected_team: selected_team,
            selected_item: selected_item,
            selected_pet_idx: selected_pet_idx,
            selected_pet_attr: selected_pet_property,
            filters: selected_filters,
            teams: team_pets
        }

        // To prevent footer overlap.
        br {}
        br {}
    })
}
