use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub(crate) redis: Arc<Mutex<redis::aio::Connection>>,
}

impl AppState {
    pub fn new(redis: Arc<Mutex<redis::aio::Connection>>) -> Self {
        Self { redis }
    }
}
