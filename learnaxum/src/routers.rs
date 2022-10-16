use axum::{extract::Query, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{rcache, state::AppState};

pub async fn index() -> &'static str {
    "Hello, World!"
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    // #[serde(default, deserialize_with = "empty_string_as_none")]
    pub uid: u32,
    pub uname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub uid: u32,
    pub uname: String,
    pub email: Option<String>,
}

pub async fn user_info(
    Extension(state): Extension<AppState>,
    Query(param): Query<Params>,
) -> Json<UserInfo> {
    let key = format!("{}->{}", param.uid, param.uname);
    let resp = rcache::get::<UserInfo>(&state.redis, key.as_str()).await;

    let mut ui = UserInfo {
        uid: param.uid,
        uname: param.uname.clone(),
        email: Default::default(),
    };

    if let Ok(user_op) = resp {
        match user_op {
            Some(user) => Json(user),
            None => Json(ui),
        }
    } else {
        ui.email = Some(format!("{}@gmail.com", ui.uname));
        let _ = rcache::set::<UserInfo>(&state.redis, &key, &ui).await;
        Json(ui)
    }
}
