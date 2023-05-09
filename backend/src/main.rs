use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::Router;
use clap::Parser;
use http::header::{ACCEPT, CONTENT_TYPE};
use hyper::Method;
use log::LevelFilter;
use tower_http::cors::{Any, CorsLayer};

mod args;
mod battle;
mod db;
mod routes;

use crate::{
    args::Args,
    routes::{battle_routes, db_routes},
};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    simple_logging::log_to_file(
        "backend.log",
        LevelFilter::from_str(&args.log_level).unwrap(),
    )
    .unwrap();

    let addr: SocketAddr = SocketAddr::from((
        IpAddr::from_str(args.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::UNSPECIFIED)),
        args.port,
    ));

    let app = app();

    log::info!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn app() -> Router {
    // https://docs.rs/tower-http/0.4.0/tower_http/cors/index.html
    let cors = CorsLayer::new()
        .allow_headers([CONTENT_TYPE, ACCEPT])
        // Allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // Allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .merge(db_routes())
        .merge(battle_routes())
        .layer(cors)
}
