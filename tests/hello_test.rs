use pretty_assertions::assert_eq;

mod common;
use common::prelude::*;
use serde_json::json;

#[tokio::test(flavor = "multi_thread")]
async fn test_get_hello() -> anyhow::Result<()> {
    let test_container = setup_service().await?;

    let client = reqwest::Client::new();

    let resp = client
        .get(test_container.format_service_url("api/v1/hello")?)
        .send()
        .await?
        .error_for_status()?;

    let body = resp
        .json::<serde_json::Value>()
        .await?;

    assert_eq!(body, json!({"message": "Hello, World!"}));

    test_container
        .shutdown()
        .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_hello_with_name() -> anyhow::Result<()> {
    let test_container = setup_service().await?;

    let client = reqwest::Client::new();

    let resp = client
        .get(test_container.format_service_url("api/v1/hello/Matt")?)
        .send()
        .await?
        .error_for_status()?;

    let body = resp
        .json::<serde_json::Value>()
        .await?;

    assert_eq!(body, json!({"message": "Hello, Matt!"}));

    test_container
        .shutdown()
        .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_hello_with_name_failure() -> anyhow::Result<()> {
    let test_container = setup_service().await?;

    let client = reqwest::Client::new();

    let resp = client
        .get(test_container.format_service_url("api/v1/hello/a")?)
        .send()
        .await?;

    let body = resp
        .json::<serde_json::Value>()
        .await?;

    assert_eq!(body.get("error"), Some(&json!("validation_error")));

    test_container
        .shutdown()
        .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_post_hello() -> anyhow::Result<()> {
    let test_container = setup_service().await?;

    let client = reqwest::Client::new();

    let resp = client
        .post(test_container.format_service_url("api/v1/hello")?)
        .json(&json!({"name": "Matt"}))
        .send()
        .await?
        .error_for_status()?;

    let body = resp
        .json::<serde_json::Value>()
        .await?;

    assert_eq!(body, json!({"message": "Hello, Matt!"}));

    test_container
        .shutdown()
        .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_post_hello_failure() -> anyhow::Result<()> {
    let test_container = setup_service().await?;

    let client = reqwest::Client::new();

    let resp = client
        .post(test_container.format_service_url("api/v1/hello")?)
        .json(&json!({"name": ""}))
        .send()
        .await?;

    let body = resp
        .json::<serde_json::Value>()
        .await?;

    assert_eq!(body.get("error"), Some(&json!("validation_error")));

    test_container
        .shutdown()
        .await?;

    Ok(())
}
