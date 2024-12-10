use crate::db::DB;

mod rpc_services;
pub use rpc_services::*;

mod jsonrpc;
pub use jsonrpc::*;

pub trait ContextDB: DB + Send + Sync {}

pub struct Context<D: ContextDB> {
    pub db: D,
}
