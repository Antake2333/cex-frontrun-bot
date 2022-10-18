mod exchange;
use chrono::Local;
use crypto_market_type::MarketType;
use crypto_markets::fetch_markets;
use std::{thread, time};

fn main() {
    let target_exchange = exchange::OKX;
    let spot = "APT";
    println!("开始获取{:?}交易所的{}交易对信息", target_exchange, spot);
    loop {
        if monitor(target_exchange, spot, MarketType::Spot) {
            break;
        }
        println!(
            "{:?}  {:?}暂未上架{}交易对",
            Local::now(),
            target_exchange,
            spot
        );
        thread::sleep(time::Duration::from_secs(1));
    }
    println!("{:?}已上架{}交易对,停止监控", target_exchange, spot);
}

fn monitor(exchange: &str, spot: &str, market_type: MarketType) -> bool {
    let markets = fetch_markets(exchange, market_type).unwrap();
    for market in markets {
        if market.base == spot.to_string().to_uppercase() {
            return true;
        }
    }
    false
}
