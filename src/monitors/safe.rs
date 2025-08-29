use super::common::{create_provider, fetch_stablecoin_metrics, format_supply};
use super::states::{Connected, DataFetched, Unconnected};
use eyre::Result;
use tracing::info;

pub struct StablecoinMonitor<State = Unconnected> {
    state: State,
}

impl Default for StablecoinMonitor<Unconnected> {
    fn default() -> Self {
        Self::new()
    }
}

impl StablecoinMonitor<Unconnected> {
    pub fn new() -> Self {
        Self { state: Unconnected }
    }

    pub async fn connect(self) -> Result<StablecoinMonitor<Connected>> {
        info!("[SAFE] Attempting to connect...");

        let provider = create_provider()?;

        info!("[SAFE] ✅ Connected! Transitioning to Connected state");
        info!("[SAFE] Only fetch_data() is now available");

        Ok(StablecoinMonitor {
            state: Connected { provider },
        })
    }
}

impl StablecoinMonitor<Connected> {
    pub async fn fetch_data(self) -> Result<StablecoinMonitor<DataFetched>> {
        info!("[SAFE] Fetching data...");

        let provider = self.state.provider;
        let metrics = fetch_stablecoin_metrics(&provider).await?;

        info!("[SAFE] ✅ All data fetched! Transitioning to DataFetched state");
        info!("[SAFE] Only display_results() is now available");

        Ok(StablecoinMonitor {
            state: DataFetched { metrics },
        })
    }
}

impl StablecoinMonitor<DataFetched> {
    pub fn display_results(&self) -> Vec<String> {
        let mut output = Vec::new();
        info!("[SAFE] Displaying final results:");

        let data = &self.state;
        output.extend(
            format_supply(&data.metrics)
                .into_iter()
                .map(|s| format!("[SAFE] {}", s)),
        );

        for msg in &output {
            info!("{}", msg);
        }
        output
    }
}
