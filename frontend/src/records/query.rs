use indexmap::IndexMap;
use itertools::Itertools;
use serde_json::Value;
use std::error::Error;

use crate::{
    components::battle::{fight::BattleResponse, ui::PetSlots, EMPTY_SLOT_ICON},
    records::{
        food::SimpleFood,
        pet::SimplePet,
        record::SAPSimpleRecord,
        team::{SimpleTeam, Teams},
    },
    BACKEND_API_URL, DEV_BACKEND_API_URL,
};

pub type ItemRecords = IndexMap<String, SAPSimpleRecord>;

pub fn in_saptest_dev() -> bool {
    match std::env::var("SAPTEST_DEV") {
        Ok(val) => val == 1.to_string(),
        Err(_e) => false,
    }
}

pub async fn get_all_sap_records() -> Result<IndexMap<String, ItemRecords>, Box<dyn Error>> {
    let mut item_img_urls: IndexMap<String, ItemRecords> = IndexMap::new();
    let mut pets = get_sap_records("pets").await?;
    // Add empty slot.
    let empty_slot = SAPSimpleRecord::Pet(SimplePet {
        name: "Slot".to_owned(),
        tier: 0,
        level: Some(1),
        img_url: EMPTY_SLOT_ICON.to_owned(),
        pack: String::from("Unknown"),
        ..Default::default()
    });
    pets.insert("Slot".to_owned(), empty_slot);

    let foods = get_sap_records("foods").await?;

    item_img_urls.insert("Pets".to_string(), pets);
    item_img_urls.insert("Foods".to_string(), foods);
    Ok(item_img_urls)
}

pub async fn post_battle(
    mut teams: IndexMap<String, PetSlots>,
) -> Result<BattleResponse, Box<dyn Error>> {
    let api_url = if in_saptest_dev() {
        DEV_BACKEND_API_URL
    } else {
        BACKEND_API_URL
    };
    let (Some(friends), Some(enemies)) = (
        teams.remove("Friend").map(|slots| slots.into_iter().rev().map(|slot| slot.1).collect_vec()),
        teams.remove("Enemy").map(|slots| slots.into_iter().rev().map(|slot| slot.1).collect_vec())
    ) else {
        return Err("Missing a team.".into())
    };

    let teams = Teams {
        friend_team: SimpleTeam {
            name: "Friend".into(),
            pets: friends,
        },
        enemy_team: SimpleTeam {
            name: "Enemy".into(),
            pets: enemies,
        },
    };

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{api_url}/battle"))
        .json(&teams)
        .send()
        .await?
        .text()
        .await?;

    serde_json::from_str(&res).map_err(Into::into)
}

pub async fn get_sap_records(categ: &str) -> Result<ItemRecords, Box<dyn Error>> {
    let api_url = if in_saptest_dev() {
        DEV_BACKEND_API_URL
    } else {
        BACKEND_API_URL
    };
    let url = format!("{api_url}/db/{categ}");

    let resp_text = reqwest::get(url).await?.text().await?;
    let pet_records: Value = serde_json::from_str(&resp_text)?;

    if let Some(records) = pet_records.as_array() {
        Ok(records
            .iter()
            .filter_map(|rec| {
                // Suffix name with level to avoid overriding hashed records.
                let Some(name) = rec.get("name").and_then(|name| name.as_str()) else {
                    return None
                };
                let Some(pack) = rec.get("pack").and_then(|pack| pack.as_str()) else {
                    return None
                };

                // If has level is pet record, otherwise is food record.
                let (item_name, converted_record) =
                    if let Some(lvl) = rec.get("lvl").and_then(|lvl| lvl.as_u64()) {
                        let pet_record = SimplePet::try_from(rec).map(SAPSimpleRecord::Pet).ok();
                        (format!("{name}_{pack}_{lvl}"), pet_record)
                    } else {
                        let food_record = SimpleFood::try_from(rec).map(SAPSimpleRecord::Food).ok();
                        (format!("{name}_{pack}"), food_record)
                    };

                converted_record.map(|valid_record| (item_name, valid_record))
            })
            .collect())
    } else {
        Err("No records".into())
    }
}
