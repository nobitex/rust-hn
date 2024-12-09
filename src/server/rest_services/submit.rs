use anyhow::anyhow;
use axum::{
    body::Body,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use hyper::Response;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    db::{Post, User},
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SubmitPostData {
    title: String,
    link: Option<String>,
    text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SubmitGetData {
    error: Option<String>,
    form: SubmitPostData,
    user: User,
}

impl SubmitPostData {
    fn validate(&mut self) -> Result<(), anyhow::Error> {
        let re = Regex::new(
            r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
        )?;
        if self.title.len() > 90 {
            return Err(anyhow!("Title too long!"));
        }
        if self.title.len() < 4 {
            return Err(anyhow!("Title too short!"));
        }
        if let Some(link) = self.link.clone() {
            if link.is_empty() {
                self.link = None;
            } else if !re.is_match(&link) {
                return Err(anyhow!("Invalid url!"));
            }
        }
        if let Some(txt) = self.text.clone() {
            if txt.is_empty() {
                self.text = None;
            } else if txt.len() > 1024 {
                return Err(anyhow!("Text too long!"));
            } else if txt.len() < 3 {
                return Err(anyhow!("Text too short!"));
            } else if txt.chars().filter(|c| *c == '\n').count() > 16 {
                return Err(anyhow!("Text too tall!"));
            }
        }
        Ok(())
    }
}

pub async fn submit_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    jwt_payload: Option<JwtData>,
) -> Result<Response<Body>, anyhow::Error> {
    if let Some(user_id) = jwt_payload.map(|d| d.user_id) {
        let ctx = ctx.lock().await;
        let user = ctx.db.get_user_by_id(user_id).await?;
        Ok(Html(
            ctx.environment
                .get_template("submit.html")?
                .render(SubmitGetData {
                    error: None,
                    form: Default::default(),
                    user,
                })?,
        )
        .into_response())
    } else {
        Ok(Redirect::to("/login").into_response())
    }
}

pub async fn submit_post_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Form(mut req): Form<SubmitPostData>,
    jwt_payload: Option<JwtData>,
) -> Result<Response<Body>, anyhow::Error> {
    let ctx = ctx.lock().await;
    let user_id = jwt_payload
        .map(|d| d.user_id)
        .ok_or(anyhow!("Not logged in!"))?;
    let user = ctx.db.get_user_by_id(user_id).await?;

    if let Err(e) = req.validate() {
        return Ok(Html(
            ctx.environment
                .get_template("submit.html")?
                .render(SubmitGetData {
                    form: req,
                    error: Some(e.to_string()),
                    user,
                })?,
        )
        .into_response());
    }

    ctx.db
        .submit_post(Post::new(
            user_id,
            None,
            None,
            Some(req.title),
            req.link,
            req.text,
        ))
        .await?;
    Ok(Redirect::to("/").into_response())
}
