use axum::{response::Html, Form};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    jwt::TokenType,
    server::{Context, ContextDB},
};

#[derive(Debug, Deserialize, Clone)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

pub async fn post_signup_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Form(payload): Form<SignupRequest>,
) -> Result<Html<String>, anyhow::Error> {
    let ctx = ctx.lock().await;

    let username = payload.username;
    // validate username
    // - min length 4
    // - max length 64
    // - just be from chars a-z, A-Z, 0-9, _, -
    let re = regex::Regex::new(r"^[a-zA-Z0-9_-]{4,64}$").unwrap();
    if !re.is_match(&username) {
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("signup_error".to_string(), "Invalid username".to_string());
        return Ok(Html(
            ctx.environment.get_template("login.html")?.render(data)?,
        ));
    }

    let password = payload.password;
    // validate password
    // - min length 6
    // - max length 128
    let re = regex::Regex::new(r"^[a-zA-Z0-9_-]{6,128}$").unwrap();
    if !re.is_match(&password) {
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("signup_error".to_string(), "Invalid password".to_string());

        return Ok(Html(
            ctx.environment.get_template("login.html")?.render(data)?,
        ));
    }

    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let user_id = match ctx.db.add_user(&username, &salt, &password).await {
        Ok(user_id) => user_id,
        Err(_) => {
            let mut data: HashMap<String, String> = HashMap::new();
            data.insert(
                "signup_error".to_string(),
                "User already exists".to_string(),
            );
            return Ok(Html(
                ctx.environment.get_template("login.html")?.render(data)?,
            ));
        }
    };
    log::info!("user with id: {:?} signed up", user_id);

    let timestamp = chrono::Utc::now().timestamp();
    let jwt_access_token = ctx
        .jwt
        .create_token(user_id, TokenType::Access, timestamp + 90 * 24 * 60 * 60)
        .await?;

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("access_token".to_string(), jwt_access_token);
    data.insert("username".to_string(), username);
    data.insert("user_id".to_string(), user_id.to_string());

    Ok(Html(
        ctx.environment.get_template("login.html")?.render(data)?,
    ))
}
