use std::{sync::Arc, time::Duration};

use axum::{Router, extract::Request, routing::get};
use tokio::{net::TcpListener, sync::broadcast, task::JoinHandle};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, normalize_path::NormalizePathLayer, timeout::TimeoutLayer, trace::TraceLayer};

pub mod config;
mod routes {
    pub mod api;
    pub mod health;
}
pub mod result;
pub(crate) mod service_state;

use crate::{config::ServiceConfig, result::ServiceResult, service_state::ServiceState};

pub struct Service {
    config: Arc<ServiceConfig>,
}

impl Service {
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn start_server(
        &self,
        listener: TcpListener,
    ) -> ServiceResult<(JoinHandle<ServiceResult<()>>, broadcast::Sender<()>)> {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        let config = self.config.clone();

        let server_handle = tokio::spawn(async move {
            let router = create_http_router(config);

            let router_with_middleware = ServiceBuilder::new()
                .layer(NormalizePathLayer::trim_trailing_slash())
                .service(router);

            let service = axum::ServiceExt::<Request>::into_make_service(router_with_middleware);

            let server = axum::serve(listener, service).with_graceful_shutdown(shutdown_signal(shutdown_rx));

            match server.await {
                Ok(_) => Ok(()),
                Err(e) => {
                    tracing::error!("Server encountered an error: {:?}", e);

                    Err(e.into())
                }
            }
        });

        Ok((server_handle, shutdown_tx))
    }
}

fn create_http_router(config: Arc<ServiceConfig>) -> Router {
    let service_state: ServiceState = config.into();

    Router::new()
        .route("/health", get(routes::health::check))
        .nest("/api", routes::api::routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .with_state(service_state.into())
}

async fn shutdown_signal(mut shutdown_rx: broadcast::Receiver<()>) {
    shutdown_rx
        .recv()
        .await
        .expect("Failed to receive shutdown signal");

    tracing::info!("Shutdown signal received, stopping server.");
}
