use axum::response::Html;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::server::{Context, ContextDB};

pub async fn logout_handler<D: ContextDB>(
    ctx: Arc<Mutex<Context<D>>>,
) -> Result<Html<String>, anyhow::Error> {
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("logout".to_string(), "true".to_string());
    Ok(Html(
        ctx.lock()
            .await
            .environment
            .get_template("login.html")?
            .render(data)?,
    ))
}
