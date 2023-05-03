// pub mod components;
// pub mod utils;
#![allow(non_snake_case)]

// use lazy_static::lazy_static;
use dioxus::prelude::*;
use sir::{global_css, AppStyle};

// use crate::components::routes::AppRoutes;

// use utils::extract_urls::SAPItem;

// use crate::utils::extract_urls::extract_sap_image_urls;

// pub const EMPTY_SLOT_IMG: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/c/c1/Empty_set_symbol.svg/200px-Empty_set_symbol.svg.png";

// lazy_static! {
//     static ref SAP_ITEM_IMG_URLS: IndexMap<String, IndexMap<String, SAPItem>> = {
//         let mut img_urls = extract_sap_image_urls();
//         let empty_pet_rec = PetRecord {
//             name: PetName::Custom("Empty".to_owned()),
//             tier: 0,
//             attack: 0,
//             health: 0,
//             pack: Pack::Unknown,
//             effect_trigger: None,
//             effect: None,
//             effect_atk: 0,
//             effect_health: 0,
//             n_triggers: 0,
//             temp_effect: false,
//             lvl: 0,
//             cost: 0,
//         };
//         let empty_slot = SAPItem {
//             icon: EMPTY_SLOT_IMG.to_string(),
//             record: SAPRecord::Pet(empty_pet_rec),
//         };
//         img_urls["Pets"].insert("Slot".to_string(), empty_slot);
//         img_urls
//     };
// }

fn main() {
    // Init debug tool for WebAssembly.
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    // assert!(SAP_ITEM_IMG_URLS.contains_key("Pets"));
    // assert!(SAP_ITEM_IMG_URLS.contains_key("Foods"));
    global_css!(
        r#"
        html,body,h1,h2,h3,h4,h5 {
            font-family: "Raleway", sans-serif;
        }
    "#
    );
    // https://www.w3schools.com/w3css/tryit.asp?filename=tryw3css_templates_analytics&stacked=h
    cx.render(rsx!{
        link {
            rel: "stylesheet",
            href: "https://www.w3schools.com/w3css/4/w3.css"
        }
        link {
            rel:"stylesheet",
            href:"https://fonts.googleapis.com/css?family=Raleway"
        }
        link {
            rel:"stylesheet",
            href:"https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
        }
        AppStyle {},
        body {
            class: "w3-white",
            // AppRoutes {}
        }
    })
}
