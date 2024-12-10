use std::sync::Arc;

use anyhow::{Ok, Result};
use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee::types::ErrorCode;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use structopt::StructOpt;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use super::{Context, ContextDB};

#[derive(Debug, Clone, StructOpt)]
pub struct JsonrpcOpt {
    #[structopt(long, default_value = "8891", env = "JSONRPC_PORT")]
    pub jsonrpc_server_port: u16,
}

fn anyhow_to_rpc_error(e: anyhow::Error) -> ErrorCode {
    log::error!("RPC Error: {}", e);
    ErrorCode::InternalError
}

pub async fn jsonrpc_server<D: ContextDB + 'static>(
    ctx: Arc<Mutex<Context<D>>>,
    opt: JsonrpcOpt,
) -> Result<()> {
    let cors = CorsLayer::new().allow_methods(Any);
    let middleware = tower::ServiceBuilder::new().layer(cors);
    let rpc_addr = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::UNSPECIFIED,
        opt.jsonrpc_server_port,
    ));
    let server = Server::builder()
        .set_http_middleware(middleware)
        .build(rpc_addr)
        .await?;
    let mut module = RpcModule::new(ctx);

    module.register_async_method("net_version", move |params, ctx, _| async move {
        log::info!("net_version! {:?}", params);
        super::rpc_services::net_version(ctx, params)
            .await
            .map_err(anyhow_to_rpc_error)
    })?;

    for method_name in [
        "debug_getBadBlocks",
        "debug_getRawBlock",
        "debug_getRawHeader",
        "debug_getRawReceipts",
        "debug_getRawTransaction",
        "engine_exchangeCapabilities",
        "engine_exchangeTransitionConfigurationV1",
        "engine_forkchoiceUpdatedV1",
        "engine_forkchoiceUpdatedV2",
        "engine_forkchoiceUpdatedV3",
        "engine_getPayloadBodiesByHashV1",
        "engine_getPayloadBodiesByHashV2",
        "engine_getPayloadBodiesByRangeV1",
        "engine_getPayloadBodiesByRangeV2",
        "engine_getPayloadV1",
        "engine_getPayloadV2",
        "engine_getPayloadV3",
        "engine_getPayloadV4",
        "engine_newPayloadV1",
        "engine_newPayloadV2",
        "engine_newPayloadV3",
        "engine_newPayloadV4",
        "eth_accounts",
        "eth_blobBaseFee",
        "eth_coinbase",
        "eth_createAccessList",
        "eth_getBlockByHash",
        "eth_getBlockReceipts",
        "eth_getBlockTransactionCountByHash",
        "eth_getBlockTransactionCountByNumber",
        "eth_getCode",
        "eth_getFilterChanges",
        "eth_getFilterLogs",
        "eth_getLogs",
        "eth_getProof",
        "eth_getStorageAt",
        "eth_getTransactionByBlockHashAndIndex",
        "eth_getTransactionByBlockNumberAndIndex",
        "eth_getUncleCountByBlockHash",
        "eth_getUncleCountByBlockNumber",
        "eth_maxPriorityFeePerGas",
        "eth_newBlockFilter",
        "eth_newFilter",
        "eth_newPendingTransactionFilter",
        "eth_sign",
        "eth_signTransaction",
        "eth_syncing",
        "eth_uninstallFilter",
    ] {
        module.register_async_method(method_name, move |params, ctx, _| async move {
            log::info!("{}! {:?}", method_name, params);
            super::rpc_services::todo(ctx, params)
                .await
                .map_err(anyhow_to_rpc_error)
        })?;
    }

    let addr = server.local_addr()?;
    log::info!("Running RPC server on: {}", addr);
    let handle = server.start(module);
    handle.stopped().await;

    Ok(())
}
