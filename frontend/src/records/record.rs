use dioxus::prelude::Scope;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

use crate::components::battle::ui::BattleUIState;

use super::{effect::SimpleEffect, food::SimpleFood, pet::SimplePet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SAPSimpleRecord {
    Pet(SimplePet),
    Food(SimpleFood),
}

impl SAPSimpleRecord {
    pub fn is_valid_item<'a>(&self, cx: Scope<'a, BattleUIState<'a>>) -> bool {
        cx.props.filters.with(|filters| {
            filters
                .iter()
                .map(|(filter_name, filter_val)| {
                    if *filter_name == "Name" {
                        let name = self.name().to_lowercase();
                        // If empty slot, allow.
                        if name == "slot" {
                            return Ok(true);
                        };

                        let filter_val = filter_val.to_lowercase();
                        Ok(if filter_val.is_empty() {
                            true
                        } else {
                            name.contains(&filter_val)
                        })
                    } else if *filter_name == "Tier" {
                        let tier = filter_val.parse::<u64>()?;
                        Ok(self.tier() == tier)
                    } else {
                        Ok(&self.pack() == filter_val || filter_val == "All")
                    }
                })
                .all(|value: Result<bool, Box<dyn Error>>| value.unwrap_or(false))
        })
    }

    pub fn name(&self) -> String {
        match self {
            SAPSimpleRecord::Pet(rec) => rec.name.to_owned(),
            SAPSimpleRecord::Food(rec) => rec.name.to_owned(),
        }
    }

    pub fn level(&self) -> Option<u64> {
        match self {
            SAPSimpleRecord::Pet(rec) => rec.level,
            SAPSimpleRecord::Food(_) => None,
        }
    }

    pub fn pack(&self) -> String {
        match self {
            SAPSimpleRecord::Pet(rec) => rec.pack.to_owned(),
            SAPSimpleRecord::Food(rec) => rec.pack.to_owned(),
        }
    }

    pub fn holdable(&self) -> bool {
        match self {
            SAPSimpleRecord::Pet(_) => false,
            SAPSimpleRecord::Food(rec) => rec.holdable,
        }
    }
    pub fn tier(&self) -> u64 {
        match self {
            SAPSimpleRecord::Pet(rec) => rec.tier,
            SAPSimpleRecord::Food(rec) => rec.tier,
        }
    }

    pub fn img_url(&self) -> String {
        match self {
            SAPSimpleRecord::Pet(rec) => rec.img_url.to_owned(),
            SAPSimpleRecord::Food(rec) => rec.img_url.to_owned(),
        }
    }

    pub fn effect(&self) -> Option<SimpleEffect> {
        match self {
            SAPSimpleRecord::Pet(rec) => rec.effect.clone(),
            SAPSimpleRecord::Food(rec) => Some(rec.effect.clone()),
        }
    }

    pub fn attack(&self) -> Option<u64> {
        match self {
            SAPSimpleRecord::Pet(rec) => rec.attack,
            SAPSimpleRecord::Food(_) => None,
        }
    }

    pub fn health(&self) -> Option<u64> {
        match self {
            SAPSimpleRecord::Pet(rec) => rec.health,
            SAPSimpleRecord::Food(_) => None,
        }
    }
}

impl TryFrom<&Value> for SAPSimpleRecord {
    type Error = Box<dyn Error>;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Ok(pet) = SimplePet::try_from(value) {
            Ok(SAPSimpleRecord::Pet(pet))
        } else if let Ok(food) = SimpleFood::try_from(value) {
            Ok(SAPSimpleRecord::Food(food))
        } else {
            Err("Not a valid pet or food record.".into())
        }
    }
}
