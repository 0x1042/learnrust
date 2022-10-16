use diesel::prelude::*;

#[derive(Queryable)]
pub struct UserInfo {
    pub id: i32,
    pub nike_name: String,
    pub email: String,
    pub invalid: bool,
}
