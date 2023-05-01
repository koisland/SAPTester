#![allow(non_snake_case)]

mod app;
mod components;
mod utils;

use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use saptest::{
    db::{
        pack::Pack,
        record::{PetRecord, SAPRecord},
    },
    PetName,
};
use utils::extract_urls::SAPItem;

use crate::{app::app, utils::extract_urls::extract_sap_image_urls};

pub const EMPTY_SLOT_IMG: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/c/c1/Empty_set_symbol.svg/200px-Empty_set_symbol.svg.png";

lazy_static! {
    static ref SAP_ITEM_IMG_URLS: IndexMap<String, IndexMap<String, SAPItem>> = {
        let mut img_urls = extract_sap_image_urls();
        let empty_pet_rec = PetRecord {
            name: PetName::Custom("Empty".to_owned()),
            tier: 0,
            attack: 0,
            health: 0,
            pack: Pack::Unknown,
            effect_trigger: None,
            effect: None,
            effect_atk: 0,
            effect_health: 0,
            n_triggers: 0,
            temp_effect: false,
            lvl: 0,
            cost: 0,
        };
        let empty_slot = SAPItem {
            icon: EMPTY_SLOT_IMG.to_string(),
            record: SAPRecord::Pet(empty_pet_rec),
        };
        img_urls["Pets"].insert("Slot".to_string(), empty_slot);
        img_urls
    };
}

#[tokio::main]
async fn main() {
    assert!(SAP_ITEM_IMG_URLS.contains_key("Pets"));
    assert!(SAP_ITEM_IMG_URLS.contains_key("Foods"));

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        .route(
            "/",
            get(move || async move {
                Html(format!(
                    r#"
            <!DOCTYPE html>
            <html>
                <head> <title>SAPTester</title>  </head>
                <body> <div id="main"></div> </body>
                {glue}
            </html>
            "#,
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
                ))
            }),
        )
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    _ = view.launch(dioxus_liveview::axum_socket(socket), app).await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
