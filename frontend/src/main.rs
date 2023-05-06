// pub mod components;
// pub mod utils;
#![allow(non_snake_case)]

mod components;
mod utils;

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use indexmap::IndexMap;

use once_cell::sync::OnceCell;
use sir::global_css;
use utils::extract_urls::ItemRecords;

use crate::components::battle::ui::Battle;
use crate::components::footer::Footer;
use crate::components::home::Home;
use crate::components::nav::Nav;
// use crate::components::routes::AppRoutes;
use crate::utils::extract_urls::get_all_sap_records;

pub const EMPTY_SLOT_IMG: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/c/c1/Empty_set_symbol.svg/200px-Empty_set_symbol.svg.png";

fn main() {
    // Init debug tool for WebAssembly.
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus_web::launch(App);
}

pub type SAPRecords = IndexMap<String, ItemRecords>;
static RECORDS: OnceCell<SAPRecords> = OnceCell::new();

pub fn App(cx: Scope) -> Element {
    global_css!(
        r#"
        html,body,h1,h2,h3,h4,h5 {
            font-family: "Raleway", sans-serif;
        }
    "#
    );

    if let Some(Ok(item_img_urls)) =
        use_future(cx, (), |_| async move { get_all_sap_records().await }).value()
    {
        let _ = RECORDS.set(item_img_urls.to_owned());
    };

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
        body {
            class: "w3-white",
            Router {
                Nav {},
                br {}
                br {}
                br {}

                Route { to: "/home" , Home {} },
                Route { to: "/battle"  Battle {} },
                Route { to: "/about" }
                Footer {}
            }
        }
    })
}
