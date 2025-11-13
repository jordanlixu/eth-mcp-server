// src/balance.rs
use ethers::prelude::*;
use ethers::abi::Abi;
use rust_decimal::Decimal;
use anyhow::Result;
use std::sync::Arc;

pub struct BalanceModule {
    pub provider: Arc<Provider<Http>>,
}

impl BalanceModule {
    pub fn new(provider: Provider<Http>) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }

    /// 获取钱包余额
    /// - address: 钱包地址
    /// - token: None -> ETH, Some(token_addr) -> ERC20
    pub async fn get_balance(&self, address: Address, token: Option<Address>) -> Result<Decimal> {
        let balance_decimal = match token {
            None => {
                // ETH 余额
                let balance_wei = self.provider.get_balance(address, None).await?;
                let balance_str = ethers::utils::format_units(balance_wei, 18).unwrap(); // ETH 固定 18 decimals
                balance_str.parse::<Decimal>()?
            }
            Some(token_addr) => {
                // ERC20 余额
                let erc20_abi: Abi =
                    serde_json::from_slice(include_bytes!("../abis/erc20_abi.json"))?;
                let erc20 = Contract::new(token_addr, erc20_abi, self.provider.clone());

                // ERC20 balance
                let balance_wei: U256 = erc20
                    .method::<_, U256>("balanceOf", address)?
                    .call()
                    .await?;

                // ERC20 decimals
                let decimals: u8 = erc20
                    .method::<_, u8>("decimals", ())?
                    .call()
                    .await?;
                let decimals_u32 = decimals as u32;

                let balance_str = ethers::utils::format_units(balance_wei, decimals_u32)
                    .unwrap(); // 安全，因为 Infallible 永远不会失败

                balance_str.parse::<Decimal>()?
            }
        };

        Ok(balance_decimal)
    }
}
