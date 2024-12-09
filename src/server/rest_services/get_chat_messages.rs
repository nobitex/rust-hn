use anyhow::anyhow;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    db::DirectMessage,
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Debug, Deserialize, Clone)]
pub struct GetChatMessagesRequest {
    pub chat_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChatMessagesResponse {
    pub chat_messages: Vec<DirectMessage>,
}

pub async fn get_chat_messages_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Json(payload): Json<GetChatMessagesRequest>,
    jwt_payload: Option<JwtData>,
) -> Result<Json<GetChatMessagesResponse>, anyhow::Error> {
    let ctx = ctx.lock().await;

    let user_id = jwt_payload
        .map(|d| d.user_id)
        .ok_or(anyhow!("Not logged in!"))?;

    let chat = ctx.db.get_chat_users(payload.chat_id).await?;
    if chat.receiver_id != user_id && chat.sender_id != user_id {
        return Err(anyhow!("You are not part of this chat"));
    }

    let chats = ctx.db.get_chat_messages(payload.chat_id).await?;
    Ok(Json(GetChatMessagesResponse {
        chat_messages: chats,
    }))
}
