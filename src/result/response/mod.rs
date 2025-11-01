use axum::{Json, response::IntoResponse};
use serde_json::json;

pub mod error;

pub type ResponseResult = Result<Response, error::ResponseError>;

pub enum Response {
    Json(serde_json::Value),
    Text(String),
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::Json(json) => Json(json),
            Response::Text(text) => Json(json!({ "message": text })),
        }
        .into_response()
    }
}
