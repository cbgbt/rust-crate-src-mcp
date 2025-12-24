use rmcp::{ServiceExt, model::CallToolRequestParam, transport::TokioChildProcess};
use serde_json::json;
use tokio::process::Command;

#[tokio::test]
#[ignore]
async fn test_get_rust_crate_source() -> anyhow::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--bin").arg("mcp-server");
    let transport = TokioChildProcess::new(cmd)?;
    let client = ().serve(transport).await?;

    let result = client
        .call_tool(CallToolRequestParam {
            name: "get_rust_crate_source".into(),
            arguments: Some(json!({"crate_name": "serde"}).as_object().unwrap().clone()),
        })
        .await?;

    assert!(result.is_error.is_none() || !result.is_error.unwrap());
    let content = &result.content[0];
    let text = match &content.raw {
        rmcp::model::RawContent::Text(text_content) => &text_content.text,
        _ => panic!("expected text content"),
    };
    let data: serde_json::Value = serde_json::from_str(text)?;
    assert_eq!(data["crate_name"], "serde");
    let checkout_path = data["checkout_path"].as_str().unwrap();
    assert!(!checkout_path.is_empty());
    assert!(data["version"].as_str().unwrap().len() > 0);

    let cargo_toml = std::path::Path::new(checkout_path).join("Cargo.toml");
    assert!(
        cargo_toml.exists(),
        "Cargo.toml should exist at {}",
        cargo_toml.display()
    );

    Ok(())
}
