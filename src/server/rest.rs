use std::sync::Arc;

use anyhow::{Ok, Result};
use axum::routing::{get, post};
use axum::{Extension, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use structopt::StructOpt;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use super::{Context, ContextDB};

#[derive(Debug, Clone, StructOpt)]
pub struct RestOpt {
    #[structopt(long, default_value = "8888", env = "REST_PORT")]
    pub rest_server_port: u16,
}

pub async fn rest_server<D: ContextDB + 'static>(
    ctx: Arc<Mutex<Context<D>>>,
    opt: RestOpt,
) -> Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World! GET Request" }))
        .route(
            "/",
            post({
                let ctx = ctx.clone();
                move || async move {
                    let ctx = ctx.lock().await;
                    let users = ctx.db.get_users().await.unwrap();

                    format!("Users: {:?}", users)
                }
            }),
        )
        .layer(Extension(ctx));
    let app_with_middleware = app.layer(CorsLayer::permissive());

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), opt.rest_server_port);
    log::info!("Running Rest server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app_with_middleware.into_make_service())
        .await
        .unwrap();

    Ok(())
}
