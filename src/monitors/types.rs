use ethers::types::U256;

#[derive(Debug)]
pub struct StablecoinMetrics {
    pub name: String,
    pub total_supply: U256,
    pub decimals: u8,
}
