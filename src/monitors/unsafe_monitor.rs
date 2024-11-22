use super::common::{create_provider, fetch_stablecoin_metrics, format_supply};
use super::types::StablecoinMetrics;
use ethers::providers::{Http, Provider};
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

        let provider = create_provider()?;
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

        self.metrics = Some(fetch_stablecoin_metrics(provider.clone()).await?);
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

        output.extend(
            format_supply(metrics)
                .into_iter()
                .map(|s| format!("[DATA] {}", s)),
        );
        Ok(output)
    }
}
