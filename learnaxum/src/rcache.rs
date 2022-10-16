use redis::{
    from_redis_value, AsyncCommands, ConnectionAddr, ConnectionInfo, RedisConnectionInfo, Value,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::opts::AppEnv;

const ONE_DAY: usize = 60 * 60 * 24;
const FIELD: &str = "data";

// redis://127.0.0.1:6379/
pub async fn create_redis_cli(flag: &AppEnv) -> anyhow::Result<redis::aio::Connection> {
    let connection_info = ConnectionInfo {
        redis: RedisConnectionInfo {
            db: i64::from(flag.redis_db),
            password: flag.redis_passwd.clone(),
            username: flag.redis_user.clone(),
        },
        addr: ConnectionAddr::Tcp(flag.redis_host.clone(), flag.redis_port),
    };

    let client = redis::Client::open(connection_info)?;

    let conn = client.get_async_connection().await?;

    Ok(conn)
}

fn optional_null<T: DeserializeOwned>(v: &Value) -> anyhow::Result<Option<T>> {
    let valid_string = from_redis_value::<String>(v)?;
    Ok(Some(serde_json::from_str::<T>(&valid_string)?))
}

pub async fn set<T: Serialize + Send + Sync>(
    redis: &std::sync::Arc<tokio::sync::Mutex<redis::aio::Connection>>,
    key: &str,
    val: &T,
) -> anyhow::Result<()> {
    let key = key.to_string();
    let value = serde_json::to_string(&val)?;
    redis.lock().await.hset(&key, FIELD, value).await?;
    redis.lock().await.expire(&key, ONE_DAY).await?;
    Ok(())
}

pub async fn get<T: DeserializeOwned + Send>(
    redis: &std::sync::Arc<tokio::sync::Mutex<redis::aio::Connection>>,
    key: &str,
) -> anyhow::Result<Option<T>> {
    let key = key.to_string();
    let value: Value = redis.lock().await.hget(&key, FIELD).await?;

    let serialized_data: Option<T> = optional_null(&value)?;
    if serialized_data.is_some() {
        redis.lock().await.expire(&key, ONE_DAY).await?;
    }
    tracing::trace!("cache hit {}", &key);
    Ok(serialized_data)
}
