
// HTTP 返回代码
// HTTP 4XX 错误码用于指示错误的请求内容、行为、格式。问题在于请求者。
// HTTP 403 错误码表示违反WAF限制(Web应用程序防火墙)。
// HTTP 429 错误码表示警告访问频次超限，即将被封IP。
// HTTP 418 表示收到429后继续访问，于是被封了。
// HTTP 5XX 错误码用于指示Binance服务侧的问题。

use std::io::Stderr;

use anyhow::{Error, Ok};
use rust_decimal::Decimal;

use self::type_::LatestPriceResult;

mod type_;

pub struct Spot {
    api_key: String,
    api_secret: String,
}

const BASE_URL: &str = "https://api.binance.com";

impl Spot {
    pub fn new(api_key: String, api_secret: String) -> Spot {
        Spot {
            api_key,
            api_secret,
        }
    }

    pub async fn latest_price(&self, symbols: Vec<String>) -> Result<Vec<Decimal>, Error> {
        // https://api.binance.com/api/v3/ticker/price?symbols=["BTCUSDT","BNBUSDT"]
        let client = reqwest::Client::new();
        let res = client
        .get(format!("{}{}", BASE_URL, "/api/v3/ticker/price"))
        .query(&[("symbols", format!(r#"["{}"]"#, symbols.join(r#"",""#)))])
        .send().await?;
        // println!("{:?}", res.text().await);
        // Err(Error::msg("vsdf"))
        let result = res.json::<Vec<LatestPriceResult>>().await?;
        Ok(result.iter().map(|x| Decimal::from_str_exact(x.price.as_str()).unwrap()).collect())
    }
}
