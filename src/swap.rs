use ethers::prelude::*;
use ethers::types::{U256};
use ethers::utils::{parse_units, format_units};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use chrono::Utc;
use anyhow::Result;
use std::sync::Arc;
use ethers::types::transaction::eip2718::TypedTransaction;
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::AppConfig;

/// 处理滑点（交易保护用）
fn apply_slippage_wei(amount: U256, slippage_bp: u32) -> U256 {
    // slippage_bp = 基点，50 = 0.5%
    amount * U256::from(10000 - slippage_bp) / U256::from(10000)
}


pub struct SwapModule {
    pub provider: Arc<Provider<Http>>,
    pub config: AppConfig,
}

impl SwapModule {
    pub fn new(provider: Provider<Http>, config: AppConfig) -> Self {
        Self {
            provider: Arc::new(provider),
            config,
        }
    }

    /// 模拟 V2 swap
    /// from_token / to_token: 传名称即可，比如 "ETH", "USDC", "BTC"
    /// 返回 (estimated_output, gas_estimate)
    pub async fn swap_tokens(
        &self,
        from_token: &str,
        to_token: &str,
        amount_in: Decimal,
        slippage: f64,
    ) -> Result<(Decimal, Decimal)> {


        info!("Something happened ddddddfjjjjjjjjjdkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");

        abigen!(
            UniswapV2Router,
            r#"[
                function getAmountsOut(uint256 amountIn, address[] path) external view returns (uint256[] amounts)
                function swapExactTokensForTokens(uint256 amountIn,uint256 amountOutMin,address[] path,address to,uint256 deadline) external returns (uint256[] amounts)
                function swapExactETHForTokens(uint256 amountOutMin,address[] path,address to,uint256 deadline) external payable returns (uint256[] amounts)
                function swapExactTokensForETH(uint256 amountIn,uint256 amountOutMin,address[] path,address to,uint256 deadline) external returns (uint256[] amounts)
            ]"#
        );

        abigen!(
            ERC20,
            r#"[
                function decimals() view returns (uint8)
            ]"#
        );

        let router = UniswapV2Router::new(self.config.uniswap_v2_router, self.provider.clone());
        let weth_addr = self.config.token_address("WETH").expect("WETH not set");

        // -------------------------------
        // 统一处理 ETH -> WETH
        // -------------------------------
        let from_addr = if from_token == "ETH" { weth_addr } else { self.config.token_address(from_token).expect("Invalid from_token") };
        let to_addr = if to_token == "ETH" { weth_addr } else { self.config.token_address(to_token).expect("Invalid to_token") };

        let is_eth_to_token = from_token == "ETH";
        let is_token_to_eth = to_token == "ETH";

        // -------------------------------
        // 获取 decimals
        // -------------------------------
        let from_decimals: u32 =
            ERC20::new(from_addr, self.provider.clone())
                .decimals()
                .call()
                .await? as u32;


        // -------------------------------
        // 构造 path + amount_in
        // -------------------------------
        let amount_in_wei: U256 = parse_units(amount_in.to_string(), from_decimals)?.into();
        let path = vec![from_addr, to_addr];

        // -------------------------------
        // 模拟 getAmountsOut
        // -------------------------------
        info!("amount_in_wei: {:#?}",amount_in_wei);
        let amounts_out = match router.get_amounts_out(amount_in_wei, path.clone()).call().await {
            Ok(res) => res,
            Err(e) => {
                return Ok((Decimal::ZERO, Decimal::ZERO));
            }
        };
        // 用整数计算滑点
        let slippage_bp = (slippage * 100.0) as u32; // 0.5% -> 50 基点
        let estimated_wei = *amounts_out.last().unwrap();

        // -------------------------------
        // 计算 min_dec 用于交易保护
        // -------------------------------
        let min_u256 = apply_slippage_wei(estimated_wei, slippage_bp);

        // -------------------------------
        // 构造交易（模拟，不发送）
        // -------------------------------
        let deadline = U256::from((Utc::now().timestamp() + 600) as u64);
        let tx: TypedTransaction = if is_eth_to_token {
            router
                .swap_exact_eth_for_tokens(min_u256, path.clone(), self.config.wallet_address, deadline)
                .value(amount_in_wei)
                .tx
        } else if is_token_to_eth {
            router
                .swap_exact_tokens_for_eth(amount_in_wei, min_u256, path.clone(), self.config.wallet_address, deadline)
                .tx
        } else {
            router
                .swap_exact_tokens_for_tokens(amount_in_wei, min_u256, path.clone(), self.config.wallet_address, deadline)
                .tx
        };



        // -------------------------------
        // 模拟调用 eth_call 获取输出（可选）
        // -------------------------------
        let _return_bytes = self.provider.call(&tx, None).await?;

        // -------------------------------
        // 估算 gas
        // -------------------------------
        let gas = self.provider.estimate_gas(&tx, None).await?;
        let gas_dec = Decimal::from_u128(gas.as_u128()).unwrap();

        let to_decimals: u32 =
            ERC20::new(to_addr, self.provider.clone())
                .decimals()
                .call()
                .await? as u32;
        let est_dec = Decimal::from_str(&format_units(estimated_wei, to_decimals)?)?;
        Ok((est_dec, gas_dec))
    }
}
