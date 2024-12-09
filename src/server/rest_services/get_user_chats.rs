use anyhow::anyhow;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    db::ChatId,
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserChatsResponse {
    pub chats: Vec<ChatId>,
}

pub async fn get_user_chats_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    jwt_payload: Option<JwtData>,
) -> Result<Json<GetUserChatsResponse>, anyhow::Error> {
    let ctx = ctx.lock().await;

    let user_id = jwt_payload
        .map(|d| d.user_id)
        .ok_or(anyhow!("Not logged in!"))?;

    let chats = ctx.db.get_user_chats(user_id).await?;

    Ok(Json(GetUserChatsResponse { chats }))
}
