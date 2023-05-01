use dioxus::prelude::Scope;
use indexmap::IndexMap;
use regex::Regex;
use serde_json::Value;
use std::{error::Error, str::FromStr};
use ureq;

use saptest::{
    db::{pack::Pack, record::SAPRecord},
    pets::pet::MIN_PET_LEVEL,
    SAPDB,
};

use crate::components::battle::ui::BattleUIState;
pub type ItemImgUrls = IndexMap<String, SAPItem>;

lazy_static::lazy_static!(
    static ref NAME_EXCEPTIONS: IndexMap<String, String> = IndexMap::from(
        [
            ("Popcorns".to_owned(), "Popcorn".to_owned()),
            ("Hammershark".to_owned(), "Hammerhead_Shark".to_owned())
        ]
    );
);

const BASE_FANDOM_FILE_URL: &str = "https://superautopets.fandom.com/wiki/File:";
const RGX_MDATA_URL: &str = r"https://static.wikia.nocookie.net/superautopets/images/\w{1}/\w{2}/";
pub const ATTACK_ICON: &str =
    "https://static.wikia.nocookie.net/superautopets/images/a/aa/Attack_Icon.png";
pub const HEALTH_ICON: &str =
    "https://static.wikia.nocookie.net/superautopets/images/4/44/Health_Icon.png";

#[derive(Debug, Clone)]
pub struct SAPItem {
    pub record: SAPRecord,
    pub icon: String,
}

#[allow(dead_code)]
impl SAPItem {
    pub fn get_name(&self) -> String {
        match &self.record {
            SAPRecord::Food(food_rec) => food_rec.name.to_string(),
            SAPRecord::Pet(pet_rec) => pet_rec.name.to_string(),
        }
    }
    pub fn is_name(&self, name: &str) -> bool {
        self.get_name() == name
    }

    pub fn get_effect(&self) -> String {
        match &self.record {
            SAPRecord::Food(food_rec) => food_rec.effect.to_owned(),
            SAPRecord::Pet(pet_rec) => pet_rec.effect.clone().unwrap_or("None".to_owned()),
        }
    }
    pub fn get_pack(&self) -> Pack {
        match &self.record {
            SAPRecord::Food(food_rec) => food_rec.pack.clone(),
            SAPRecord::Pet(pet_rec) => pet_rec.pack.clone(),
        }
    }

    pub fn is_pack(&self, pack: &str) -> Result<bool, Box<dyn Error>> {
        Ok(self.get_pack() == Pack::from_str(pack)?)
    }

    pub fn is_holdable(&self) -> bool {
        match &self.record {
            SAPRecord::Food(food_rec) => food_rec.holdable,
            SAPRecord::Pet(_) => false,
        }
    }

    pub fn get_tier(&self) -> usize {
        match &self.record {
            SAPRecord::Food(food_rec) => food_rec.tier,
            SAPRecord::Pet(pet_rec) => pet_rec.tier,
        }
    }

    pub fn is_tier(&self, tier: usize) -> bool {
        self.get_tier() == tier
    }

    pub fn is_valid_item<'a>(&self, cx: Scope<'a, BattleUIState<'a>>) -> bool {
        cx.props.filters.with(|filters| {
            filters
                .iter()
                .map(|(filter_name, filter_val)| {
                    if *filter_name == "Name" {
                        let name = self.get_name().to_lowercase();
                        // If empty slot, allow.
                        if name == "Slot" {
                            return Ok(true);
                        };

                        let filter_val = filter_val.to_lowercase();
                        Ok(if filter_val.is_empty() {
                            true
                        } else {
                            name.contains(&filter_val)
                        })
                    } else if *filter_name == "Tier" {
                        let tier = filter_val.parse::<usize>()?;
                        Ok(self.is_tier(tier))
                    } else {
                        self.is_pack(filter_val)
                            .map_err(Into::<Box<dyn Error>>::into)
                    }
                })
                .all(|value| value.unwrap_or(false))
        })
    }
}

impl std::hash::Hash for SAPItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.icon.hash(state);
    }
}

pub fn extract_img_url(record: SAPRecord) -> Result<(String, SAPItem), Box<dyn Error>> {
    let rec_name = match &record {
        SAPRecord::Food(food_rec) => food_rec.name.to_string(),
        SAPRecord::Pet(pet_rec) => pet_rec.name.to_string(),
    };

    let item_name = if NAME_EXCEPTIONS.contains_key(&rec_name) {
        NAME_EXCEPTIONS.get(&rec_name).unwrap().replace(' ', "_")
    } else {
        rec_name.replace(' ', "_")
    };

    let fd_item_filename = format!("{item_name}_Icon.png");
    let mdata_url = format!("{BASE_FANDOM_FILE_URL}{fd_item_filename}");

    // Fairly inefficient as we're literally parsing an entire page for two numbers preceding the name of the item icon.
    // https://static.wikia.nocookie.net/superautopets/images/?/?/item.png
    let res_mdata_page = ureq::get(&mdata_url).call()?.into_string()?;

    let rgx_img_url = format!("{RGX_MDATA_URL}{fd_item_filename}/revision/latest");

    let re = Regex::new(&rgx_img_url)?;

    if let Some(mtch) = re.captures(&res_mdata_page).and_then(|cap| cap.get(0)) {
        Ok((
            rec_name,
            SAPItem {
                record,
                icon: mtch.as_str().to_owned(),
            },
        ))
    } else {
        Err("No image".into())
    }
}

fn extract_existing_urls(path: Option<&str>, entity: saptest::Entity) -> IndexMap<String, SAPItem> {
    let levels = vec![MIN_PET_LEVEL.to_string()];
    let mut params = match entity {
        saptest::Entity::Pet => vec![("lvl", &levels)],
        saptest::Entity::Food => vec![],
    };
    // Find the file path for an existing json file with the {item_name: item_icon_url}
    if let Ok(items_str) = std::fs::read_to_string(path.unwrap_or("")) {
        let mut items: IndexMap<String, String> = IndexMap::new();
        let items_json: Value = serde_json::from_str(&items_str).unwrap();
        // Unpack value and insert in order.
        if let Value::Object(map) = items_json {
            for (item_name, item_img_url) in map.into_iter() {
                items.insert(item_name, item_img_url.as_str().unwrap_or("").to_owned());
            }
        };

        let items_list: Vec<String> = items.keys().cloned().collect();
        params.push(("name", &items_list));
        // Extract the records.
        let records = SAPDB.execute_query(entity, &params).unwrap();
        records
            .into_iter()
            .map(|record| {
                let name = match &record {
                    SAPRecord::Food(food_rec) => food_rec.name.to_string(),
                    SAPRecord::Pet(pet_rec) => pet_rec.name.to_string(),
                };
                // And construct a SAPItem storing the record and its icon.
                let icon_url = items.get(&name).unwrap().to_string();
                (
                    name,
                    SAPItem {
                        record,
                        icon: icon_url,
                    },
                )
            })
            .collect()
    } else {
        // Query the entity and extract image urls from the fandom wiki.
        let records = SAPDB.execute_query(entity, &params).unwrap();
        records.into_iter().flat_map(extract_img_url).collect()
    }
}

pub fn extract_sap_image_urls() -> IndexMap<String, ItemImgUrls> {
    let foods = extract_existing_urls(Some("data/foods.json"), saptest::Entity::Food);
    let pets = extract_existing_urls(Some("data/pets.json"), saptest::Entity::Pet);

    let mut all_res = IndexMap::new();
    all_res.insert("Pets".to_owned(), pets);
    all_res.insert("Foods".to_owned(), foods);
    // dbg!(&all_res);
    all_res
}
