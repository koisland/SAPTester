use indexmap::IndexMap;
use serde_json::Value;
use std::error::Error;

pub type ItemRecords = IndexMap<String, Value>;

pub const BACKEND_API_URL: &str = "http://127.0.0.1:3030/db";

// pub const ATTACK_ICON: &str =
//     "https://static.wikia.nocookie.net/superautopets/images/a/aa/Attack_Icon.png";
// pub const HEALTH_ICON: &str =
//     "https://static.wikia.nocookie.net/superautopets/images/4/44/Health_Icon.png";

pub async fn get_all_sap_records() -> Result<IndexMap<String, ItemRecords>, Box<dyn Error>> {
    let mut item_img_urls: IndexMap<String, ItemRecords> = IndexMap::new();
    let pets = get_sap_records("pets").await?;
    let foods = get_sap_records("foods").await?;

    item_img_urls.insert("Pets".to_string(), pets);
    item_img_urls.insert("Foods".to_string(), foods);
    Ok(item_img_urls)
}

pub async fn get_sap_records(categ: &str) -> Result<ItemRecords, Box<dyn Error>> {
    let url = format!("{BACKEND_API_URL}/{categ}");

    let resp_text = reqwest::get(url).await?.text().await?;
    let pet_records: Value = serde_json::from_str(&resp_text)?;

    if let Some(records) = pet_records.as_array() {
        Ok(records
            .iter()
            .filter_map(|rec| {
                rec.get("name")
                    .and_then(|name| name.as_str())
                    .map(|name| (name.to_owned(), rec.to_owned()))
            })
            .collect())
    } else {
        Err("No records".into())
    }
}
