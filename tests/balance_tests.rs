// tests/balance_tests.rs
use ethers::prelude::*;
use rust_decimal::Decimal;
use std::{env, sync::Arc};
use anyhow::Result;
use eth_mcp_server::balance::BalanceModule;

#[tokio::test]
async fn test_eth_balance() -> Result<()> {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::try_init();

    let rpc_url = env::var("INFURA_URL").expect("INFURA_URL not set");

    let provider = Arc::new(Provider::<Http>::connect(rpc_url.as_str()).await);

    let balance_module = BalanceModule::new((*provider).clone());

    let wallet: Address = env::var("WALLET_ADDRESS")?.parse()?;

    let balance: Decimal = balance_module.get_balance(wallet, None).await?;
    println!("ETH Balance: {}", balance);

    Ok(())
}

#[tokio::test]
async fn test_erc20_balance() -> Result<()> {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::try_init();

    let rpc_url = env::var("INFURA_URL").expect("INFURA_URL not set");
    let provider = Arc::new(Provider::<Http>::connect(rpc_url.as_str()).await);
    let balance_module = BalanceModule::new((*provider).clone());

    let wallet: Address = env::var("WALLET_ADDRESS")?.parse()?;

    let uni_contract: Address = env::var("UNI")?.parse()?;


    let balance: Decimal = balance_module.get_balance(wallet, Some(uni_contract)).await?;
    println!("UNI Balance: {}", balance);

    Ok(())
}

// 1️⃣ Burned ETH (BETH)
// Contract: 0x716bC7e331c9Da551e5Eb6A099c300db4c08E994
// Description: Represents ETH permanently removed from circulating supply (burn mechanism tracking).
//
// 2️⃣ Uniswap (UNI)
// Official ERC-20 Contract: 0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984
// Description: Governance token of the Uniswap Protocol.
