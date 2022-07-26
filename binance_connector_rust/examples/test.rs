
extern crate binance_connector_rust;

use anyhow::{Result, Ok};
use binance_connector_rust::spot::Spot;

#[tokio::main]
async fn main() -> Result<()> {

    let spot = Spot::new("".to_string(), "".to_string());
    let result = spot.latest_price(vec!("BTCUSDT".to_string())).await.unwrap();
    println!("price: {:?}", result);
    
    Ok(())
}

