#![allow(non_snake_case)]

mod components;
mod records;

use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};
use indexmap::IndexMap;
use once_cell::sync::OnceCell;

use crate::{
    components::{about::About, battle::ui::Battle, footer::Footer, home::Home, nav::Nav},
    records::query::{get_all_sap_records, ItemRecords},
};

pub const DEV_BACKEND_API_URL: &str = "http://127.0.0.1:3030";
pub const BACKEND_API_URL: &str = "https://saptest.fly.dev";
pub const SAPTEST_URL: &str = "https://github.com/koisland/SuperAutoTest";
pub const SAPAI_URL: &str = "https://github.com/manny405/sapai";

pub type SAPRecords = IndexMap<String, ItemRecords>;
static RECORDS: OnceCell<SAPRecords> = OnceCell::new();

fn main() {
    // Init debug tool for WebAssembly.
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus_web::launch(App);
}

fn AppRoutes(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Nav {}
            for _ in 0..3 {
                br {}
            }
            Route { to: "/home", Home {} }
            Route { to: "/battle", Battle {} }
            Route { to: "/about", About {} }
            Redirect { from: "", to: "/home" }
            Footer {}
        }
    })
}

pub fn App(cx: Scope) -> Element {
    // Get all SAP records from backend on app init.
    if let Some(Ok(item_img_urls)) =
        use_future(cx, (), |_| async move { get_all_sap_records().await }).value()
    {
        let _ = RECORDS.set(item_img_urls.to_owned());
    };

    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://www.w3schools.com/w3css/4/w3.css" }
        link { rel: "stylesheet", href: "https://fonts.googleapis.com/css?family=Raleway" }
        body { class: "w3-white", AppRoutes {} }
    })
}
