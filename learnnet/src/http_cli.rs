use std::collections::HashMap;
use std::time;

use tracing::debug;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = init_http_cli().unwrap();
}

fn init_http_cli() -> anyhow::Result<reqwest::Client> {
    let cli = reqwest::ClientBuilder::new()
        .connect_timeout(time::Duration::from_secs(1))
        .timeout(time::Duration::from_secs(2))
        .pool_idle_timeout(time::Duration::from_secs(60 * 10))
        .pool_max_idle_per_host(1024)
        .tcp_nodelay(true)
        .connection_verbose(false)
        .build()?;

    debug!("init http client success ");
    Ok(cli)
}

pub async fn get(url: &str) -> anyhow::Result<String> {
    let resp = CLIENT
        .get(url)
        .send()
        .await?
        .text_with_charset("utf8")
        .await?;

    Ok(resp)
}

pub async fn get_resp_json(url: &str) -> anyhow::Result<usize> {
    let json = CLIENT
        .get(url)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    // debug!("json response {:?}", json);

    Ok(json.len())
}

pub async fn get_origin_body(url: &str) -> anyhow::Result<()> {
    let body = CLIENT.get(url).send().await?;
    let addr = body.remote_addr();
    debug!("addr :{:?}", addr);

    let status = body.status();

    debug!("status :{:?}", status);

    let bb = body.bytes().await?;

    // debug!("body:{:?}", bb);
    Ok(())
}
