use std::net::SocketAddr;

use axum::{routing::get, Router};
use tokio::signal::{self};
use tower::ServiceBuilder;
use tracing::warn;

use crate::{
    opts::AppEnv,
    rcache::create_redis_cli,
    routers::{index, user_info},
    state::AppState,
};

#[allow(clippy::expect_used)]
async fn shutdown_signal(env: &AppEnv) {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("install Ctrl+C fail.");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("install signal handler fail.")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    warn!("wait for {} seconds to exist", env.wait_time.as_secs());
    std::thread::sleep(env.wait_time);
    warn!("graceful shutdown...")
}

pub async fn run(env: &AppEnv) {
    let redis_conn = create_redis_cli(env).await.unwrap();

    let appstate = AppState {
        redis: std::sync::Arc::new(tokio::sync::Mutex::new(redis_conn)),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/user_info", get(user_info))
        .layer(ServiceBuilder::new().layer(axum::Extension(appstate)));

    let addr = &env.addr.parse::<SocketAddr>().unwrap();

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(env))
        .await
        .unwrap();
}
