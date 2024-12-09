use axum::{response::Html, Form};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    jwt::TokenType,
    server::{Context, ContextDB},
};

#[derive(Debug, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn get_login_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
) -> Result<Html<String>, anyhow::Error> {
    Ok(Html(
        ctx.lock()
            .await
            .environment
            .get_template("login.html")?
            .render(())?,
    ))
}

pub async fn post_login_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Form(payload): Form<LoginRequest>,
) -> Result<Html<String>, anyhow::Error> {
    let ctx = ctx.lock().await;

    let user = match ctx.db.get_user(&payload.username, &payload.password).await {
        Ok(user) => user,
        Err(_) => {
            let mut data: HashMap<String, String> = HashMap::new();
            data.insert(
                "login_error".to_string(),
                "Invalid username or password".to_string(),
            );

            return Ok(Html(
                ctx.environment.get_template("login.html")?.render(data)?,
            ));
        }
    };

    let timestamp = chrono::Utc::now().timestamp();
    let jwt_access_token = ctx
        .jwt
        .create_token(user.id, TokenType::Access, timestamp + 90 * 24 * 60 * 60)
        .await?;

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("access_token".to_string(), jwt_access_token);
    data.insert("username".to_string(), user.username);
    data.insert("user_id".to_string(), user.id.to_string());

    Ok(Html(
        ctx.environment.get_template("login.html")?.render(data)?,
    ))
}
