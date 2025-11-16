// tests/swap_tests.rs
use ethers::prelude::*;
use std::sync::Arc;
use std::env;
use rust_decimal::Decimal;
use anyhow::Result;
use rust_decimal::prelude::FromPrimitive;
use eth_mcp_server::swap::SwapModule;

#[tokio::test]
async fn test_simulate_swap_eth_to_weth() -> Result<()> {
    // 加载环境变量 & 初始化日志
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::try_init();

    // 获取 INFURA RPC URL
    let rpc_url = env::var("INFURA_URL").expect("INFURA_URL not set");

    // 连接 Provider
    let provider = Arc::new(Provider::<Http>::connect(rpc_url.as_str()).await);

    let swap_module = SwapModule::new((*provider).clone());

    // Uniswap V2 Router Sepolia 地址 (示例)
    let router_addr: Address = env::var("UNISWAP_V2_ROUTER")?.parse()?;


    // ETH -> WETH
    let usdc_addr: Address = env::var("USDC")?.parse()?;// Sepolia WETH
    let from_token = Address::zero();


    let amount_in = Decimal::from_f64(0.001).unwrap();
    let slippage = 0.5; // 0.5%

    // 调用 simulate_swap
    let (estimated_output, min_output, gas_estimate) = swap_module
        .swap_tokens(router_addr, from_token, usdc_addr, amount_in, slippage)
        .await?;

    println!("Estimated output: {}", estimated_output);
    println!("Min output (slippage applied): {}", min_output);
    println!("Estimated gas: {}", gas_estimate);

    // 简单断言
    assert!(estimated_output > Decimal::ZERO, "Estimated output should be > 0");
    assert!(min_output <= estimated_output, "Min output should <= estimated output");
    assert!(gas_estimate > Decimal::ZERO, "Gas estimate should be > 0");

    Ok(())
}
