use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Teams {
    pub friend_team: SimpleTeam,
    pub enemy_team: SimpleTeam,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleTeam {
    name: String,
    pets: Vec<Option<SimplePet>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimplePet {
    pub name: String,
    pub attack: Option<usize>,
    pub health: Option<usize>,
    pub level: Option<usize>,
    pub item: Option<String>,
}
