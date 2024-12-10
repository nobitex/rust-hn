use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use structopt::StructOpt;

use crate::server::ContextDB;

use super::DB;

#[derive(Debug, Clone, StructOpt)]
pub struct PostgresOpt {
    #[structopt(long, default_value = "localhost", env = "POSTGRES_HOST")]
    pub pg_host: String,
    #[structopt(long, default_value = "5432", env = "POSTGRES_PORT")]
    pub pg_port: u16,
    #[structopt(long, default_value = "postgres", env = "POSTGRES_USER")]
    pub user: String,
    #[structopt(long, default_value = "password", env = "POSTGRES_PASSWORD")]
    pub pg_password: String,
    #[structopt(long, default_value = "satoshi", env = "POSTGRES_DB")]
    pub pg_db: String,
}

impl PostgresOpt {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.pg_password, self.pg_host, self.pg_port, self.pg_db
        )
    }
}

#[derive(Debug, Clone)]
pub struct PostgresDB {
    opt: PostgresOpt,
    pool: PgPool,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct User {
    pub id: i32,
    pub name: String,
}

impl PostgresDB {
    pub async fn new(opt: PostgresOpt) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(&opt.url()).await?;
        Ok(Self { opt, pool })
    }
}

#[async_trait]
impl DB for PostgresDB {
    async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = query_as!(User, "SELECT * FROM Sample")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    async fn add_user(&self, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO Sample (name) VALUES ($1)")
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

impl ContextDB for PostgresDB {}
