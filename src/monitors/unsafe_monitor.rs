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
use std::str::FromStr;
use std::sync::Arc;

pub struct StablecoinMonitor {
    provider: Option<Arc<Provider<Http>>>,
    metrics: Option<Vec<StablecoinMetrics>>,
}

impl Default for StablecoinMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl StablecoinMonitor {
    pub fn new() -> Self {
        Self {
            provider: None,
            metrics: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut output = Vec::new();
        output.push("[LOG] Attempting to connect...".to_string());

        if self.provider.is_some() {
            output.push("[WARNING] Provider already exists!".to_string());
        }

        dotenv().ok();
        let api_key = env::var("ALCHEMY_API_KEY")?;
        let provider = Provider::<Http>::try_from(format!(
            "https://eth-mainnet.g.alchemy.com/v2/{}",
            api_key
        ))?;

        self.provider = Some(Arc::new(provider));
        output.push("[LOG] Connection successful".to_string());
        Ok(())
    }

    pub async fn fetch_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut output = Vec::new();
        output.push("[LOG] Attempting to fetch data...".to_string());

        let provider = match self.provider.as_ref() {
            Some(p) => p,
            None => {
                let err = "[ERROR] No provider found - connect() was not called first";
                output.push(err.to_string());
                return Err(err.into());
            }
        };

        let abi: Abi = serde_json::from_str(
            r#"[
            {"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"type":"function"},
            {"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"type":"function"}
        ]"#,
        )?;

        let mut metrics = Vec::new();
        for (address, name) in [(USDT_ADDRESS, "USDT"), (USDC_ADDRESS, "USDC")] {
            output.push(format!("[LOG] Fetching data for {}", name));
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
            output.push(format!("[LOG] Successfully fetched {} data", name));
        }

        self.metrics = Some(metrics);
        Ok(())
    }

    pub fn display_results(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut output = Vec::new();
        output.push("[LOG] Attempting to display results...".to_string());

        let metrics = match self.metrics.as_ref() {
            Some(m) => m,
            None => {
                let err = "[ERROR] No metrics found - fetch_data() was not called first";
                output.push(err.to_string());
                return Err(err.into());
            }
        };

        for metric in metrics {
            let supply = metric.total_supply.as_u128() as f64 / 10f64.powi(metric.decimals as i32);
            output.push(format!("[DATA] {} Supply: ${:.2}", metric.name, supply));
        }

        Ok(output)
    }
}
