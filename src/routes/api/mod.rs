use axum::Router;

use crate::service_state::SharedServiceState;

mod v1 {
    pub mod hello;
}

pub fn routes() -> Router<SharedServiceState> {
    Router::new().nest("/v1", Router::new().merge(v1::hello::routes()))
}
