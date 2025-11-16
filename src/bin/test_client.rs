use dotenv::dotenv;
use ethers::addressbook::Address;
use rmcp::{
    model::CallToolRequestParam,
    service::ServiceExt,
    transport::{ConfigureCommandExt, TokioChildProcess},
    RmcpError,
};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use tokio::process::Command;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), RmcpError> {
    // 初始化环境变量
    dotenv().ok();

    // ===== Logging =====
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_PKG_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ===== Start local MCP Server =====
    let child = TokioChildProcess::new(Command::new("cargo").configure(|cmd| {
        cmd.arg("run").arg("--bin").arg("eth-mcp-server");
    }))
    .map_err(RmcpError::transport_creation::<TokioChildProcess>)?;

    // ===== Create MCP Client =====
    let client = ().serve(child).await?;

    info!("Connected to server: {:#?}", client.peer_info());

    // ===== List tools =====
    let tools = client.list_tools(Default::default()).await?;
    info!("Available tools: {:#?}", tools);

    // ===== Call tool: get_balance =====
    let wallet = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS not set");
    let usdc_contract = env::var("USDC").expect("USDC_ADDRESS not set");
    let balance_result = client
        .call_tool(CallToolRequestParam {
            name: "get_balance".into(), // String -> Cow<'_, str>
            arguments: serde_json::json!({
                "address": wallet,
                "token": usdc_contract
            })
            .as_object()
            .cloned(),
        })
        .await?;

    info!("get_balance result: {:#?}", balance_result);

    // 查询 ETH/USD
    let eth_price_result = client
        .call_tool(CallToolRequestParam {
            name: "get_price".into(),
            arguments: serde_json::json!({
                "token": null  // None 表示默认 ETH/USD
            })
            .as_object()
            .cloned(),
        })
        .await?;

    info!("get_price ETH/USD result: {:#?}", eth_price_result);

    // 查询 BTC/USD
    let btc_price_result = client
        .call_tool(CallToolRequestParam {
            name: "get_price".into(),
            arguments: serde_json::json!({
                "token": "BTC"
            })
            .as_object()
            .cloned(),
        })
        .await?;

    info!("get_price BTC/USD result: {:#?}", btc_price_result);

    let swap_result = client
        .call_tool(CallToolRequestParam {
            name: "swap_tokens".into(),
            arguments: serde_json::json!({"from_token":"ETH","to_token":"USDC","amount_in":"0.001","slippage":0.5}).as_object().cloned(),
        })
        .await?;

    info!("swap_tokens result: {:#?}", swap_result);

    // ===== Shutdown client =====
    client.cancel().await?;
    Ok(())
}
