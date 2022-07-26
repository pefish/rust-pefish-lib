use rust_decimal::{Decimal, prelude::FromPrimitive};

#[cfg(test)]

use super::future::Future;
use super::future::PlaceOrderParams;

#[tokio::main]
#[test]
async fn latest_price_test() {
    let future = Future::new("".to_string(), "".to_string());
    let result = future.latest_price("BTCUSDT".to_string()).await.unwrap();
    // println!("price: {:?}", result);
    assert_eq!(result > Decimal::ZERO, true);
}

#[tokio::main]
#[test]
async fn sign_test() {
    let future = Future::new("".to_string(), "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9".to_string());
    let result = future.sign("symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943");
    // println!("result: {:?}", result);
    assert_eq!(result, "3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9");
}

#[tokio::main]
#[test]
async fn place_order_test() {
    let future = Future::new("".to_string(), "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9".to_string());
    let result = future.place_order(PlaceOrderParams {
        symbol: "BTCUSDT".to_string(),
        side: "BUY".to_string(),
        type_: "LIMIT".to_string(),
        quantity: Some(Decimal::from_i32(1).unwrap()),
        price: Some(Decimal::from_i32(9000).unwrap()),
        recv_window: Some(5000),
        stop_price: None,
        close_position: None,
        timestamp: 1591702613943,
    }).await.unwrap();
    // println!("result: {:?}", result);
    // assert_eq!(result, "3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9");
}
