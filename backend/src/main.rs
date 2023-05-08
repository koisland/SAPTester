use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::Router;
use clap::Parser;
use log::LevelFilter;

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
        IpAddr::from_str(args.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
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
    Router::new().merge(db_routes()).merge(battle_routes())
}
