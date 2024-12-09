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
pub struct StartChatRequest {
    pub receiver_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StartChatResponse {
    pub chat_id: i32,
}

pub async fn start_chat_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Json(payload): Json<StartChatRequest>,
    jwt_payload: Option<JwtData>,
) -> Result<Json<StartChatResponse>, anyhow::Error> {
    let ctx = ctx.lock().await;

    let user_id = jwt_payload
        .map(|d| d.user_id)
        .ok_or(anyhow!("Not logged in!"))?;
    // let user = match ctx.db.get_user_by_id(user_id).await {
    //     Ok(user) => user,
    //     Err(_) => return Err(anyhow!("User not found")),
    // };
    // if user.is_verified == false {
    //     return Err(anyhow!("You are not verified"));
    // }

    let receiver_user_id = match ctx
        .db
        .get_user_by_onchain_address(&payload.receiver_address)
        .await
    {
        Ok(user) => user.id,
        Err(_) => return Err(anyhow!("Receipt user not found")),
    };

    if user_id == receiver_user_id {
        return Err(anyhow!("Can't start chat with yourself"));
    }

    let chat_id = match ctx.db.start_chat(user_id, receiver_user_id).await {
        Ok(chat_id) => chat_id,
        Err(_) => return Err(anyhow!("Chat already exists")),
    };

    Ok(Json(StartChatResponse { chat_id }))
}
