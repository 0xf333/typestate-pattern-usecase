use crate::constants::*;
use ethers::{
    providers::{Provider, Http},
    types::{Address, U256},
    contract::Contract,
    abi::Abi,
};
use std::sync::Arc;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct StablecoinMetrics {
    pub name: String,
    pub total_supply: U256,
    pub decimals: u8,
}

pub struct StablecoinMonitor {
    provider: Option<Arc<Provider<Http>>>,
    metrics: Option<Vec<StablecoinMetrics>>,
}

impl StablecoinMonitor {
    pub fn new() -> Self {
        Self {
            provider: None,
            metrics: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let api_key = env::var("ALCHEMY_API_KEY")?;
        let provider = Provider::<Http>::try_from(
            format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key)
        )?;
        
        self.provider = Some(Arc::new(provider));
        Ok(())
    }

    pub async fn fetch_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let provider = self.provider.as_ref()
            .ok_or("Provider not initialized")?;
        
        let mut metrics = Vec::new();
        let abi: Abi = serde_json::from_str(r#"[
            {"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"type":"function"},
            {"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"type":"function"}
        ]"#)?;

        for (address, name) in [
            (USDT_ADDRESS, "USDT"),
            (USDC_ADDRESS, "USDC"),
        ] {
            let contract = Contract::new(
                Address::from_str(address)?,
                abi.clone(),
                provider.clone(),
            );

            let total_supply: U256 = contract.method::<_, U256>("totalSupply", ())?.call().await?;
            let decimals: u8 = contract.method::<_, u8>("decimals", ())?.call().await?;

            metrics.push(StablecoinMetrics {
                name: name.to_string(),
                total_supply,
                decimals,
            });
        }

        self.metrics = Some(metrics);
        Ok(())
    }

    pub fn display_results(&self) -> Result<(), Box<dyn std::error::Error>> {
        let metrics = self.metrics.as_ref()
            .ok_or("Metrics not fetched")?;

        println!("\nStablecoin Analysis Results");
        println!("=========================================");
        
        for metric in metrics {
            let supply = metric.total_supply.as_u128() as f64 / 10f64.powi(metric.decimals as i32);
            println!("\n{}", metric.name);
            println!("Total Supply: ${:.2}", supply);
        }
        Ok(())
    }
} 