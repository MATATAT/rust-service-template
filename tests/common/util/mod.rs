use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use anyhow::{Context, bail};
use svc_template::config::ServiceConfig;
use tokio::net::TcpListener;
use url::Url;

use crate::common::util::test_container::TestContainer;

pub mod test_container;

pub async fn setup_service() -> anyhow::Result<TestContainer> {
    setup_service_with_config(ServiceConfig::default()).await
}

pub async fn setup_service_with_config(config: ServiceConfig) -> anyhow::Result<TestContainer> {
    // 1) Bind to an ephemeral port
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
        .await
        .context("bind test listener")?;
    let service_address = create_uri(
        listener
            .local_addr()
            .context("local_addr")?,
    );

    // 2) Start the server
    let service = svc_template::Service::new(config);
    let (service_handle, shutdown_tx) = service.start_server(listener)?;

    // 3) Wait for readiness (simple ping loop against /health)
    wait_until_ready(&service_address)
        .await
        .context("Server was not ready in time")?;

    Ok(TestContainer::new(service_address, service_handle, shutdown_tx))
}

async fn wait_until_ready(url: &Url) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let url = url
        .join("/health")
        .context("Cannot create health URL")?;
    let start = tokio::time::Instant::now();
    let deadline = start + Duration::from_secs(5);

    loop {
        let client = client.clone();
        let url = url.clone();

        if tokio::time::Instant::now() >= deadline {
            bail!("server not ready after 5s")
        }
        match client.get(url).send().await {
            Ok(r) if r.status().is_success() => return Ok(()),
            _ => tokio::time::sleep(Duration::from_millis(50)).await,
        }
    }
}

fn create_uri(addr: SocketAddr) -> Url {
    Url::parse(&format!("http://{addr}")).expect("invalid URL")
}
