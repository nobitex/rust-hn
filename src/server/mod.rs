use std::fmt::Debug;

use crate::db::DB;

mod rpc_services;
pub use rpc_services::*;

mod jsonrpc;
pub use jsonrpc::*;

mod rest_services;
pub use rest_services::*;

mod rest;
pub use rest::*;

mod utils;
pub use utils::*;

pub trait ContextDB: DB + Send + Sync {}

pub struct Context<D: ContextDB> {
    pub db: D,
}

impl<D: ContextDB> Debug for Context<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").finish()
    }
}
