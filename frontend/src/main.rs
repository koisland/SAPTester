#![allow(non_snake_case)]

mod components;
mod utils;

use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};
use indexmap::IndexMap;
use once_cell::sync::OnceCell;

use crate::{
    components::{about::About, battle::ui::Battle, footer::Footer, home::Home, nav::Nav},
    utils::get_records::{get_all_sap_records, ItemRecords},
};

pub const SAPTEST_URL: &str = "https://github.com/koisland/SuperAutoTest";
pub const SAPAI_URL: &str = "https://github.com/manny405/sapai";
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
    // Get all SAP records from backend on app init.
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

                for _ in 0..3 {
                    br {}
                }

                Route { to: "/home" , Home {} },
                Route { to: "/battle",  Battle {} },
                Route { to: "/about", About {} },
                Redirect { from: "", to: "/home" }
                Footer {}
            }
        }
    })
}
