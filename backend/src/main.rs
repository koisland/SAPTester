use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use clap::Parser;

use axum::{
    routing::{get, post},
    Router,
};
use log::LevelFilter;

mod api;
mod args;
mod battle;

use crate::{api::db::db, args::Args, battle::response::battle};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    simple_logging::log_to_file("backend.log", LevelFilter::Error).unwrap();

    let addr: SocketAddr = SocketAddr::from((
        IpAddr::from_str(args.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        args.port,
    ));

    let app = Router::new()
        .route("/battle", post(battle))
        .route("/db", get(db));

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
