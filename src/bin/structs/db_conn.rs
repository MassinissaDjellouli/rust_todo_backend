use std::env;
use std::sync::Once;
use sqlx::Postgres;

static mut INSTANCE:Option<DBConn> = None;
pub struct DBConn {
    pool: sqlx::Pool<Postgres>,
}
impl DBConn {
    #[async_recursion::async_recursion]
    pub async fn get_instance() -> &'static sqlx::Pool<Postgres> {
        unsafe{
            return match INSTANCE {
                Some(ref mut instance) => {
                    &instance.pool
                },
                None => {
                    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                    let pool = sqlx::postgres::PgPoolOptions::new()
                        .max_connections(5)
                        .connect(&db_url)
                        .await
                        .expect("Failed to connect to Postgres");
                    println!("Connected to Postgres");
                    INSTANCE = Some(DBConn {
                        pool
                    });
                    Self::get_instance().await
                }
            }
        }

    }
}