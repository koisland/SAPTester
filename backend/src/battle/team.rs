use itertools::Itertools;
use saptest::{
    error::SAPTestError,
    pets::pet::{MAX_PET_LEVEL, MAX_PET_STATS, MIN_PET_LEVEL, MIN_PET_STATS},
    Food, FoodName, Pet, PetName, Team,
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
pub struct SimpleTeam {
    name: String,
    pets: Vec<Option<SimplePet>>,
}

#[derive(Deserialize, Default, Clone)]
pub struct SimplePet {
    pub name: String,
    pub attack: Option<usize>,
    pub health: Option<usize>,
    pub level: Option<usize>,
    pub item: Option<String>,
}

impl TryFrom<SimplePet> for Pet {
    type Error = SAPTestError;

    fn try_from(simple_pet: SimplePet) -> Result<Self, Self::Error> {
        let item = simple_pet
            .item
            .and_then(|item_name| FoodName::from_str(&item_name).ok())
            .and_then(|item_name| Food::try_from(item_name).ok());

        let pet_lvl = simple_pet
            .level
            .map_or(1, |lvl| lvl.clamp(MIN_PET_LEVEL, MAX_PET_LEVEL));

        PetName::from_str(&simple_pet.name)
            .and_then(|pet_name| Pet::new(pet_name, None, pet_lvl))
            .map(|mut pet| {
                //  Assign item.
                pet.item = item;
                // Assign stats if given.
                if let Some(Ok(attack)) = simple_pet.attack.map(TryInto::<isize>::try_into) {
                    pet.stats.attack = attack.clamp(MIN_PET_STATS, MAX_PET_STATS)
                }
                if let Some(Ok(health)) = simple_pet.health.map(TryInto::<isize>::try_into) {
                    pet.stats.health = health.clamp(MIN_PET_STATS, MAX_PET_STATS)
                }
                pet
            })
    }
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

#[cfg(test)]
mod tests {
    use saptest::TeamViewer;

    use super::*;

    #[test]
    fn test_build_def_pet() {
        let def_ant = SimplePet {
            name: "Ant".to_owned(),
            ..Default::default()
        };

        let ant: Pet = def_ant.try_into().unwrap();

        assert!(
            ant.name == PetName::Ant
                && ant.get_level() == 1
                && ant.stats.attack == 2
                && ant.stats.health == 1
                && ant.item == None
        )
    }

    #[test]
    fn test_deserialize_pet() {
        let pet_only_name = r#"{"name": "Ant"}"#;
        let pet_w_stats = r#"{"name": "Ant", "attack": 1, "health": 2}"#;
        let pet_leveled = r#"{"name": "Ant", "level": 2}"#;
        let pet_w_food = r#"{"name": "Ant", "item": "Grapes"}"#;

        let pet_only_name: SimplePet = serde_json::from_str(pet_only_name).unwrap();
        let pet_w_stats: SimplePet = serde_json::from_str(pet_w_stats).unwrap();
        let pet_leveled: SimplePet = serde_json::from_str(pet_leveled).unwrap();
        let pet_w_food: SimplePet = serde_json::from_str(pet_w_food).unwrap();

        assert_eq!(pet_only_name.name, "Ant".to_owned());
        assert!(pet_w_stats.attack == Some(1) && pet_w_stats.health == Some(2));
        assert_eq!(pet_leveled.level, Some(2));
        assert_eq!(pet_w_food.item, Some("Grapes".to_owned()));
    }

    #[test]
    fn test_build_team() {
        let simple_pet = SimplePet {
            name: "Ant".to_owned(),
            ..Default::default()
        };
        let simple_pets = vec![
            Some(simple_pet.clone()),
            None,
            Some(simple_pet.clone()),
            Some(simple_pet),
        ];
        let simple_team = SimpleTeam {
            name: "The Super Auto Pets".to_owned(),
            pets: simple_pets,
        };
        let team: Team = simple_team.try_into().unwrap();

        assert!(
            // Name set.
            team.get_name() == "The Super Auto Pets" &&
            team.all().len() == 3 &&
            // Order preserved.
            team.nth(1).is_none()
        )
    }
}
