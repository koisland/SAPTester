use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

use crate::{
    components::battle::{ui::BattleUIState, ALLOWED_TEAM_SIZE},
    RECORDS,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleEffect {
    pub text: String,
    pub trigger: String,
    pub uses: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleFood {
    pub name: String,
    pub effect: SimpleEffect,
}

impl TryFrom<&Value> for SimpleFood {
    type Error = Box<dyn Error>;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let name = value
            .get("name")
            .and_then(|name| name.as_str())
            .map(|name| name.to_string());

        let effect_text = value
            .get("effect")
            .and_then(|effect| effect.as_str())
            .map(|effect_str| effect_str.to_string());

        let n_uses = value
            .get("single_use")
            .and_then(|is_single_use| is_single_use.as_bool())
            .map(|is_single_use| if is_single_use { 1 } else { 0 });

        let (Some(name), Some(effect), Some(uses)) = (name, effect_text, n_uses) else {
            return Err("No effect or number of uses.".into())
        };

        // Records do not contain effect trigger. Would need to deserialize entire food to get effect triggers. Not necessary.
        Ok(SimpleFood {
            name,
            effect: SimpleEffect {
                text: effect,
                trigger: "None".to_owned(),
                uses,
            },
        })
    }
}

impl TryFrom<&Value> for SimpleEffect {
    type Error = Box<dyn Error>;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let effect_text = value
            .get("effect")
            .and_then(|effect| effect.as_str())
            .map(|effect_str| effect_str.to_string());
        let effect_trigger = value
            .get("effect_trigger")
            .and_then(|effect_trigger| effect_trigger.as_str())
            .map(|effect_trigger_str| effect_trigger_str.to_string());
        let num_triggers = value
            .get("n_triggers")
            .and_then(|n_triggers| n_triggers.as_u64());
        let (Some(effect), Some(trigger), Some(num_triggers)) = (effect_text, effect_trigger, num_triggers) else {
            return Err("Missing an effect field.".into())
        };

        Ok(SimpleEffect {
            text: effect,
            trigger,
            uses: num_triggers,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PetProperty {
    Attack(Option<usize>),
    Health(Option<usize>),
    Effect(Option<SimpleEffect>),
    Food(Option<String>),
    Level(usize),
}

#[derive(Serialize, Deserialize)]
pub struct Teams {
    pub friend_team: SimpleTeam,
    pub enemy_team: SimpleTeam,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleTeam {
    name: String,
    pets: Vec<Option<SimplePet>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimplePet {
    pub name: String,
    pub attack: Option<usize>,
    pub health: Option<usize>,
    pub level: Option<usize>,
    pub item: Option<String>,
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
                        .map(|rec| PetProperty::Effect(rec.try_into().ok())),
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
    item_info: &Value,
) -> Result<(), Box<dyn Error>> {
    let (Some(name), Some(img_url), Some(atk), Some(health), Some(lvl)) = (
        item_info.get("name").and_then(|name| name.as_str()),
        item_info.get("img_url").and_then(|img_url| img_url.as_str()),
        item_info.get("attack").and_then(|img_url| img_url.as_u64()),
        item_info.get("health").and_then(|img_url| img_url.as_u64()),
        item_info.get("lvl").and_then(|img_url| img_url.as_u64()),

    ) else {
        return Err("Got a food record. Cannot add item to team.".into());
    };
    // Create pet only if selected team has less than 6 pets.
    let selected_team = cx.props.selected_team.get();
    let team_size = cx
        .props
        .teams
        .with(|teams| teams.get(selected_team).map(|teams| teams.len()));

    if team_size.filter(|size| *size < ALLOWED_TEAM_SIZE).is_some() {
        let slot = if name == "Slot" {
            None
        } else {
            Some(SimplePet {
                name: name.to_owned(),
                attack: atk.try_into().ok(),
                health: health.try_into().ok(),
                level: lvl.try_into().ok(),
                item: None,
            })
        };

        // Get a mut handle to the selected team pets.
        cx.props.teams.with_mut(|teams| {
            if let Some(selected_team) = teams.get_mut(selected_team) {
                selected_team.push_front((img_url.to_string(), slot))
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
