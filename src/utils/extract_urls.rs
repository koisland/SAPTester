use std::{collections::HashMap, error::Error};

use regex::Regex;
use ureq;

use saptest::{db::record::SAPRecord, pets::pet::MIN_PET_LEVEL, SAPDB};
pub type ItemImgUrls = HashMap<String, SAPItem>;

lazy_static::lazy_static!(
    static ref NAME_EXCEPTIONS: HashMap<String, String> = HashMap::from(
        [
            ("Popcorns".to_owned(), "Popcorn".to_owned()),
            ("Hammershark".to_owned(), "Hammerhead_Shark".to_owned())
        ]
    );
);

const BASE_FANDOM_FILE_URL: &str = "https://superautopets.fandom.com/wiki/File:";
const RGX_MDATA_URL: &str = r"https://static.wikia.nocookie.net/superautopets/images/\w{1}/\w{2}/";

#[derive(Debug, Clone)]
pub struct SAPItem {
    pub record: SAPRecord,
    pub icon: String,
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

fn extract_existing_urls(path: Option<&str>, entity: saptest::Entity) -> HashMap<String, SAPItem> {
    let levels = vec![MIN_PET_LEVEL.to_string()];
    let mut params = match entity {
        saptest::Entity::Pet => vec![("lvl", &levels)],
        saptest::Entity::Food => vec![],
    };
    // Find the file path for an existing json file with the {item_name: item_icon_url}
    if let Ok(items_str) = std::fs::read_to_string(path.unwrap_or("")) {
        let items_json: HashMap<String, String> = serde_json::from_str(&items_str).unwrap();
        let items_list: Vec<String> = items_json.keys().cloned().collect();
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
                let icon_url = items_json.get(&name).unwrap().to_string();
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

pub fn extract_sap_image_urls() -> HashMap<String, ItemImgUrls> {
    let foods = extract_existing_urls(Some("data/foods.json"), saptest::Entity::Food);
    let pets = extract_existing_urls(Some("data/pets.json"), saptest::Entity::Pet);

    let mut all_res = HashMap::new();
    all_res.insert("Pets".to_owned(), pets);
    all_res.insert("Foods".to_owned(), foods);
    // dbg!(&all_res);
    all_res
}
