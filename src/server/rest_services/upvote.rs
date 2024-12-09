use anyhow::anyhow;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    db::Upvote,
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpvotePostRequest {
    post_id: i32,
}

pub async fn upvote_post_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Json(data): Json<UpvotePostRequest>,
    jwt_payload: Option<JwtData>,
) -> Result<Json<serde_json::Value>, anyhow::Error> {
    let user_id = jwt_payload
        .map(|d| d.user_id)
        .ok_or(anyhow!("Not logged in!"))?;
    ctx.lock()
        .await
        .db
        .upvote(Upvote {
            post_id: data.post_id,
            user_id,
        })
        .await?;
    Ok(Json(serde_json::json!({"ok":true})))
}
