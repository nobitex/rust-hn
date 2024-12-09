use axum::{http::HeaderValue, response::Redirect, Form};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemovePostRequest {
    id: i32,
}
pub async fn remove_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    re: Option<HeaderValue>,
    Form(req): Form<RemovePostRequest>,
    jwt_payload: Option<JwtData>,
) -> Result<Redirect, anyhow::Error> {
    if let Some(user_id) = jwt_payload.map(|d| d.user_id) {
        let u = ctx.lock().await.db.get_user_by_id(user_id).await?;
        if u.is_admin {
            ctx.lock().await.db.remove_post(req.id).await?;
        }
    }

    return if let Some(referer) = re {
        Ok(Redirect::to(referer.to_str()?))
    } else {
        Ok(Redirect::to("/"))
    };
}
