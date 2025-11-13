// src/price.rs
use ethers::prelude::*;
use rust_decimal::Decimal;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use ethers::abi::Abi;

pub struct PriceModule {
    pub provider: Arc<Provider<Http>>,
    /// token address 或 symbol → Chainlink feed address
    pub feed_map: HashMap<String, Address>,
}

impl PriceModule {
    pub fn new(provider: Provider<Http>) -> Self {
        let mut feed_map = HashMap::new();

        // Sepolia 测试网示例
        feed_map.insert("ETH".to_string(), "0x694AA1769357215DE4FAC081bf1f309aDC325306".parse().unwrap()); // ETH/USD
        feed_map.insert("BTC".to_string(), "0x1b44F3514812d835EB1BDB0acB33d3fA3351Ee43".parse().unwrap()); // BTC/USD
        feed_map.insert(
            "DAI".to_string(),
            "0xAed0c38402a5d19df6E4c03F4E2DceD6e29c1ee9".parse().unwrap(),
        ); // DAI/USD
        feed_map.insert(
            "USDC".to_string(),
            "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".parse().unwrap(),
        ); // USDC/USD
        feed_map.insert(
            "UNI".to_string(),
            "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".parse().unwrap(),
        ); // UNI/ETH 示例，实际要查 Chainlink 文档

        Self {
            provider: Arc::new(provider),
            feed_map,
        }
    }

    /// 获取价格
    /// - token: None -> ETH/USD
    /// - token: Some(symbol or address) -> 对应 feed 原生单位
    pub async fn get_price(&self, token: Option<&str>) -> Result<Decimal> {
        // 默认 ETH
        let key = token.unwrap_or("ETH");

        let feed_address = self
            .feed_map
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("No feed available for token {}", key))?;

        let abi: Abi = serde_json::from_slice(include_bytes!("../abis/aggregatorv3_abi.json"))?;
        let contract = Contract::new(*feed_address, abi, self.provider.clone());

        // latestAnswer() -> i128
        let answer: i128 = contract
            .method::<(), i128>("latestAnswer", ())?
            .call()
            .await?;

        // Chainlink feed 一般 8 decimals
        let price = Decimal::from_i128_with_scale(answer, 8);
        Ok(price)
    }
}
