#![allow(non_snake_case)]

mod app;
mod components;
mod utils;

use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use lazy_static::lazy_static;
use std::collections::HashMap;
use utils::extract_urls::SAPItem;

use crate::{app::app, utils::extract_urls::extract_sap_image_urls};

lazy_static! {
    static ref SAP_ITEM_IMG_URLS: HashMap<String, HashMap<String, SAPItem>> =
        extract_sap_image_urls();
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
                <head> <title>Dioxus LiveView with axum</title>  </head>
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
