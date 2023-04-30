use std::{error::Error, str::FromStr};

use dioxus::prelude::*;
use saptest::{db::record::SAPRecord, Effect, Food, FoodName, Pet, Statistics};

use crate::{components::battle::ui::BattleUIState, utils::extract_urls::SAPItem};

use super::ALLOWED_TEAM_SIZE;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub enum TeamType {
    #[default]
    Friend,
    Enemy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PetProperty {
    Stats(Statistics),
    Effect(Vec<Effect>),
    Food(Option<Food>),
    Level(usize),
}

impl std::fmt::Display for TeamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamType::Friend => write!(f, "Friend"),
            TeamType::Enemy => write!(f, "Enemy"),
        }
    }
}

pub fn get_selected_pet_property(
    cx: Scope<BattleUIState>,
    property_name: &str,
) -> Option<PetProperty> {
    cx
        .props
        .teams.with(|teams| {
            let selected_team = teams.get(cx.props.selected_team.get());
            let (Some(team), Some(pet_idx)) = (selected_team, cx.props.selected_pet_idx.get()) else {
                return None
            };
            team.get(*pet_idx).map(|(_, pet)| {
                match property_name {
                    "Stats" => Some(PetProperty::Stats(pet.stats)),
                    "Effect" => Some(PetProperty::Effect(pet.effect.clone())),
                    "Food" => Some(PetProperty::Food(pet.item.clone())),
                    "Level" => Some(PetProperty::Level(pet.get_level())),
                    _ => None
                }
            })
        }
    ).flatten()
}

pub fn swap_pet_on_team(
    cx: &Scoped<BattleUIState>,
    from: usize,
    to: usize,
) -> Result<(), Box<dyn Error>> {
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

pub fn add_pet_to_team(
    cx: &Scoped<BattleUIState>,
    item_info: &SAPItem,
) -> Result<(), Box<dyn Error>> {
    let SAPRecord::Pet(pet_record) = &item_info.record else {
        return Err("Got a food record. Cannot add item to team.".into());
    };
    // Create pet only if selected team has less than 6 pets.
    let selected_team = cx.props.selected_team.get();
    let team_size = cx
        .props
        .teams
        .with(|teams| teams.get(selected_team).map(|teams| teams.len()));

    if team_size.filter(|size| *size < ALLOWED_TEAM_SIZE).is_some() {
        let pet = Pet::new(pet_record.name.clone(), None, 1)?;

        // Get a mut handle to the selected team pets.
        cx.props.teams.with_mut(|teams| {
            if let Some(selected_team) = teams.get_mut(selected_team) {
                selected_team.push_front((item_info.icon.to_string(), pet))
            }
        })
    }

    Ok(())
}

pub fn remove_pet_from_team(cx: &Scoped<BattleUIState>, pet_idx: usize) {
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

pub fn assign_food_to_pet(
    cx: &Scoped<BattleUIState>,
    pet_idx: usize,
    item_name: Option<&String>,
) -> Result<(), Box<dyn Error>> {
    let selected_team = cx.props.selected_team.get();
    // Convert name to food.
    let food = if let Some(item_name) = item_name {
        Some(Food::try_from(FoodName::from_str(item_name)?)?)
    } else {
        None
    };
    cx.props.teams.with_mut(|teams| {
        let Some(Some((_, selected_pet))) = teams
            .get_mut(selected_team)
            .map(|team| team.get_mut(pet_idx)) else
        {
            return Err("Cannot access pet".into())
        };
        selected_pet.item = food;
        Ok(())
    })
}

pub fn assign_pet_stats<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    pet_stats: Statistics,
) -> Result<(), Box<dyn Error>> {
    let selected_team = cx.props.selected_team.get();
    let Some(selected_pet_idx) = cx.props.selected_pet_idx.get() else {
        return Err("Cannot get selected pet idx.".into());
    };

    cx.props.teams.with_mut(|teams| {
        let Some(Some((_, selected_pet))) = teams
            .get_mut(selected_team)
            .map(|team| team.get_mut(*selected_pet_idx)) else
        {
            return Err("Cannot access pet".into())
        };
        selected_pet.stats = pet_stats;
        Ok(())
    })
}

pub fn assign_pet_level<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    pet_level: usize,
) -> Result<(), Box<dyn Error>> {
    let selected_team = cx.props.selected_team.get();
    let Some(selected_pet_idx) = cx.props.selected_pet_idx.get() else {
        return Err("Cannot get selected pet idx.".into());
    };

    cx.props.teams.with_mut(|teams| {
        let Some(Some((_, selected_pet))) = teams
            .get_mut(selected_team)
            .map(|team| team.get_mut(*selected_pet_idx)) else
        {
            return Err("Cannot access pet".into())
        };
        selected_pet.set_level(pet_level)?;
        Ok(())
    })
}
