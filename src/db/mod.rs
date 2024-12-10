use async_trait::async_trait;

mod postgres;
pub use postgres::*;

#[async_trait]
pub trait DB {
    async fn get_users(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn add_user(&self, name: &str) -> Result<(), sqlx::Error>;
}
