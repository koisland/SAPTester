use dioxus::prelude::*;
use std::error::Error;

use crate::{
    components::battle::{ui::BattleUIState, ALLOWED_TEAM_SIZE},
    records::{pet::PetProperty, record::SAPSimpleRecord},
    RECORDS,
};

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
                let Some(pet) = pet else {
                    return None
                };
                let pet_name_lvl = format!("{}_{}", pet.name, pet.level.unwrap_or(1));

                match property_name {
                    "Attack" => Some(PetProperty::Attack(pet.attack)),
                    "Health" => Some(PetProperty::Health(pet.health)),
                    "Effect" => RECORDS.get()
                        .and_then(|rec| rec.get("Pets"))
                        .and_then(|pets| pets.get(&pet_name_lvl))
                        .map(|rec| PetProperty::Effect(rec.effect())),
                    "Food" => Some(PetProperty::Food(pet.item.clone())),
                    "Level" => Some(PetProperty::Level(pet.level.unwrap_or(1))),
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
    item_info: &SAPSimpleRecord,
) -> Result<(), Box<dyn Error>> {
    let SAPSimpleRecord::Pet(pet) = item_info else {
        return Err("Got a food record. Cannot add item to team.".into());
    };
    // Create pet only if selected team has less than 6 pets.
    let selected_team = cx.props.selected_team.get();
    let team_size = cx
        .props
        .teams
        .with(|teams| teams.get(selected_team).map(|teams| teams.len()));

    if team_size.filter(|size| *size < ALLOWED_TEAM_SIZE).is_some() {
        // Add empty space if pet name is 'Slot'.
        let slot = if pet.name == "Slot" {
            None
        } else {
            Some(pet.clone())
        };

        // Get a mut handle to the selected team pets.
        cx.props.teams.with_mut(|teams| {
            if let Some(selected_team) = teams.get_mut(selected_team) {
                selected_team.push_front((pet.img_url.clone(), slot))
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

pub fn assign_pet_property<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    selected_pet_idx: usize,
    property: PetProperty,
) -> Result<(), Box<dyn Error>> {
    let selected_team = cx.props.selected_team.get();

    cx.props.teams.with_mut(|teams| {
        let Some((_, Some(selected_pet))) = teams
            .get_mut(selected_team)
            .and_then(|team| team.get_mut(selected_pet_idx))
            else
        {
            return Err("Cannot access pet".into())
        };
        match property {
            PetProperty::Attack(atk) => selected_pet.attack = atk,
            PetProperty::Health(health) => selected_pet.health = health,
            PetProperty::Food(food) => selected_pet.item = food,
            PetProperty::Level(lvl) => selected_pet.level = Some(lvl),
            _ => return Err("Cannot assign value to that property".into()),
        }
        Ok(())
    })
}
