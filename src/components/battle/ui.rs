use dioxus::prelude::*;
use indexmap::IndexMap;
use saptest::Pet;
use std::collections::VecDeque;

use crate::components::{
    battle::{
        item_selection::GameItemsContainer,
        selected_pet::PetAttrContainer,
        team::TeamContainer,
        utils::{PetProperty, TeamType},
        ALLOWED_TEAM_SIZE,
    },
    tabs::TabContainer,
};

#[derive(Props)]
pub struct BattleUIState<'a> {
    pub selected_team: &'a UseState<String>,
    pub selected_pet_idx: &'a UseState<Option<usize>>,
    pub selected_item: &'a UseState<Option<String>>,
    pub selected_pet_attr: &'a UseState<String>,
    pub teams: &'a UseState<IndexMap<String, VecDeque<(String, Pet)>>>,
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
            teams: team_pets
        }
    });
    let pet_attr_component = cx.render(rsx! {
        PetAttrContainer {
            selected_team: selected_team,
            selected_item: selected_item,
            selected_pet_idx: selected_pet_idx,
            selected_pet_attr: selected_pet_property,
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
                            PetProperty::Stats.to_string(),
                            pet_attr_component.clone()
                        ),
                        (
                            PetProperty::Effect.to_string(),
                            pet_attr_component.clone()
                        ),
                        (
                            PetProperty::Food.to_string(),
                            pet_attr_component
                        ),
                    ])
                }
            }
        }

        hr {}

        div {
            class: "w3-container",
            GameItemsContainer {
                selected_team: selected_team,
                selected_item: selected_item,
                selected_pet_idx: selected_pet_idx,
                selected_pet_attr: selected_pet_property,
                teams: team_pets
            }
        }
    })
}
