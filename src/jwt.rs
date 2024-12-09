use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use structopt::StructOpt;
use tokio::sync::Mutex;
use tower_cookies::Cookies;

use crate::server::{logout_handler, Context, ContextDB};

#[derive(Clone)]
pub struct Jwt {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

#[derive(Debug, Clone, StructOpt)]
pub struct JwtOpts {
    #[structopt(long, default_value = "very-secret-phrase", env = "JWT_SECRET")]
    pub jwt_secret: String,
}

impl Jwt {
    pub fn new(opt: JwtOpts) -> Self {
        Self {
            encoding: EncodingKey::from_secret(opt.jwt_secret.as_bytes()),
            decoding: DecodingKey::from_secret(opt.jwt_secret.as_bytes()),
        }
    }

    pub async fn create_token(
        &self,
        user_id: i32,
        token_type: TokenType,
        exp: i64,
    ) -> Result<String, anyhow::Error> {
        let jwt_data = JwtData {
            user_id,
            token_type,
            exp,
        };
        jsonwebtoken::encode(&jsonwebtoken::Header::default(), &jwt_data, &self.encoding)
            .map_err(|_| AuthError::TokenCreation.into())
    }
}

pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    #[allow(dead_code)]
    MissingCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl Into<anyhow::Error> for AuthError {
    fn into(self) -> anyhow::Error {
        match self {
            AuthError::WrongCredentials => anyhow::anyhow!("Wrong credentials"),
            AuthError::MissingCredentials => anyhow::anyhow!("Missing credentials"),
            AuthError::TokenCreation => anyhow::anyhow!("Token creation error"),
            AuthError::InvalidToken => anyhow::anyhow!("Invalid token"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtData {
    pub user_id: i32,
    pub token_type: TokenType,
    pub exp: i64,
}

pub async fn auth<D: ContextDB>(
    State(ctx): State<Arc<Mutex<Context<D>>>>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let token: Option<String> = cookies
        .get("access_token")
        .and_then(|c| c.value().parse().ok())
        .or_else(|| {
            request
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.strip_prefix("Bearer ").map(|s| s.to_string()))
        });
    let token: Option<&str> = token.as_deref();

    if token.is_none() {
        // return Err(AuthError::MissingCredentials.into_response());
        return Ok(next.run(request).await);
    }

    let token = match token.ok_or(AuthError::InvalidToken) {
        Ok(token) => token,
        Err(e) => return Err(e.into_response()),
    };

    let jwt = ctx.lock().await.jwt.clone();
    let jwt_data =
        match jsonwebtoken::decode::<JwtData>(&token, &jwt.decoding, &Validation::default()) {
            Ok(token_data) => {
                if token_data.claims.exp < chrono::Utc::now().timestamp() {
                    return Ok(logout_handler(ctx.clone()).await.unwrap().into_response());
                }
                token_data.claims
            }
            Err(_) => {
                // return Err(AuthError::WrongCredentials.into_response());
                return Ok(logout_handler(ctx.clone()).await.unwrap().into_response());
            }
        };

    request.extensions_mut().insert(jwt_data);

    Ok(next.run(request).await)
}
