use super::common::{create_provider, fetch_stablecoin_metrics, format_supply};
use super::states::{Connected, DataFetched, Unconnected};
use super::types::StablecoinMetrics;
use ethers::providers::{Http, Provider};
use std::marker::PhantomData;
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

        let provider = create_provider()?;

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
        let metrics = fetch_stablecoin_metrics(provider.clone()).await?;

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
        output.extend(
            format_supply(metrics)
                .into_iter()
                .map(|s| format!("[SAFE] {}", s)),
        );

        for msg in &output {
            println!("{}", msg);
        }
        output
    }
}
