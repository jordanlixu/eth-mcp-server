use std::collections::HashMap;
use std::env;
use ethers::types::Address;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub infura_url: String,
    pub wallet_address: Address,
    pub token_addresses: HashMap<String, Address>,
    pub uniswap_v2_router: Address,
}

impl AppConfig {
    pub fn load() -> Self {
        fn read_address(env_key: &str) -> Address {
            env::var(env_key)
                .unwrap_or_else(|_| panic!("{} must be set", env_key))
                .parse()
                .unwrap_or_else(|_| panic!("Invalid address in {}", env_key))
        }

        let infura_url = env::var("INFURA_URL").expect("INFURA_URL must be set");
        let wallet_address = read_address("WALLET_ADDRESS");

        // Tokens you provided
        const TOKENS: &[&str] = &["ETH", "BTC", "WETH", "USDC", "UNI", "BETH"];

        let token_addresses = TOKENS
            .iter()
            .map(|key| (key.to_string(), read_address(key)))
            .collect::<HashMap<_, _>>();

        // Add Uniswap Router
        let uniswap_v2_router = read_address("UNISWAP_V2_ROUTER");

        Self {
            infura_url,
            wallet_address,
            token_addresses,
            uniswap_v2_router,
        }
    }

    pub fn token_address(&self, token_name: &str) -> Option<Address> {
        self.token_addresses.get(token_name).cloned()
    }
}
