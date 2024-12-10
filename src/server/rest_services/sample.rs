use axum::Json;
use serde::{Deserialize, Serialize};
use std::{ops::Add, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    db::User,
    server::{Context, ContextDB},
};

#[derive(Debug, Deserialize, Clone)]
pub struct SampleRequest {
    pub sample: String,
}

#[derive(Serialize, Clone)]
pub struct SampleResponse {
    pub users: Vec<User>,
}

pub async fn sample_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Json(payload): Json<SampleRequest>,
) -> Result<Json<SampleResponse>, anyhow::Error> {
    let users = ctx.lock().await.db.get_users().await?;
    log::info!("Users: {:?}", users);

    Ok(Json(SampleResponse { users }))
}
