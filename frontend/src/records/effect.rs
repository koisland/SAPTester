use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleEffect {
    pub text: String,
    pub trigger: String,
    pub uses: u64,
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
