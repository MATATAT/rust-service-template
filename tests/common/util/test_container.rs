use std::time::Duration;

use anyhow::Context;
use tokio::{sync::broadcast, task::JoinHandle, time::timeout};
use url::Url;

pub struct TestContainer {
    service_address: Url,
    service_handle: JoinHandle<anyhow::Result<()>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl TestContainer {
    pub fn new(
        service_address: Url,
        service_handle: JoinHandle<anyhow::Result<()>>,
        shutdown_tx: broadcast::Sender<()>,
    ) -> Self {
        Self {
            service_address,
            service_handle,
            shutdown_tx,
        }
    }

    pub async fn shutdown(self) -> anyhow::Result<()> {
        self.shutdown_tx.send(())?;

        timeout(Duration::from_secs(5), self.service_handle)
            .await
            .context("server did not shut down within timeout")???;

        Ok(())
    }

    pub fn format_service_url(&self, path: &str) -> anyhow::Result<Url> {
        self.service_address
            .join(path)
            .context("Cannot format service URL")
    }
}
