use clap::Parser;

// https://github.com/dxps/fullstack-rust-axum-dioxus-rwa/blob/main/backend/src/bin/server.rs
#[derive(Parser, Debug)]
#[clap(
    name = "server",
    about = "The server side of Fullstack Rust RealWorld App project."
)]
pub struct Args {
    /// The HTTP listening address.
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1")]
    pub addr: String,

    /// The HTTP listening port.
    #[clap(short = 'p', long = "port", default_value = "3030")]
    pub port: u16,

    /// The logging level.
    #[clap(short = 'l', long = "log", default_value = "info")]
    pub log_level: String,
}
