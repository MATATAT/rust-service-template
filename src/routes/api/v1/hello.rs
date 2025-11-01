use axum::{Router, extract::State, routing::get};

use crate::{
    result::response::{Response, ResponseResult},
    service_state::SharedServiceState,
};

pub fn routes() -> Router<SharedServiceState> {
    Router::new().nest("/hello", Router::new().route("/", get(hello_handler)))
}

#[axum::debug_handler]
async fn hello_handler(State(service_state): State<SharedServiceState>) -> ResponseResult {
    Ok(Response::Text(format!("Hello, {}!", service_state.response_name())))
}
