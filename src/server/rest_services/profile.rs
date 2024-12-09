use anyhow::anyhow;
use axum::response::Html;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    db::User,
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProfileData {
    user: User,
}

pub async fn profile_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    jwt_payload: Option<JwtData>,
) -> Result<Html<String>, anyhow::Error> {
    let ctx = ctx.lock().await;

    let user_id = jwt_payload
        .map(|d| d.user_id)
        .ok_or(anyhow!("Not logged in!"))?;
    let user = ctx.db.get_user_by_id(user_id).await?;

    Ok(Html(
        ctx.environment
            .get_template("profile.html")?
            .render(ProfileData { user })?,
    ))
}
