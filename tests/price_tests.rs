// tests/price_tests.rs
use ethers::prelude::*;
use rust_decimal::Decimal;
use std::env;
use std::sync::Arc;
use eth_mcp_server::price::PriceModule;

#[tokio::test]
async fn test_price_module() {
    // 初始化环境变量和日志
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::try_init();

    // RPC 连接
    let rpc_url = env::var("INFURA_URL").expect("INFURA_URL not set");
    let provider = Arc::new(Provider::<Http>::connect(rpc_url.as_str()).await);

    // PriceModule 实例
    let price_module = PriceModule::new((*provider).clone());

    // 查询 ETH/USD
    let eth_price: Decimal = price_module.get_price(None).await.unwrap();
    println!("ETH/USD price: {}", eth_price);

    // 查询 DAI/USD
    // let dai_price: Decimal = price_module.get_price(Some("DAI")).await.unwrap();
    // println!("DAI/USD price: {}", dai_price);

    // 查询 UNI/ETH
    // let uni_price: Decimal = price_module.get_price(Some("UNI")).await.unwrap();
    // println!("UNI/ETH price: {}", uni_price);

    // 查询 USDC/USD
    // let usdc_price: Decimal = price_module.get_price(Some("USDC")).await.unwrap();
    // println!("USDC/USD price: {}", usdc_price);

    // 查询 BTC/USD
    let btc_price: Decimal = price_module.get_price(Some("BTC")).await.unwrap();
    println!("BTC/USD price: {}", btc_price);

    // 简单断言
    assert!(eth_price > Decimal::new(0, 0));
    // assert!(dai_price > Decimal::new(0, 0));
    assert!(btc_price > Decimal::new(0, 0));
}
