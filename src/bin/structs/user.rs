use serde::{Deserialize, Serialize};
use sqlx::Postgres;
#[derive(Serialize)]
#[derive(sqlx::FromRow)]
pub struct User{
    pub id: uuid::Uuid,
    pub username: String,
    pub hashed_password: String
}
#[derive(Deserialize)]
pub struct CreateUserDTO{
    pub username: String,
    pub hashed_password: String
}

impl User {
    pub async fn find_by_username(username: &str, pool: & sqlx::Pool<Postgres>) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT * FROM todo_app.users WHERE username = $1"#,
            username
        ).fetch_one(pool).await
    }
    pub async fn create(username: &str, hashed_password: &str, pool: & sqlx::Pool<Postgres>) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"INSERT INTO todo_app.users (username, hashed_password) VALUES ($1, $2) RETURNING *"#,
            username,
            hashed_password
        ).fetch_one(pool).await
    }
    pub async fn get(id: uuid::Uuid, pool: & sqlx::Pool<Postgres>) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT * FROM todo_app.users WHERE id = $1"#,
            id
        ).fetch_one(pool).await
    }
}