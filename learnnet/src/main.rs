mod http_cli;
mod tcp;

use std::time::Instant;

use clap::Parser;
use tracing::debug;
use tracing::error;
use tracing::info;

/// mock http client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct HttpArgs {
    /// url to fetch
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::init();

    let hargs = HttpArgs::parse();
    debug!("start get1...");
    let start = Instant::now();

    // match http_cli::get(&hargs.url).await {
    //     Ok(str) => {
    //         info!("response  {}", str.len());
    //     }
    //     Err(err) => {
    //         error!("err {}", err);
    //     }
    // };

    let ff = http_cli::get(&hargs.url);

    debug!("start get2... {:?}", start.elapsed());

    match http_cli::get_origin_body(&hargs.url).await {
        Ok(_) => {}
        Err(err) => {
            error!("err {:?}", err);
        }
    }

    debug!("start get3... {:?}", start.elapsed());

    let res = ff.await;

    debug!("start get3... {:?}", start.elapsed());

    // debug!("res {:?}", res);
    // tcp::start("127.0.0.1:10085").unwrap();
}
