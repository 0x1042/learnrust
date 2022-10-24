use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Clone)]
pub struct UserInfo {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub invalid: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub user_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub user_name: Option<String>,
    pub email: Option<String>,
}
