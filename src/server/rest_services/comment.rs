use anyhow::anyhow;
use axum::{
    body::Body,
    extract::Query,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use hyper::Response;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    db::{Post, PostWrapper, User},
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CommentPostRequest {
    post_id: i32,
    parent_id: Option<i32>,
    text: String,
}

impl CommentPostRequest {
    fn validate(&self) -> Result<(), anyhow::Error> {
        if self.text.len() > 1024 {
            return Err(anyhow!("Comment too long!"));
        }
        if self.text.len() < 3 {
            return Err(anyhow!("Comment too short!"));
        }
        if self.text.chars().filter(|c| *c == '\n').count() > 16 {
            return Err(anyhow!("Comment too tall!"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentGetRequest {
    root: i32,
    id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentGetData {
    error: Option<String>,
    post: PostWrapper,
    form: CommentPostRequest,
    user: Option<User>,
}

pub async fn comment_post_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Form(req): Form<CommentPostRequest>,
    jwt_payload: Option<JwtData>,
) -> Result<Response<Body>, anyhow::Error> {
    let ctx = &mut ctx.lock().await;
    let user_id = jwt_payload
        .map(|d| d.user_id)
        .ok_or(anyhow!("Not logged in!"))?;
    let user = ctx.db.get_user_by_id(user_id).await?;

    if let Err(e) = req.validate() {
        let res = if req.parent_id.is_none() {
            ctx.db
                .get_posts(Some(user_id), None, Some(req.post_id), "", 0)
        } else {
            ctx.db
                .get_posts(Some(user_id), Some(req.post_id), req.parent_id, "", 0)
        }
        .await?;

        let data = CommentGetData {
            error: Some(e.to_string()),
            post: res.first().ok_or(anyhow!("Post not found!"))?.clone(),
            form: req,
            user: Some(user),
        };

        return Ok(
            Html(ctx.environment.get_template("comment.html")?.render(data)?).into_response(),
        );
    }

    ctx.db
        .submit_post(Post::new(
            user_id,
            Some(req.post_id),
            req.parent_id,
            None,
            None,
            Some(req.text),
        ))
        .await?;
    Ok(Redirect::to(&format!("/item?id={}", req.post_id)).into_response())
}

pub async fn comment_get_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Query(req): Query<CommentGetRequest>,
    jwt_payload: Option<JwtData>,
) -> Result<Response<Body>, anyhow::Error> {
    let ctx = &mut ctx.lock().await;
    let user_id = jwt_payload.map(|d| d.user_id);
    let user = if let Some(uid) = user_id {
        Some(ctx.db.get_user_by_id(uid).await?)
    } else {
        return Ok(Redirect::to("/login").into_response());
    };

    let data = CommentGetData {
        error: None,
        post: ctx
            .db
            .get_posts(user_id, Some(req.root), Some(req.id), "", 0)
            .await?
            .first()
            .ok_or(anyhow!("Post not found!"))?
            .clone(),
        form: Default::default(),
        user,
    };

    Ok(Html(ctx.environment.get_template("comment.html")?.render(data)?).into_response())
}
