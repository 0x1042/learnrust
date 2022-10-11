mod http_cli;
mod tcp;
mod tokio_tcp;

use clap::Parser;

/// mock http client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct HttpArgs {
    /// url to fetch
    #[clap(short, long, default_value = "https://www.bilibili.com")]
    url: String,
}

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::init();

    tokio_tcp::new_start("127.0.0.1:20085").await.unwrap();
}
