// tests/swap_tests.rs
use ethers::prelude::*;
use std::sync::Arc;
use std::env;
use rust_decimal::Decimal;
use anyhow::Result;
use rust_decimal::prelude::FromPrimitive;
use eth_mcp_server::config::AppConfig;
use eth_mcp_server::swap::SwapModule;

#[tokio::test]
async fn test_simulate_swap_eth_to_weth() -> Result<()> {
    // 加载环境变量 & 初始化日志
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::try_init();

    // 初始化配置
    let config = AppConfig::load();

    // 获取 INFURA RPC URL
    let rpc_url = env::var("INFURA_URL").expect("INFURA_URL not set");

    // 连接 Provider
    let provider = Arc::new(Provider::<Http>::connect(rpc_url.as_str()).await);

    let swap_module = SwapModule::new((*provider).clone(),config);

    let from_token = "ETH";      // 原生 ETH
    let to_token = "USDC";       // 目标 ERC20 token

    let amount_in = Decimal::from_f64(0.001).unwrap();
    let slippage = 0.5; // 0.5%

    // 调用模拟 swap
    let (estimated_output, gas_estimate) = swap_module
        .swap_tokens(from_token, to_token, amount_in, slippage)
        .await?;


    println!("Estimated output: {}", estimated_output);
    println!("Estimated gas: {}", gas_estimate);

    // 简单断言
    assert!(estimated_output > Decimal::ZERO, "Estimated output should be > 0");
    assert!(gas_estimate > Decimal::ZERO, "Gas estimate should be > 0");

    Ok(())
}
