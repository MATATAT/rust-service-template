use std::sync::Arc;

use crate::config::ServiceConfig;

pub type SharedServiceState = Arc<ServiceState>;

#[derive(Debug, Clone)]
pub struct ServiceState {
    response_name: String,
}

impl ServiceState {
    pub fn new(response_name: String) -> Self {
        Self { response_name }
    }

    pub fn response_name(&self) -> &str {
        &self.response_name
    }
}

impl From<Arc<ServiceConfig>> for ServiceState {
    fn from(config: Arc<ServiceConfig>) -> Self {
        ServiceState::new(config.hello_name.clone())
    }
}
