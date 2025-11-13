use ethers::prelude::*;
use std::env;
use dotenv::dotenv;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let rpc_url = env::var("INFURA_URL")?;

    let provider = Provider::<Http>::connect(rpc_url.as_str()).await
        .interval(std::time::Duration::from_millis(200u64));

    let address: Address = env::var("WALLET_ADDRESS")?.parse()?;


    let balance_wei = provider.get_balance(address, None).await?;

    let balance_str = ethers::utils::format_units(balance_wei, "ether")?;

    let balance_decimal: Decimal = balance_str.parse()?;

    println!("ETH Balance: {} ETH", balance_decimal);

    Ok(())
}
