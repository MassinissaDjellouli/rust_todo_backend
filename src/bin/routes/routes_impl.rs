use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use axum::http::StatusCode;
use axum::Json;
use crate::bin::structs::db_conn::DBConn;
use crate::bin::structs::user::{CreateUserDTO, User};
use sqlx::Error as SqlxError;

pub struct Routes<'a,I,O>
where I:serde::de::DeserializeOwned,
      O:serde::Serialize{
    routes:HashMap<&'a str, Box<dyn Fn(Json<I>) -> dyn Future<Output=Result<O,StatusCode>>>>
}

impl<'a,I,O> Routes<'a,I,O>
    where I:serde::de::DeserializeOwned,
          O:serde::Serialize{
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        routes.insert("/user",Box::new(post_create_user));
        Self {
            routes
        }
    }
    pub async fn get_route(&self,route:&str) -> Result<&Box<dyn Fn(Json<I>) -> dyn Future<Output=Result<O,StatusCode>>>,StatusCode> {
        self.routes.get(route).ok_or(StatusCode::NOT_FOUND)

    }
}
pub async fn post_create_user(Json(payload): Json<CreateUserDTO>) -> Result<Json<User>,StatusCode> {
    let username = payload.username;
    let hashed_password = payload.hashed_password;
    match User::create(username.as_str(), hashed_password.as_str(),DBConn::get_instance().await).await{
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(match e{
            SqlxError::RowNotFound | SqlxError::ColumnNotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        })
    }
}