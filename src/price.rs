// src/price.rs
use anyhow::{anyhow, Result};
use ethers::abi::Abi;
use ethers::prelude::*;
use rust_decimal::Decimal;
use std::sync::Arc;

use crate::config::AppConfig;

const AGGREGATOR_ABI_JSON: &[u8] = include_bytes!("../abis/aggregatorv3_abi.json");

pub struct PriceModule {
    pub provider: Arc<Provider<Http>>,
    pub config: AppConfig,
}

impl PriceModule {
    pub fn new(provider: Provider<Http>, config: AppConfig) -> Self {
        Self {
            provider: Arc::new(provider),
            config,
        }
    }

    // ----------------------------------------
    // Public API
    // ----------------------------------------

    /// 获取价格
    /// token:
    ///   - None → 默认 ETH/USD
    ///   - Some("WETH") → 配置里查地址
    ///   - Some("0x...") → 直接当 Chainlink feed address
    pub async fn get_price(&self, token: Option<&str>) -> Result<Decimal> {
        // 默认 ETH
        let key = token.unwrap_or("ETH");

        let feed_address = self.resolve_feed_address(key)?;

        self.fetch_price(feed_address).await
    }

    pub async fn eth_price(&self) -> Result<Decimal> {
        self.get_price(None).await
    }

    pub async fn price(&self, symbol: &str) -> Result<Decimal> {
        self.get_price(Some(symbol)).await
    }

    // ----------------------------------------
    // Internal
    // ----------------------------------------

    /// 把 symbol 或 0x 地址映射成 chainlink feed address
    fn resolve_feed_address(&self, input: &str) -> Result<Address> {
        // 1. 如果用户传入 0x... 就直接解析
        if input.starts_with("0x") {
            return Ok(input.parse()?);
        }

        // 2. 去你的 config 里查
        if let Some(addr) = self.config.token_address(input) {
            return Ok(addr);
        }

        Err(anyhow!("Unknown token or feed address: {}", input))
    }

    /// 调用链上 price feed 获取价格
    /// 根据 feed 地址获取价格
     async fn fetch_price(&self, feed_addr: Address) -> Result<Decimal> {
        // 从 JSON 加载 ABI
        let abi: Abi = serde_json::from_slice(include_bytes!("../abis/aggregatorv3_abi.json"))?;
        let contract = Contract::new(feed_addr, abi, self.provider.clone());

        // 调用 latestRoundData() 获取最新价格
        let (_round_id, answer, _started_at, _updated_at, _answered_in_round): (u128, i128, u64, u64, u128) =
            contract.method("latestRoundData", ())?.call().await?;

        // 动态获取 decimals
        let decimals: u8 = contract.method("decimals", ())?.call().await?;

        // 转成 Decimal 并按 decimals 缩放
        let price = Decimal::from_i128_with_scale(answer, decimals.into());

        Ok(price)
    }
}
