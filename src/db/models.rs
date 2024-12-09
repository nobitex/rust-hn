use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_salt: String,
    pub password_hash: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub created_at: Option<chrono::NaiveDateTime>,

    pub karma: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OnchainAddress {
    pub id: i32,
    pub user_id: i32,
    pub address: String,
    pub proof: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DirectMessage {
    pub id: i32,
    pub chat_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserChat {
    pub partner_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub root_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub content: Option<String>,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,

    pub username: Option<String>,
    pub comments_count: Option<i64>,
    pub total_upvotes: Option<i64>,
    pub upvoted: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostWrapper {
    pub post: Post,
    pub elapsed: String,
    pub depth: usize,
    pub is_me: bool,
    pub readable_link: Option<String>,
}

impl Post {
    pub fn new(
        user_id: i32,
        root_id: Option<i32>,
        parent_id: Option<i32>,
        title: Option<String>,
        link: Option<String>,
        content: Option<String>,
    ) -> Self {
        Self {
            parent_id,
            root_id,
            title,
            link,
            content,
            user_id,
            id: Default::default(),
            created_at: Default::default(),
            total_upvotes: Default::default(),
            upvoted: Default::default(),
            comments_count: Default::default(),
            username: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Upvote {
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Chat {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatId {
    pub id: i32,
    pub partner_username: Option<String>,
    pub partner_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatUsers {
    pub sender_id: i32,
    pub receiver_id: i32,
}
