use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

use crate::records::effect::SimpleEffect;

#[derive(Debug, Clone, PartialEq)]
pub enum PetProperty {
    Attack(Option<u64>),
    Health(Option<u64>),
    Effect(Option<SimpleEffect>),
    Food(Option<String>),
    Level(u64),
}

/// A simplified PetRecord for [`saptest`].
/// We want to avoid importing this struct from our library as will cause panic in wasm because of rusqlite.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SimplePet {
    pub name: String,
    pub attack: Option<u64>,
    pub health: Option<u64>,
    pub level: Option<u64>,
    pub item: Option<String>,
    #[serde(skip_serializing)]
    pub tier: u64,
    #[serde(skip_serializing)]
    pub img_url: String,
    #[serde(skip_serializing)]
    pub effect: Option<SimpleEffect>,
    #[serde(skip_serializing)]
    pub pack: String,
}

impl TryFrom<&Value> for SimplePet {
    type Error = Box<dyn Error>;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let effect = SimpleEffect::try_from(value)?;

        let name = value.get("name").and_then(|name| name.as_str());
        let tier = value.get("tier").and_then(|tier| tier.as_u64());
        let img_url = value
            .get("img_url")
            .and_then(|url| url.as_str())
            .map(|url| url.to_string());

        let attack = value.get("attack").and_then(|attack| attack.as_u64());
        let level = value.get("lvl").and_then(|level| level.as_u64());
        let health = value.get("health").and_then(|health| health.as_u64());
        let pack = value
            .get("pack")
            .and_then(|pack| pack.as_str())
            .unwrap_or("Unknown");

        let (Some(pet_name), Some(img_url), Some(tier))= (name, img_url, tier) else {
            return Err("Value missing name, url, or tier field.".into())
        };

        Ok(SimplePet {
            name: pet_name.to_owned(),
            img_url,
            attack,
            health,
            level,
            effect: Some(effect),
            item: None,
            tier,
            pack: pack.to_owned(),
        })
    }
}
