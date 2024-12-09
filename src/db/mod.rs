use async_trait::async_trait;

mod models;
pub use models::*;

mod postgres;
pub use postgres::*;

#[async_trait]
pub trait DB {
    async fn get_user_by_id(&self, user_id: i32) -> Result<User, sqlx::Error>;
    async fn remove_post(&self, post_id: i32) -> Result<(), sqlx::Error>;
    async fn upvote(&self, upvote: Upvote) -> Result<(), sqlx::Error>;

    async fn submit_post(&self, post: Post) -> Result<i32, sqlx::Error>;
    async fn get_posts(
        &self,
        user_id: Option<i32>,
        root_id: Option<i32>,
        post_id: Option<i32>,
        prefix: &str,
        page: u32,
    ) -> Result<Vec<PostWrapper>, sqlx::Error>;

    async fn add_user(
        &self,
        username: &str,
        salt: &str,
        raw_password: &str,
    ) -> Result<i32, sqlx::Error>;
    async fn get_user(&self, username: &str, raw_password: &str) -> Result<User, sqlx::Error>;

    async fn get_user_onchain_addresses(
        &self,
        user_id: i32,
    ) -> Result<Vec<OnchainAddress>, sqlx::Error>;
    async fn add_user_onchain_address(
        &self,
        user_id: i32,
        address: &str,
        proof: &str,
    ) -> Result<i32, sqlx::Error>;

    // async fn get_user_chats(&self, user_id: i32) -> Result<Vec<UserChat>, sqlx::Error>;
    async fn get_user_by_onchain_address(&self, address: &str) -> Result<User, sqlx::Error>;

    async fn start_chat(&self, user_id: i32, partner_id: i32) -> Result<i32, sqlx::Error>;
    async fn get_user_chats(&self, user_id: i32) -> Result<Vec<ChatId>, sqlx::Error>;
    async fn get_chat_messages(&self, chat_id: i32) -> Result<Vec<DirectMessage>, sqlx::Error>;
    async fn get_chat_users(&self, chat_id: i32) -> Result<ChatUsers, sqlx::Error>;
    async fn send_chat_message(
        &self,
        chat_id: i32,
        receiver_id: i32,
        message: &str,
    ) -> Result<i32, sqlx::Error>;
}
