use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use rate_limited_svc::{Service, config::ServiceConfig};
use tokio::{net::TcpListener, signal::unix::SignalKind, time::timeout, try_join};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, filter::Directive, layer::SubscriberExt, util::SubscriberInitExt};

const SERVICE_PORT_ENV_VAR: &str = "SERVICE_PORT";
const DEFAULT_SERVICE_PORT: u16 = 8080;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    match Cli::parse().command {
        Commands::Run => run_service().await,
    }
}

async fn run_service() -> Result<(), anyhow::Error> {
    let mut tracing_level = LevelFilter::INFO;
    if cfg!(debug_assertions) {
        tracing_level = LevelFilter::DEBUG;
    }

    let default_directive: Directive = format!("{{project_name}}={}", tracing_level).parse()?;

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(default_directive)
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_error::ErrorLayer::default())
        .init();

    let port = std::env::var(SERVICE_PORT_ENV_VAR).map_or(Ok(DEFAULT_SERVICE_PORT), |p| p.parse())?;

    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port))).await?;

    tracing::info!("Starting service at {}", listener.local_addr()?);
    let (service_handle, shutdown_tx) = Service::new(ServiceConfig::default()).start_server(listener)?;

    let mut terminate_signal = tokio::signal::unix::signal(SignalKind::terminate())?;

    tokio::select! {
        _ = terminate_signal.recv() => {
            tracing::info!("Termination signal received, shutting down service.");
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Ctrl-C signal received, shutting down service.");
        }
    }

    shutdown_tx.send(())?;

    timeout(Duration::from_secs(10), async {
        match try_join!(service_handle) {
            Ok(_) => tracing::info!("Service shut down gracefully."),
            Err(e) => tracing::error!("Error during service shutdown: {:?}", e),
        }
    })
    .await
    .map_err(|e| anyhow!("Service shutdown timed out: {:?}", e))?;

    Ok(())
}
