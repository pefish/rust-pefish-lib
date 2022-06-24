use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct LatestPriceResult {
    pub symbol: String,
    pub price: String,
}
