use anyhow::anyhow;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Debug, Deserialize, Clone)]
pub struct SendChatMessageRequest {
    pub chat_id: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendChatMessageResponse {
    pub direct_message_id: i32,
}

pub async fn send_chat_message_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Json(payload): Json<SendChatMessageRequest>,
    jwt_payload: Option<JwtData>,
) -> Result<Json<SendChatMessageResponse>, anyhow::Error> {
    let ctx = ctx.lock().await;

    let user_id = jwt_payload
        .map(|d| d.user_id)
        .ok_or(anyhow!("Not logged in!"))?;

    let chat = ctx.db.get_chat_users(payload.chat_id).await?;
    if chat.receiver_id != user_id && chat.sender_id != user_id {
        return Err(anyhow!("You are not part of this chat"));
    }

    let receiver_id = if chat.receiver_id == user_id {
        chat.sender_id
    } else {
        chat.receiver_id
    };

    let direct_message_id = ctx
        .db
        .send_chat_message(payload.chat_id, receiver_id, &payload.message)
        .await?;

    Ok(Json(SendChatMessageResponse { direct_message_id }))
}
