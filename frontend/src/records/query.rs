use indexmap::IndexMap;
use itertools::Itertools;
use serde_json::Value;
use std::{collections::VecDeque, error::Error};

use crate::{
    components::battle::{fight::BattleResponse, ui::PetSlots, EMPTY_SLOT_ICON},
    records::{
        food::SimpleFood,
        pet::SimplePet,
        record::SAPSimpleRecord,
        team::{SimpleTeam, Teams},
    },
    BACKEND_API_URL, DEV_BACKEND_API_URL, RECORDS,
};

pub type ItemRecords = IndexMap<String, SAPSimpleRecord>;

pub fn in_saptest_dev() -> bool {
    match std::env::var("SAPTEST_DEV") {
        Ok(val) => val == 1.to_string(),
        Err(_e) => false,
    }
}

pub fn retrieve_record<'a>(rec_type: &'a str, item_name: &'a str) -> Option<&'a SAPSimpleRecord> {
    RECORDS
        .get()
        .and_then(|records| records.get(rec_type))
        .and_then(|items| items.get(item_name))
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

/// Reformat slots so pets in correct order before being sent to battle endpoint.
fn reformat_slots(slots: VecDeque<(String, Option<SimplePet>)>) -> Vec<Option<SimplePet>> {
    slots
        .into_iter()
        // Slots stored in reverse order so front-most pet always on right side visually.
        .rev()
        .map(|mut slot| {
            // Convert item name id with pack in it to basic name.
            // ex. Honey_Turtle -> Honey
            if let Some(item_pack_name) = slot.1.as_mut().and_then(|pet| pet.item.as_mut()) {
                *item_pack_name = retrieve_record("Foods", item_pack_name)
                    .map(|rec| rec.name())
                    .unwrap_or(item_pack_name.to_owned())
            }
            slot.1
        })
        .collect_vec()
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
        teams.remove("Friend").map(reformat_slots),
        teams.remove("Enemy").map(reformat_slots)
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
