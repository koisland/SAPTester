use indexmap::IndexMap;
use serde_json::Value;
pub type ItemIconUrls = IndexMap<String, String>;

lazy_static::lazy_static!(
    static ref NAME_EXCEPTIONS: IndexMap<String, String> = IndexMap::from(
        [
            ("Popcorns".to_owned(), "Popcorn".to_owned()),
            ("Hammershark".to_owned(), "Hammerhead_Shark".to_owned())
        ]
    );
);

// const BASE_FANDOM_FILE_URL: &str = "https://superautopets.fandom.com/wiki/File:";
// const RGX_MDATA_URL: &str = r"https://static.wikia.nocookie.net/superautopets/images/\w{1}/\w{2}/";
// pub const ATTACK_ICON: &str =
//     "https://static.wikia.nocookie.net/superautopets/images/a/aa/Attack_Icon.png";
// pub const HEALTH_ICON: &str =
//     "https://static.wikia.nocookie.net/superautopets/images/4/44/Health_Icon.png";

#[derive(Debug, Clone)]
pub struct SAPIcon {
    pub name: String,
    pub icon: String,
}

impl std::hash::Hash for SAPIcon {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.icon.hash(state);
    }
}

fn extract_existing_urls(path: &str) -> IndexMap<String, String> {
    // Find the file path for an existing json file with the {item_name: item_icon_url}
    let items_str = std::fs::read_to_string(path).expect("No file with items urls.");
    let mut items: IndexMap<String, String> = IndexMap::new();
    let items_json: Value = serde_json::from_str(&items_str).unwrap();
    // Unpack value and insert in order.
    if let Value::Object(map) = items_json {
        for (item_name, item_img_url) in map.into_iter() {
            items.insert(item_name, item_img_url.as_str().unwrap_or("").to_owned());
        }
    };

    items
}

pub fn extract_sap_image_urls() -> IndexMap<String, ItemIconUrls> {
    let foods = extract_existing_urls("data/foods.json");
    let pets = extract_existing_urls("data/pets.json");

    let mut all_res = IndexMap::new();
    all_res.insert("Pets".to_owned(), pets);
    all_res.insert("Foods".to_owned(), foods);
    // dbg!(&all_res);
    all_res
}
