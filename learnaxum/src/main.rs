pub mod core;
pub mod db;
pub mod model;
pub mod opts;
pub mod rcache;
pub mod routers;
pub mod state;

use clap::Parser;
use opts::AppEnv;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "TRACE")
    }

    tracing_subscriber::fmt::init();

    let env = AppEnv::parse();

    core::run(&env).await;
}
