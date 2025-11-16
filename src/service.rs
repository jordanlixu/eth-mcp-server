use rmcp::{
    ServerHandler,
    handler::server::{
        router::tool::ToolRouter,
        wrapper::{Json, Parameters},
    },
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use ethers::types::Address;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::str::FromStr;

use crate::balance::BalanceModule;
use crate::price::PriceModule;
use crate::swap::SwapModule;

// 输入输出类型
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct BalanceArgs {
    pub address: String,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct BalanceResult {
    pub balance: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct PriceArgs {
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct PriceResult {
    pub price: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct SwapArgs {
    pub from_token: String,
    pub to_token: String,
    pub amount_in: String,
    pub slippage: f64,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct SwapResult {
    pub estimated_output: String,
    pub gas: String,
}

// MCP 服务
pub struct TokenService {
    pub balance: Arc<BalanceModule>,
    pub price: Arc<PriceModule>,
    pub swap: Arc<SwapModule>,
    pub tool_router: ToolRouter<TokenService>,
}

#[tool_router]
impl TokenService {
    #[tool]
    async fn get_balance(
        &self,
        Parameters(args): Parameters<BalanceArgs>,
    ) -> Json<BalanceResult> {
        let address: Address = args.address.parse().unwrap();
        let token: Option<Address> = args.token.map(|s| s.parse().ok()).flatten();
        let bal: Decimal = self.balance.get_balance(address, token).await.unwrap();
        Json(BalanceResult { balance: bal.to_string() })
    }

    #[tool]
    async fn get_price(
        &self,
        Parameters(args): Parameters<PriceArgs>,
    ) -> Json<PriceResult> {
        let price: Decimal = self.price.get_price(args.token.as_deref()).await.unwrap();
        Json(PriceResult { price: price.to_string() })
    }

    #[tool]
    async fn swap_tokens(
        &self,
        Parameters(args): Parameters<SwapArgs>,
    ) -> Json<SwapResult>  {

        let amount_dec = Decimal::from_str(&args.amount_in).unwrap();

        // let amount_in = Decimal::from_f64(0.001).unwrap();

        // 调用 swap_tokens
        let (estimated_output, gas) = self
            .swap
            .swap_tokens(&args.from_token, &args.to_token,amount_dec, args.slippage)
            .await
            .unwrap_or((Decimal::ZERO, Decimal::ZERO));

        Json(SwapResult {
            estimated_output: estimated_output.to_string(),
            gas: gas.to_string(),
        })
    }

    pub fn new(balance: Arc<BalanceModule>, price: Arc<PriceModule>, swap: Arc<SwapModule>) -> Self {
         Self {
            balance,
            price,
            swap,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for TokenService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("TokenService MCP Server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
