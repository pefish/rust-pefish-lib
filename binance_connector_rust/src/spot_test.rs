use rust_decimal::Decimal;

#[cfg(test)]

use super::spot::Spot;

#[tokio::main]
#[test]
async fn latest_price_test() {
    let spot = Spot::new("".to_string(), "".to_string());
    let result = spot.latest_price(vec!("BTCUSDT".to_string())).await.unwrap();
    // println!("hh: {:?}", result);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0] > Decimal::ZERO, true);
}
