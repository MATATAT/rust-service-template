use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    healthy: bool,
}

pub(crate) async fn check() -> impl IntoResponse {
    let health = Health { healthy: true };

    (StatusCode::OK, Json(health))
}
