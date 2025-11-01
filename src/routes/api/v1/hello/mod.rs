use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use validator::Validate;

use crate::{
    result::response::{Response, ResponseResult},
    routes::api::v1::hello::dto::HelloRequest,
    service_state::SharedServiceState,
};

mod dto;

pub fn routes() -> Router<SharedServiceState> {
    Router::new().nest(
        "/hello",
        Router::new()
            .route("/", get(get_hello_handler))
            .route("/{name}", get(get_hello_name_handler))
            .route("/", post(post_hello_handler)),
    )
}

#[axum::debug_handler]
async fn get_hello_handler(State(service_state): State<SharedServiceState>) -> ResponseResult {
    Ok(Response::Text(format!("Hello, {}!", service_state.response_name())))
}

#[axum::debug_handler]
async fn get_hello_name_handler(
    State(service_state): State<SharedServiceState>,
    Path(name): Path<String>,
) -> ResponseResult {
    Ok(Response::Text(format!(
        "Hello, {}! (from {})",
        name,
        service_state.response_name()
    )))
}

#[axum::debug_handler]
async fn post_hello_handler(
    State(service_state): State<SharedServiceState>,
    Json(payload): Json<HelloRequest>,
) -> ResponseResult {
    payload.validate()?;

    Ok(Response::Text(format!(
        "Hello, {}! (from {})",
        payload.name,
        service_state.response_name()
    )))
}
