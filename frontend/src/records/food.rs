use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

use super::effect::SimpleEffect;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleFood {
    pub name: String,
    pub tier: u64,
    pub pack: String,
    pub holdable: bool,
    pub img_url: String,
    pub effect: SimpleEffect,
}

impl TryFrom<&Value> for SimpleFood {
    type Error = Box<dyn Error>;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let name = value
            .get("name")
            .and_then(|name| name.as_str())
            .map(|name| name.to_string());
        let img_url = value
            .get("img_url")
            .and_then(|url| url.as_str())
            .map(|url| url.to_string());
        let tier = value.get("tier").and_then(|tier| tier.as_u64());
        let holdable = value
            .get("holdable")
            .and_then(|holdable| holdable.as_bool())
            .unwrap_or(false);
        let effect_text = value
            .get("effect")
            .and_then(|effect| effect.as_str())
            .map(|effect_str| effect_str.to_string());
        let pack = value
            .get("pack")
            .and_then(|pack| pack.as_str())
            .unwrap_or("Unknown");

        let n_uses = value
            .get("single_use")
            .and_then(|is_single_use| is_single_use.as_bool())
            .map(|is_single_use| if is_single_use { 1 } else { 0 });

        let (Some(name), Some(img_url), Some(effect), Some(uses), Some(tier)) = (name, img_url, effect_text, n_uses, tier) else {
            return Err("No effect or number of uses.".into())
        };

        // Records do not contain effect trigger. Would need to deserialize entire food to get effect triggers. Not necessary.
        Ok(SimpleFood {
            name,
            tier,
            img_url,
            holdable,
            pack: pack.to_owned(),
            effect: SimpleEffect {
                text: effect,
                trigger: "None".to_owned(),
                uses,
            },
        })
    }
}
