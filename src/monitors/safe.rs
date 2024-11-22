use super::states::{Connected, DataFetched, Unconnected};
use super::types::StablecoinMetrics;
use crate::constants::{USDC_ADDRESS, USDT_ADDRESS};
use dotenv::dotenv;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    types::{Address, U256},
};
use std::env;
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;

pub struct StablecoinMonitor<State = Unconnected> {
    provider: Option<Arc<Provider<Http>>>,
    metrics: Option<Vec<StablecoinMetrics>>,
    state: PhantomData<State>,
}

impl Default for StablecoinMonitor<Unconnected> {
    fn default() -> Self {
        Self::new()
    }
}

impl StablecoinMonitor<Unconnected> {
    pub fn new() -> Self {
        Self {
            provider: None,
            metrics: None,
            state: PhantomData,
        }
    }

    pub async fn connect(self) -> Result<StablecoinMonitor<Connected>, Box<dyn std::error::Error>> {
        println!("[SAFE] Attempting to connect...");

        dotenv().ok();
        let api_key = env::var("ALCHEMY_API_KEY")?;
        let provider = Provider::<Http>::try_from(format!(
            "https://eth-mainnet.g.alchemy.com/v2/{}",
            api_key
        ))?;

        println!("[SAFE] ✅ Connected! Transitioning to Connected state");
        println!("[SAFE] Only fetch_data() is now available");

        Ok(StablecoinMonitor {
            provider: Some(Arc::new(provider)),
            metrics: None,
            state: PhantomData,
        })
    }
}

impl StablecoinMonitor<Connected> {
    pub async fn fetch_data(
        self,
    ) -> Result<StablecoinMonitor<DataFetched>, Box<dyn std::error::Error>> {
        println!("[SAFE] Fetching data...");

        let provider = self.provider.as_ref().unwrap();

        let abi: Abi = serde_json::from_str(
            r#"[
            {"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"type":"function"},
            {"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"type":"function"}
        ]"#,
        )?;

        let mut metrics = Vec::new();
        for (address, name) in [(USDT_ADDRESS, "USDT"), (USDC_ADDRESS, "USDC")] {
            println!("[SAFE] Fetching {} data...", name);
            let contract =
                Contract::new(Address::from_str(address)?, abi.clone(), provider.clone());

            let total_supply: U256 = contract
                .method::<_, U256>("totalSupply", ())?
                .call()
                .await?;
            let decimals: u8 = contract.method::<_, u8>("decimals", ())?.call().await?;

            metrics.push(StablecoinMetrics {
                name: name.to_string(),
                total_supply,
                decimals,
            });
        }

        println!("[SAFE] ✅ All data fetched! Transitioning to DataFetched state");
        println!("[SAFE] Only display_results() is now available");

        Ok(StablecoinMonitor {
            provider: Some(provider.clone()),
            metrics: Some(metrics),
            state: PhantomData,
        })
    }
}

impl StablecoinMonitor<DataFetched> {
    pub fn display_results(&self) -> Vec<String> {
        let mut output = Vec::new();
        println!("[SAFE] Displaying final results:");

        let metrics = self.metrics.as_ref().unwrap();
        for metric in metrics {
            let supply = metric.total_supply.as_u128() as f64 / 10f64.powi(metric.decimals as i32);
            let msg = format!("[SAFE] {} Supply: ${:.2}", metric.name, supply);
            println!("{}", msg);
            output.push(msg);
        }
        output
    }
}
