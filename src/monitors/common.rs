use super::types::StablecoinMetrics;
use crate::constants::{USDC_ADDRESS, USDT_ADDRESS};
use dotenv::dotenv;
use ethers::{
    abi::{Abi, AbiParser},
    contract::Contract,
    providers::{Http, Provider},
    types::{Address, U256},
};
use eyre::Result;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

pub fn get_stablecoin_abi() -> Result<Abi> {
    let source = include_str!("../abis/tether.json");
    let items: Vec<String> = serde_json::from_str(source)?;
    let entries: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
    let abi = AbiParser::default().parse(&entries)?;
    Ok(abi)
}

pub async fn fetch_stablecoin_metrics(provider: &Provider<Http>) -> Result<Vec<StablecoinMetrics>> {
    let abi = get_stablecoin_abi()?;
    let mut metrics = Vec::new();

    let client = Arc::new(provider.clone());
    for (address, name) in [(USDT_ADDRESS, "USDT"), (USDC_ADDRESS, "USDC")] {
        let contract = Contract::new(Address::from_str(address)?, abi.clone(), client.clone());

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

    Ok(metrics)
}

pub fn format_supply(metrics: &[StablecoinMetrics]) -> Vec<String> {
    metrics
        .iter()
        .map(|metric| {
            let supply = metric.total_supply.as_u128() as f64 / 10f64.powi(metric.decimals as i32);
            format!("{} Supply: ${:.2}", metric.name, supply)
        })
        .collect()
}

pub fn create_provider() -> Result<Provider<Http>> {
    dotenv().ok();
    let api_key = env::var("ALCHEMY_API_KEY")?;
    let provider =
        Provider::<Http>::try_from(format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key))?;
    Ok(provider)
}
