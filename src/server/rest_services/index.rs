use axum::{extract::Query, response::Html};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    db::User,
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexGetRequest {
    p: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct IndexData {
    posts: Vec<crate::db::PostWrapper>,
    user: Option<User>,
    page: usize,
}

pub async fn index_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    prefix: &str,
    jwt_payload: Option<JwtData>,
    req: Query<IndexGetRequest>,
) -> Result<Html<String>, anyhow::Error> {
    let ctx = &ctx.lock().await;

    let user_id = jwt_payload.map(|d| d.user_id);
    let user = if let Some(uid) = user_id {
        Some(ctx.db.get_user_by_id(uid).await?)
    } else {
        None
    };

    let data = IndexData {
        posts: ctx
            .db
            .get_posts(
                user_id,
                None,
                None,
                prefix,
                req.p.unwrap_or_default().try_into()?,
            )
            .await?,
        user,
        page: req.p.unwrap_or_default(),
    };

    Ok(Html(
        ctx.environment.get_template("index.html")?.render(data)?,
    ))
}
