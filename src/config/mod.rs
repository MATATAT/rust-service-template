use std::{env, io::BufReader};
use tokio::fs::File;

pub mod service_config;

pub use service_config::ServiceConfig;
use validator::Validate;

use crate::{config::service_config::ServiceConfigWrapper, result::ServiceResult};

pub async fn load_config() -> ServiceResult<ServiceConfig> {
    let config_path = env::var("APP_CONFIG_PATH").expect("APP_CONFIG_PATH must be set");
    let config_file = File::open(&config_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to open config file at {}: {}", &config_path, e))?;
    let reader = BufReader::new(config_file.into_std().await);
    let config: ServiceConfigWrapper = serde_yml::from_reader(reader)?;

    if let Err(e) = config.0.validate() {
        return Err(e.into());
    }

    Ok(config.0)
}
