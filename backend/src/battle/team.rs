use itertools::Itertools;
use saptest::{
    error::SAPTestError,
    pets::pet::{MAX_PET_LEVEL, MAX_PET_STATS, MIN_PET_LEVEL, MIN_PET_STATS},
    Food, FoodName, Pet, PetName, Statistics, Team,
};
use serde::Deserialize;
use std::str::FromStr;

use super::TEAM_SIZE;

#[derive(Deserialize)]
pub struct Teams {
    pub friend_team: SimpleTeam,
    pub enemy_team: SimpleTeam,
}

#[derive(Deserialize)]
pub struct SimplePet {
    pub name: String,
    pub attack: usize,
    pub health: usize,
    pub level: usize,
    pub item: Option<String>,
}

impl TryFrom<SimplePet> for Pet {
    type Error = SAPTestError;

    fn try_from(simple_pet: SimplePet) -> Result<Self, Self::Error> {
        let (attack, health) = (
            simple_pet.attack.try_into().unwrap_or_default(),
            simple_pet.health.try_into().unwrap_or_default(),
        );
        let mut stats = Statistics { attack, health };
        // Clamp stats to 0 to 50.
        stats.clamp(MIN_PET_STATS, MAX_PET_STATS);

        let item = simple_pet
            .item
            .and_then(|item_name| FoodName::from_str(&item_name).ok())
            .and_then(|item_name| Food::try_from(item_name).ok());

        let pet_lvl = simple_pet.level.clamp(MIN_PET_LEVEL, MAX_PET_LEVEL);
        PetName::from_str(&simple_pet.name)
            .and_then(|pet_name| Pet::new(pet_name, Some(stats), pet_lvl))
            .map(|mut pet| {
                pet.item = item;
                pet
            })
    }
}

#[derive(Deserialize)]
pub struct SimpleTeam {
    name: String,
    pets: Vec<Option<SimplePet>>,
}

impl TryFrom<SimpleTeam> for Team {
    type Error = SAPTestError;

    fn try_from(simple_team: SimpleTeam) -> Result<Self, Self::Error> {
        let pets = simple_team
            .pets
            .into_iter()
            // If pet in slot, generate pet. Otherwise, treat as empty slot.
            .map(|slot| slot.and_then(|pet| pet.try_into().ok()))
            .collect_vec();

        Team::new(&pets, TEAM_SIZE).map(|mut team| {
            let _ = team.set_name(&simple_team.name);
            team
        })
    }
}
