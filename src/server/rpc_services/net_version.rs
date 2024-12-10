use std::sync::Arc;

use anyhow::Result;
use jsonrpsee::types::Params;
use tokio::sync::Mutex;

use super::Context;
use crate::server::ContextDB;

pub async fn net_version<D: ContextDB + 'static>(
    ctx: Arc<Arc<Mutex<Context<D>>>>,
    _params: Params<'static>,
) -> Result<String> {
    Ok(format!("0x0"))
}
