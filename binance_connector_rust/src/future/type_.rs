use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct LatestPriceResult {
    pub symbol: String,
    pub price: String,
    pub time: u64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOrderParams {
    pub symbol: String,
    pub side: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<Decimal>,
    #[serde(rename = "closePosition")]
    pub close_position: Option<String>,
    #[serde(rename = "recvWindow")]
    pub recv_window: Option<u64>,
    pub timestamp: u64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOrderResult {
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "orderId")]
    pub order_id: u64,
    pub type_: String,
}
