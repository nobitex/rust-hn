use anyhow::anyhow;
use axum::{extract::Query, response::Html};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::{
    db::{PostWrapper, User},
    jwt::JwtData,
    server::{Context, ContextDB},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemGetRequest {
    id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ItemData {
    post: PostWrapper,
    comments: Vec<PostWrapper>,
    user: Option<User>,
}

fn sort_comments(mut posts: Vec<PostWrapper>) -> Vec<PostWrapper> {
    let mut parents = HashMap::<i32, i32>::new();
    let mut paths = HashMap::<i32, VecDeque<i32>>::new();
    for p in posts.iter().rev() {
        if let Some(parent) = p.post.parent_id {
            parents.insert(p.post.id, parent);
        }
        let mut curr = p.post.id;
        let path = paths.entry(p.post.id).or_default();
        path.push_front(curr);
        while let Some(p) = parents.get(&curr) {
            curr = *p;
            path.push_front(curr);
        }
    }
    posts.sort_by_key(|p| paths.get(&p.post.id).unwrap());
    posts
        .iter_mut()
        .for_each(|p| p.depth = paths.get(&p.post.id).unwrap().len() - 1);
    posts
}

pub async fn item_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
    Query(req): Query<ItemGetRequest>,
    jwt_payload: Option<JwtData>,
) -> Result<Html<String>, anyhow::Error> {
    let ctx = ctx.lock().await;

    let user_id = jwt_payload.map(|d| d.user_id);
    let user = if let Some(uid) = user_id {
        Some(ctx.db.get_user_by_id(uid).await?)
    } else {
        None
    };

    let data = ItemData {
        post: ctx
            .db
            .get_posts(user_id, None, Some(req.id), "", 0)
            .await?
            .first()
            .ok_or(anyhow!("Post not found!"))?
            .clone(),
        comments: sort_comments(ctx.db.get_posts(user_id, Some(req.id), None, "", 0).await?),
        user,
    };

    Ok(Html(
        ctx.environment.get_template("item.html")?.render(data)?,
    ))
}
