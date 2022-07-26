
use std::io::Stderr;

use anyhow::{Error, Ok};
use rust_decimal::Decimal;

pub use self::type_::{LatestPriceResult, PlaceOrderParams, PlaceOrderResult};

pub mod type_;

pub struct Future {
    api_key: String,
    api_secret: String,
}

const BASE_URL: &str = "https://fapi.binance.com";

impl Future {
    pub fn new(api_key: String, api_secret: String) -> Future {
        Future {
            api_key,
            api_secret,
        }
    }

    pub fn sign (&self, data: &str) -> String {
        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, self.api_secret.as_bytes());

        let tag = ring::hmac::sign(&key, data.as_bytes());
        
        hex::encode(tag.as_ref())
    }

    pub async fn latest_price(&self, symbol: String) -> Result<Decimal, Error> {
        // https://fapi.binance.com/fapi/v1/ticker/price?symbol=BTCUSDT
        let client = reqwest::Client::new();
        let res = client
        .get(format!("{}{}", BASE_URL, "/fapi/v1/ticker/price"))
        .query(&[("symbol", symbol.as_str())])
        .send().await?;
        // println!("{:?}", res.text().await);
        // Err(Error::msg("vsdf"))
        let result = res.json::<LatestPriceResult>().await?;
        Ok(Decimal::from_str_exact(result.price.as_str()).unwrap())
    }

    pub async fn place_order(&self, params: PlaceOrderParams) -> Result<PlaceOrderResult, Error> {
        let v = serde_json::to_value(&params).unwrap();
        for (k, v) in v.as_object().unwrap().iter() {
            println!("{}:{}", k, v.is_null());
        }

        // let client = reqwest::Client::new();
        // let res = client
        // .post(format!("{}{}", BASE_URL, "/fapi/v1/order"))
        // .header("X-MBX-APIKEY", self.api_key.as_str())
        // .body("bsfgbs")
        // .send().await?;
        // // println!("{:?}", res.text().await);
        // // Err(Error::msg("vsdf"))
        // let result = res.json::<LatestPriceResult>().await?;
        Ok(PlaceOrderResult {
            client_order_id: "".to_string(),
            order_id: 0,
            type_: "".to_string(),
        })
    }
}
