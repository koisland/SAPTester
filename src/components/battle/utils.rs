use std::{error::Error, str::FromStr};

use dioxus::prelude::*;
use saptest::{db::record::SAPRecord, Food, FoodName, Pet, Statistics};

use crate::{components::battle::ui::BattleUIState, utils::extract_urls::SAPItem};

use super::ALLOWED_TEAM_SIZE;

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
    Food,
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
            PetProperty::Food => write!(f, "Food"),
        }
    }
}

pub fn get_selected_pet<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Option<&'a Pet> {
    if let Some(Some(Some((_, selected_pet)))) = cx.props.selected_pet_idx.get().map(|pet_idx| {
        cx.props
            .teams
            .get()
            .get(cx.props.selected_team.get())
            .map(|team| team.get(pet_idx))
    }) {
        Some(selected_pet)
    } else {
        None
    }
}

pub fn swap_pet_on_team(
    cx: &Scoped<BattleUIState>,
    from: usize,
    to: usize,
) -> Result<(), Box<dyn Error>> {
    if from != to {
        let selected_team = cx.props.selected_team.get();
        let mut teams = cx.props.teams.make_mut();
        if let Some(selected_team) = teams.get_mut(selected_team) {
            selected_team.swap(from, to);
            // Keep swapped pet as selected pet.
            cx.props.selected_pet_idx.set(Some(to));
        };
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
    let mut teams = cx.props.teams.make_mut();
    if let Some(selected_team) = teams.get_mut(selected_team) {
        selected_team.push_front((item_info.icon.to_string(), pet))
    };

    Ok(())
}

pub fn remove_pet_from_team(cx: &Scoped<BattleUIState>, pet_idx: usize) {
    let mut teams = cx.props.teams.make_mut();
    if let Some(selected_team_pets) = teams.get_mut(cx.props.selected_team.get()) {
        // Remove pet from pets.
        // Should never panic.
        selected_team_pets.remove(pet_idx);

        // Reset selected pet.
        cx.props.selected_pet_idx.set(None)
    }
}

pub fn assign_food_to_pet(
    cx: &Scoped<BattleUIState>,
    pet_idx: usize,
    item_name: Option<&String>,
) -> Result<(), Box<dyn Error>> {
    // Convert name to food.
    let food = if let Some(item_name) = item_name {
        Some(Food::try_from(FoodName::from_str(item_name)?)?)
    } else {
        None
    };
    let mut teams = cx.props.teams.make_mut();
    if let Some((_, pet)) = teams
        .get_mut(cx.props.selected_team.get())
        .and_then(|team| team.get_mut(pet_idx))
    {
        // Assign pet food or remove food.
        pet.item = food;
    }

    Ok(())
}

pub fn assign_pet_stats<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    pet_stats: Statistics,
) -> Result<(), Box<dyn Error>> {
    let selected_team = cx.props.selected_team.get();
    let Some(selected_pet_idx) = cx.props.selected_pet_idx.get() else {
        return Err("Cannot get selected pet idx.".into());
    };

    let mut teams = cx.props.teams.make_mut();
    if let Some(Some((_, selected_pet))) = teams
        .get_mut(selected_team)
        .map(|team| team.get_mut(*selected_pet_idx))
    {
        selected_pet.stats = pet_stats
    }
    Ok(())
}

pub fn assign_pet_level<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    pet_level: usize,
) -> Result<(), Box<dyn Error>> {
    let selected_team = cx.props.selected_team.get();
    let Some(selected_pet_idx) = cx.props.selected_pet_idx.get() else {
        return Err("Cannot get selected pet idx.".into());
    };

    let mut teams = cx.props.teams.make_mut();
    if let Some(Some((_, selected_pet))) = teams
        .get_mut(selected_team)
        .map(|team| team.get_mut(*selected_pet_idx))
    {
        selected_pet.set_level(pet_level)?;
    }
    Ok(())
}
