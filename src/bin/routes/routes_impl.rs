use axum::Json;
use crate::bin::structs::db_conn::DBConn;
use crate::bin::structs::user::{CreateUserDTO, User};


pub fn post_create_user(Json(payload): Json<CreateUserDTO>) -> Result<Json<User>,Err> {
    let username = payload.username;
    let hashed_password = payload.hashed_password;
    User::create(username.as_str(), hashed_password.as_str(),DBConn::GetInstance())?
}