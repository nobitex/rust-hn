use std::str::FromStr;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{query_as, PgPool};
use structopt::StructOpt;
use url::Url;

use crate::server::ContextDB;

use super::PostWrapper;

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
    _opt: PostgresOpt,
    pool: PgPool,
}

impl PostgresDB {
    pub async fn new(opt: PostgresOpt) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(&opt.url()).await?;
        Ok(Self { _opt: opt, pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        let migrations = sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(migrations)
    }
}

#[async_trait]
impl super::DB for PostgresDB {
    async fn get_user_by_id(&self, user_id: i32) -> Result<super::User, sqlx::Error> {
        let user = query_as!(
            super::User,
            r#" SELECT users.*, 
                (SELECT COUNT(*) FROM upvotes
                    JOIN posts on upvotes.post_id=posts.id
                    WHERE posts.user_id=users.id)
                as karma FROM users WHERE id = $1"#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
    async fn remove_post(&self, post_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM posts cascade where id = $1", post_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn upvote(&self, upvote: super::Upvote) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO upvotes (user_id, post_id) VALUES ($1, $2)",
            upvote.user_id,
            upvote.post_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn submit_post(&self, post: super::Post) -> Result<i32, sqlx::Error> {
        let id = sqlx::query!(
            r#" INSERT INTO posts (title, link, user_id, root_id, parent_id, content)
                VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"#,
            post.title,
            post.link,
            post.user_id,
            post.root_id,
            post.parent_id,
            post.content
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        Ok(id)
    }
    async fn get_posts(
        &self,
        user_id: Option<i32>,
        root_id: Option<i32>,
        post_id: Option<i32>,
        prefix: &str,
        page: u32,
    ) -> Result<Vec<super::PostWrapper>, sqlx::Error> {
        let posts = query_as!(
            super::Post,
            r#"SELECT
                posts.*,
                users.username                                                  as username, 
                COUNT(upvotes.post_id)                                          as total_upvotes, 
                (SELECT true FROM upvotes 
                    WHERE upvotes.post_id = posts.id AND upvotes.user_id = $1)  as upvoted,
                (SELECT COUNT(*) FROM posts as pst
                    WHERE pst.root_id = posts.id)                               as comments_count
                FROM posts 
                LEFT JOIN upvotes ON upvotes.post_id = posts.id 
                LEFT JOIN users ON users.id = posts.user_id
                WHERE ($2 = -1 OR posts.id = $2) AND
                    (($4 = -1 AND posts.root_id is null) OR ($4 <> -1 AND posts.root_id = $4)) AND
                    (posts.title is null OR starts_with(posts.title, $5))
                GROUP BY (users.username, posts.id)
                ORDER BY posts.created_at DESC
                OFFSET $3
                LIMIT 20"#,
            user_id,
            post_id.unwrap_or(-1),
            (page * 20) as i32,
            root_id.unwrap_or(-1),
            prefix
        )
        .fetch_all(&self.pool)
        .await?;
        let mut psts = Vec::new();
        for post in posts.into_iter() {
            let secs = (Utc::now().naive_local() - post.created_at).num_seconds();
            let elapsed = if secs < 60 {
                format!("{} seconds", secs)
            } else if secs < 60 * 60 {
                format!("{} minutes", secs / 60)
            } else if secs < 60 * 60 * 24 {
                format!("{} hours", secs / 60 / 60)
            } else {
                format!("{} days", secs / 60 / 60 / 24)
            };
            let mut readable_link = None;
            if let Some(lnk) = &post.link {
                if let Ok(u) = Url::from_str(lnk) {
                    readable_link = u.domain().map(|s| s.to_string());
                }
            }
            psts.push(PostWrapper {
                readable_link,
                is_me: Some(post.user_id) == user_id,
                post,
                elapsed,
                depth: 0,
            });
        }
        Ok(psts)
    }

    async fn add_user(
        &self,
        username: &str,
        salt: &str,
        raw_password: &str,
    ) -> Result<i32, sqlx::Error> {
        let hashed_password = sha256::digest(salt.to_string() + raw_password);
        let id = sqlx::query!(
            "INSERT INTO users (username, password_salt, password_hash) VALUES ($1, $2, $3) RETURNING id",
            username,
            salt,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        Ok(id)
    }

    async fn get_user(
        &self,
        username: &str,
        raw_password: &str,
    ) -> Result<super::User, sqlx::Error> {
        let user = query_as!(
            super::User,
            r#"SELECT *, CAST(0 as bigint) as karma FROM users WHERE username = $1"#,
            username
        )
        .fetch_one(&self.pool)
        .await?;
        let hashed_password = sha256::digest(user.password_salt.to_string() + raw_password);
        if hashed_password == user.password_hash {
            Ok(user)
        } else {
            Err(sqlx::error::Error::RowNotFound)
        }
    }

    async fn get_user_onchain_addresses(
        &self,
        user_id: i32,
    ) -> Result<Vec<super::OnchainAddress>, sqlx::Error> {
        let addresses = query_as!(
            super::OnchainAddress,
            "SELECT * FROM onchain_addresses WHERE user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(addresses)
    }

    async fn add_user_onchain_address(
        &self,
        user_id: i32,
        address: &str,
        proof: &str,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO onchain_addresses (user_id, address, proof) VALUES ($1, $2, $3) RETURNING id",
            user_id,
            address,
            proof,
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        Ok(id)
    }

    async fn get_user_by_onchain_address(&self, address: &str) -> Result<super::User, sqlx::Error> {
        let user = query_as!(
            super::User,
            r#"
            SELECT users.*, (select 1000000000000) as karma FROM users WHERE id = (
                SELECT user_id FROM onchain_addresses WHERE address = $1
            )
            "#,
            address
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn start_chat(&self, user_id: i32, partner_id: i32) -> Result<i32, sqlx::Error> {
        let id = sqlx::query!(
            r#"
                INSERT INTO chats (sender_id, receiver_id) SELECT $1, $2
                WHERE NOT EXISTS (
                    SELECT id FROM chats WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
                ) RETURNING id;
            "#,
            user_id,
            partner_id
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        Ok(id)
    }

    async fn get_user_chats(&self, user_id: i32) -> Result<Vec<super::ChatId>, sqlx::Error> {
        let chats = query_as!(
            super::ChatId,
            r#"
            SELECT id, (
                SELECT username FROM users WHERE (id = receiver_id OR id = sender_id) AND id <> $1
            ) as partner_username, (
                SELECT id FROM users WHERE (id = receiver_id OR id = sender_id) AND id <> $1
            ) as partner_id FROM chats WHERE sender_id = $1 OR receiver_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(chats)
    }

    async fn get_chat_users(&self, chat_id: i32) -> Result<super::ChatUsers, sqlx::Error> {
        let chat = query_as!(
            super::ChatUsers,
            r#"
            SELECT sender_id, receiver_id FROM chats WHERE id = $1
            "#,
            chat_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(chat)
    }

    async fn get_chat_messages(
        &self,
        chat_id: i32,
    ) -> Result<Vec<super::DirectMessage>, sqlx::Error> {
        let messages = query_as!(
            super::DirectMessage,
            r#"
            SELECT * FROM direct_messages WHERE chat_id = $1
            "#,
            chat_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(messages)
    }

    async fn send_chat_message(
        &self,
        chat_id: i32,
        receiver_id: i32,
        message: &str,
    ) -> Result<i32, sqlx::Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO direct_messages (chat_id, receiver_id, message) VALUES ($1, $2, $3) RETURNING id
            "#,
            chat_id,
            receiver_id,
            message
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        Ok(id)
    }
}

impl ContextDB for PostgresDB {}
