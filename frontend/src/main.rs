// pub mod components;
// pub mod utils;
#![allow(non_snake_case)]

mod components;
mod utils;

use dioxus::prelude::*;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use sir::{global_css, AppStyle};

use crate::components::routes::AppRoutes;
use crate::utils::extract_urls::extract_sap_image_urls;

pub const EMPTY_SLOT_IMG: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/c/c1/Empty_set_symbol.svg/200px-Empty_set_symbol.svg.png";

lazy_static! {
    static ref SAP_ITEM_IMG_URLS: IndexMap<String, IndexMap<String, String>> = {
        let mut img_urls = extract_sap_image_urls();
        img_urls["Pets"].insert("Slot".to_string(), EMPTY_SLOT_IMG.to_string());
        img_urls
    };
}

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
    // let fut = use_future(cx, (), |_| async move {
    //     reqwest::get("https://dog.ceo/api/breeds/image/random/")
    //         .await
    //         .unwrap()
    //         .json::<DogApi>()
    //         .await
    // });

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
            AppRoutes {}
        }
    })
}
