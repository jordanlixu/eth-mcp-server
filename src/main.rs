use std::env;
use std::sync::Arc;
use tokio::io::{stdin, stdout};
use ethers::providers::{Provider, Http, ProviderExt};
use dotenv::dotenv;
use rmcp::ServiceExt;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

mod balance;
mod price;
mod swap;
mod service;

use crate::service::TokenService;
use balance::BalanceModule;
use price::PriceModule;
use swap::SwapModule;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化环境变量
    dotenv().ok();

    // 初始化日志
    tracing_subscriber::fmt::init();


    // 获取 RPC URL
    let rpc_url = env::var("INFURA_URL").expect("ETH_NODE_URL not set");

    // 创建 Provider (ethers 2.x 推荐用 connect)
    let provider = Provider::<Http>::connect(rpc_url.as_str()).await;

    // 初始化各模块（模块内部会把 provider 包成 Arc）
    let balance_module = Arc::new(BalanceModule::new(provider.clone()));
    let price_module = Arc::new(PriceModule::new(provider.clone()));
    let swap_module = Arc::new(SwapModule::new(provider));

    let service = TokenService::new(balance_module, price_module, swap_module);

    // 构建 transport (stdin/stdout)
    let transport = (stdin(), stdout());

    // 启动 MCP server
    let server = service.serve(transport).await?;
    server.waiting().await?;

    Ok(())
}
