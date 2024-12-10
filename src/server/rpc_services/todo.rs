use std::sync::Arc;

use crate::server::ContextDB;

use super::Context;

use anyhow::Result;
use jsonrpsee::types::Params;
use tokio::sync::Mutex;

pub async fn todo<D: ContextDB + 'static>(
    _ctx: Arc<Arc<Mutex<Context<D>>>>,
    _params: Params<'static>,
) -> Result<String> {
    Ok("Not implemented!".to_string())
}
