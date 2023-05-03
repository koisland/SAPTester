use crate::{api::db::db, battle::response::battle};
use axum::{
    routing::{get, post},
    Router,
};
use log::LevelFilter;

mod api;
mod battle;

#[tokio::main]
async fn main() {
    simple_logging::log_to_file("backend.log", LevelFilter::Info).unwrap();

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let app = Router::new()
        .route("/battle", post(battle))
        .route("/db", get(db));

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
