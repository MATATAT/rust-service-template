use serde::{Deserialize, Deserializer};
use serde_yml::Value;
use validator::Validate;

#[derive(Debug)]
pub struct ServiceConfigWrapper(pub ServiceConfig);

#[derive(Debug, Default, Validate, Deserialize)]
pub struct ServiceConfig {
    #[serde(rename = "hello-name", default = "default_hello_name")]
    #[validate(length(min = 2, max = 100))]
    pub hello_name: String,
}

impl<'de> Deserialize<'de> for ServiceConfigWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let nested = value
            .get("svc")
            .and_then(|svc| svc.get("svc-template"))
            .ok_or_else(|| serde::de::Error::custom("Failed to find svc.svc-template"))?;
        let service = ServiceConfig::deserialize(nested).map_err(serde::de::Error::custom)?;

        Ok(ServiceConfigWrapper(service))
    }
}

fn default_hello_name() -> String {
    "World".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deserialize_service_config() {
        let yaml_data = r#"
  svc:
    hello-name: "World"
        "#;

        let config: ServiceConfigWrapper = serde_yml::from_str(yaml_data).unwrap();
        assert_eq!(config.0.hello_name, "World");
    }

    #[test]
    fn test_deserialize_service_config_with_default() {
        let yaml_data = r#"
  svc: {}
        "#;

        let config: ServiceConfigWrapper = serde_yml::from_str(yaml_data).unwrap();
        assert_eq!(config.0.hello_name, "World");
    }

    #[test]
    fn test_deserialize_service_config_missing_svc() {
        let yaml_data = r#"
  other:
    hello-name: "World"
        "#;

        let config: ServiceConfigWrapper = serde_yml::from_str(yaml_data).unwrap();
        assert_eq!(config.0.hello_name, "World");
    }

    #[test]
    fn test_deserialize_service_config_empty() {
        let yaml_data = r#""#;

        let config: ServiceConfigWrapper = serde_yml::from_str(yaml_data).unwrap();
        assert_eq!(config.0.hello_name, "World");
    }
}
