use std::env;
// swap.rs
use ethers::prelude::*;
use ethers::types::{Address, U256};
use std::sync::Arc;
use rust_decimal::Decimal;
use anyhow::Result;
use ethers::utils::format_units;
use chrono::Utc;
use rust_decimal::prelude::FromPrimitive;


fn decimal_to_u256(amount: Decimal, decimals: u32) -> U256 {
    let scale = Decimal::from_u64(10u64.pow(decimals)).expect("Decimal::from_u64 failed");
    let scaled = (amount * scale).trunc(); // 向下取整
    U256::from_dec_str(&scaled.to_string()).expect("Decimal -> U256 failed")
}



pub struct SwapModule {
    pub provider: Arc<Provider<Http>>,
}

impl SwapModule {
    pub fn new(provider: Provider<Http>) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }

    /// 模拟代币兑换（ETH -> ERC20 或 ERC20 -> ERC20）
    pub async fn swap_tokens(
        &self,
        router_addr: Address,
        from_token: Address,
        to_token: Address,
        amount_in: Decimal,
        slippage: f64,
    ) -> Result<(Decimal, Decimal, Decimal)> {
        abigen!(
            UniswapV2Router,
            r#"[
                function getAmountsOut(uint256 amountIn, address[] path) external view returns (uint256[] amounts)
                function swapExactTokensForTokens(uint256 amountIn,uint256 amountOutMin,address[] path,address to,uint256 deadline) external returns (uint256[] amounts)
                function swapExactETHForTokens(uint256 amountOutMin, address[] path, address to, uint256 deadline) external payable returns (uint256[] amounts)
            ]"#
        );

        abigen!(
            ERC20,
            r#"[
                function decimals() view returns (uint8)
            ]"#
        );

        let router = UniswapV2Router::new(router_addr, self.provider.clone());
        let weth_addr: Address = env::var("WETH")?.parse()?;// Sepolia WETH

        // 获取 to_token 的小数位
        let decimals: u32 = if to_token == Address::zero() {
            18 // ETH 默认 18
        } else {
            let token = ERC20::new(to_token, self.provider.clone());
            token.decimals().call().await? as u32
        };

        // 构造路径与输入量
        let (is_eth, path, amount_in_wei) = if from_token == Address::zero() {
            // ETH -> ERC20
            let amount_in_wei = decimal_to_u256(amount_in, 18);
            (true, vec![weth_addr, to_token], amount_in_wei)
        } else {
            // ERC20 -> ERC20
            let amount_in_wei = decimal_to_u256(amount_in, decimals);
            (false, vec![from_token, to_token], amount_in_wei)
        };



        // 调用 getAmountsOut 模拟兑换
        let amounts_out = match router.get_amounts_out(amount_in_wei, path.clone()).call().await {
            Ok(res) => res,
            Err(e) => {
                println!("getAmountsOut failed: {:?}", e);
                return Ok((Decimal::ZERO, Decimal::ZERO, Decimal::ZERO));
            }
        };
        println!("Amounts out (raw U256): {:?}", amounts_out);
        let estimated_output = *amounts_out.last().unwrap();

        // 转为 Decimal 并考虑滑点
        let estimated_output_decimal = Decimal::from_f64(
            format_units(estimated_output, decimals)?.parse::<f64>()?
        ).unwrap_or(Decimal::ZERO);

        let slippage_multiplier = Decimal::from_f64(1.0 - slippage / 100.0).unwrap();
        let min_output_decimal = estimated_output_decimal * slippage_multiplier;
        let min_output_u256 = decimal_to_u256(min_output_decimal, decimals);

        // 模拟交易
        let dummy_to: Address = env::var("WALLET_ADDRESS")?.parse()?;

        let deadline_secs = Utc::now().timestamp() + 600; // 现在 + 10 分钟
        let deadline = U256::from_dec_str(&deadline_secs.to_string())?;

        let tx = if is_eth {
            // ETH -> ERC20
            println!("ETH -> ERC20");
            router
                .swap_exact_eth_for_tokens(min_output_u256, path.clone(), dummy_to, deadline)
                .value(amount_in_wei)
                .tx
        } else {
            // ERC20 -> ERC20
            println!("ERC20 -> ERC20");
            router
                .swap_exact_tokens_for_tokens(amount_in_wei, min_output_u256, path.clone(), dummy_to, deadline)
                .tx
        };

        // 打印 path
        println!("Swap path:");
        for (i, addr) in path.iter().enumerate() {
            println!("  [{}] {}", i, addr);
        }

        // 模拟调用
        let _sim_result = self.provider.call(&tx, None).await;

        // 模拟 gas 估算
        let gas_estimate = self.provider.estimate_gas(&tx, None).await?;
        let gas_decimal = Decimal::from_u128(gas_estimate.as_u128()).unwrap_or(Decimal::ZERO);

        Ok((estimated_output_decimal, min_output_decimal, gas_decimal))
    }
}
