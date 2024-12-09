use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use axum::body::Body;
use axum::extract::{FromRequest, Query, Request, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{extract, middleware, Extension, Form, Json, Router};
use hyper::StatusCode;
use mime_guess::from_ext;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use structopt::StructOpt;
use tokio::sync::Mutex;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

use crate::jwt::{auth, JwtData};
use crate::server::{
    comment_get_handler, comment_post_handler, get_chat_messages_handler, get_login_handler,
    get_user_chats_handler, index_handler, item_handler, logout_handler, post_login_handler,
    post_signup_handler, profile_handler, remove_handler, send_chat_message_handler,
    start_chat_handler, submit_handler, submit_post_handler, upvote_post_handler, LoginRequest,
    SignupRequest,
};

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
        .route(
            "/",
            get({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    super::handle_error(
                        async {
                            let jwt_payload = request.extensions().get::<JwtData>().cloned();
                            let query = Query::from_request(request, &State(ctx.clone())).await?;
                            Ok::<Response<Body>, anyhow::Error>(
                                index_handler(ctx.clone(), "", jwt_payload, query)
                                    .await?
                                    .into_response(),
                            )
                        }
                        .await,
                    )
                }
            }),
        )
        .route(
            "/show",
            get({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    super::handle_error(
                        async {
                            let jwt_payload = request.extensions().get::<JwtData>().cloned();
                            let query = Query::from_request(request, &State(ctx.clone())).await?;
                            Ok::<Response<Body>, anyhow::Error>(
                                index_handler(ctx.clone(), "Show SF: ", jwt_payload, query)
                                    .await?
                                    .into_response(),
                            )
                        }
                        .await,
                    )
                }
            }),
        )
        .route(
            "/ask",
            get({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    super::handle_error(
                        async {
                            let jwt_payload = request.extensions().get::<JwtData>().cloned();
                            let query = Query::from_request(request, &State(ctx.clone())).await?;
                            Ok::<Response<Body>, anyhow::Error>(
                                index_handler(ctx.clone(), "Ask SF: ", jwt_payload, query)
                                    .await?
                                    .into_response(),
                            )
                        }
                        .await,
                    )
                }
            }),
        )
        .route(
            "/hire",
            get({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    super::handle_error(
                        async {
                            let jwt_payload = request.extensions().get::<JwtData>().cloned();
                            let query = Query::from_request(request, &State(ctx.clone())).await?;
                            Ok::<Response<Body>, anyhow::Error>(
                                index_handler(ctx.clone(), "Hire SF: ", jwt_payload, query)
                                    .await?
                                    .into_response(),
                            )
                        }
                        .await,
                    )
                }
            }),
        )
        .route(
            "/submit",
            get({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    super::handle_error(submit_handler(ctx.clone(), jwt_payload).await)
                }
            }),
        )
        .route(
            "/remove",
            post({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let referer = request.headers().get("referer").cloned();
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let json_payload = match Form::from_request(request, &State(ctx.clone())).await
                    {
                        Ok(form_payload) => form_payload,
                        Err(e) => {
                            return super::handle_error(Err(anyhow::anyhow!(format!(
                                "Error parsing Form: {}",
                                e
                            ))));
                        }
                    };
                    super::handle_error(
                        remove_handler(ctx.clone(), referer, json_payload, jwt_payload).await,
                    )
                }
            }),
        )
        .route(
            "/item",
            get({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let query_payload =
                        match Query::from_request(request, &State(ctx.clone())).await {
                            Ok(query_payload) => query_payload,
                            Err(e) => {
                                return super::handle_error(Err(anyhow::anyhow!(format!(
                                    "Error parsing Query: {}",
                                    e
                                ))));
                            }
                        };

                    super::handle_error(item_handler(ctx.clone(), query_payload, jwt_payload).await)
                }
            }),
        )
        .route(
            "/submit",
            post({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let json_payload = match Form::from_request(request, &State(ctx.clone())).await
                    {
                        Ok(form_payload) => form_payload,
                        Err(e) => {
                            return super::handle_error(Err(anyhow::anyhow!(format!(
                                "Error parsing Form: {}",
                                e
                            ))));
                        }
                    };
                    super::handle_error(
                        submit_post_handler(ctx.clone(), json_payload, jwt_payload).await,
                    )
                }
            }),
        )
        .route(
            "/upvote",
            post({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let json_payload = match Json::from_request(request, &State(ctx.clone())).await
                    {
                        Ok(json_payload) => json_payload,
                        Err(e) => {
                            return super::handle_error(Err(anyhow::anyhow!(format!(
                                "Error parsing Json: {}",
                                e
                            ))));
                        }
                    };

                    super::handle_error(
                        upvote_post_handler(ctx.clone(), json_payload, jwt_payload).await,
                    )
                }
            }),
        )
        .route(
            "/comment",
            get({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let query_payload =
                        match Query::from_request(request, &State(ctx.clone())).await {
                            Ok(json_payload) => json_payload,
                            Err(e) => {
                                return super::handle_error(Err(anyhow::anyhow!(format!(
                                    "Error parsing Query: {}",
                                    e
                                ))));
                            }
                        };

                    super::handle_error(
                        comment_get_handler(ctx.clone(), query_payload, jwt_payload).await,
                    )
                }
            }),
        )
        .route(
            "/comment",
            post({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let form_payload = match Form::from_request(request, &State(ctx.clone())).await
                    {
                        Ok(form_payload) => form_payload,
                        Err(e) => {
                            return super::handle_error(Err(anyhow::anyhow!(format!(
                                "Error parsing Form: {}",
                                e
                            ))));
                        }
                    };

                    super::handle_error(
                        comment_post_handler(ctx.clone(), form_payload, jwt_payload).await,
                    )
                }
            }),
        )
        .route(
            "/start_chat",
            post({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let json_payload = match Json::from_request(request, &State(ctx.clone())).await
                    {
                        Ok(json_payload) => json_payload,
                        Err(e) => {
                            return super::handle_error(Err(anyhow::anyhow!(format!(
                                "Error parsing Json: {}",
                                e
                            ))));
                        }
                    };

                    super::handle_error(
                        start_chat_handler(ctx.clone(), json_payload, jwt_payload).await,
                    )
                }
            }),
        )
        .route(
            "/get_chat_messages",
            post({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let json_payload = match Json::from_request(request, &State(ctx.clone())).await
                    {
                        Ok(json_payload) => json_payload,
                        Err(e) => {
                            return super::handle_error(Err(anyhow::anyhow!(format!(
                                "Error parsing Json: {}",
                                e
                            ))));
                        }
                    };

                    super::handle_error(
                        get_chat_messages_handler(ctx.clone(), json_payload, jwt_payload).await,
                    )
                }
            }),
        )
        .route(
            "/send_chat_message",
            post({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    let json_payload = match Json::from_request(request, &State(ctx.clone())).await
                    {
                        Ok(json_payload) => json_payload,
                        Err(e) => {
                            return super::handle_error(Err(anyhow::anyhow!(format!(
                                "Error parsing Json: {}",
                                e
                            ))));
                        }
                    };

                    super::handle_error(
                        send_chat_message_handler(ctx.clone(), json_payload, jwt_payload).await,
                    )
                }
            }),
        )
        .route(
            "/get_user_chats",
            post({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    super::handle_error(get_user_chats_handler(ctx.clone(), jwt_payload).await)
                }
            }),
        )
        .route(
            "/profile",
            get({
                let ctx = ctx.clone();
                move |request: Request| async move {
                    let jwt_payload = request.extensions().get::<JwtData>().cloned();
                    super::handle_error(profile_handler(ctx.clone(), jwt_payload).await)
                }
            }),
        )
        .route(
            "/login",
            get({
                let ctx = ctx.clone();
                move || async move { super::handle_error(get_login_handler(ctx.clone()).await) }
            }),
        )
        .route(
            "/logout",
            get({
                let ctx = ctx.clone();
                move || async move { super::handle_error(logout_handler(ctx.clone()).await) }
            }),
        )
        .route(
            "/signup",
            post({
                let ctx = ctx.clone();
                move |Form(req): Form<SignupRequest>| async move {
                    super::handle_error(post_signup_handler(ctx.clone(), extract::Form(req)).await)
                }
            }),
        )
        .route(
            "/login",
            post({
                let ctx = ctx.clone();
                move |Form(req): Form<LoginRequest>| async move {
                    super::handle_error(post_login_handler(ctx.clone(), extract::Form(req)).await)
                }
            }),
        )
        .route(
            "/assets/*file",
            get(|params: extract::Path<String>| async move {
                let file_name = params.as_str();
                serve_file(file_name).unwrap()
            }),
        )
        .route(
            "/favicon.ico",
            get(|| async move { serve_file("favicon.ico").unwrap() }),
        )
        .route(
            "/robots.txt",
            get(|| async move { Response::new(Body::from("User-agent: *\nAllow: /\n")) }),
        )
        .layer(Extension(ctx.clone()))
        .layer(middleware::from_fn_with_state(ctx.clone(), auth))
        .layer(CookieManagerLayer::new())
        .with_state(ctx.clone());

    let app_with_middleware = app.layer(CorsLayer::permissive());

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), opt.rest_server_port);
    log::info!("Running Rest server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app_with_middleware.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn serve_file(path: &str) -> Result<Response<Body>, anyhow::Error> {
    let content: Vec<u8> = match std::fs::read(Path::new("assets").join(path)) {
        Ok(content) => content,
        Err(e) => {
            log::error!("Error reading file: {}", e);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Error reading file"))?);
        }
    };

    let mime_type = match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some(ext) => from_ext(ext).first_or_octet_stream().as_ref().to_string(),
        None => "application/octet_stream".to_string(),
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", mime_type)
        .header("Cache-Control", "max-age=2592000, public")
        .body(Body::from(content))?;

    Ok(response)
}
