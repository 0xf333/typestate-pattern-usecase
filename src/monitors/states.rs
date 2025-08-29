use super::types::StablecoinMetrics;
use ethers::providers::{Http, Provider};

pub struct Unconnected;

pub struct Connected {
    pub(super) provider: Provider<Http>,
}

pub struct DataFetched {
    pub(super) metrics: Vec<StablecoinMetrics>,
}
