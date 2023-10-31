use std::vec;

use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;
use crate::misc;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BitmartTickerResponse {
    pub message: String,
    pub code: u64,
    pub trace: String,
    pub data: CoinPrice,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CoinPrice {
    pub symbol: String,
    pub last_price: String,
    pub quote_volume_24h: String,
    pub base_volume_24h: String,
    pub high_24h: String,
    pub low_24h: String,
    pub open_24h: String,
    pub close_24h: String,
    pub best_ask: String,
    pub best_ask_size: String,
    pub best_bid: String,
    pub best_bid_size: String,
    pub fluctuation: String,
    pub timestamp: u64,
    pub url: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BitmartTickerResponseV2 {
    pub data: BitmartTickers,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BitmartTickers {
    pub tickers: Vec<BitmartTicker>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BitmartTicker {
    pub symbol: String,
    pub last_price: String,
    pub timestamp: u64,
    pub base_volume_24h: String,
}

pub async fn get_latest_price(base: &str, currency: &str) -> Result<f64> {
    let request_url: String = utils::get_latest_price_url(base, currency);
    let response: BitmartTickerResponse =
        request::request(&request_url).await?;
    let price: f64 = response.data.last_price.parse()?;
    Ok(price)
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    invalid_time: u64,
) -> Result<(Vec<f64>, Vec<f64>)> {
    let request_url: String = utils::get_latest_price_url_v2();
    let base_indexs = utils::get_pairs(bases, currency);
    let response: BitmartTickerResponseV2 =
        request::request(&request_url).await?;

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];
    let mut vec_volumes = vec![0.0f64; size];

    for t in response.data.tickers.iter() {
        if base_indexs.contains_key(&(*t).symbol) {
            let ts: u64 = (*t).timestamp;
            let current_ts = misc::get_timestamp();
            if current_ts > ts + invalid_time {
                continue;
            }
            let index = *base_indexs.get(&(*t).symbol).unwrap();
            let price: f64 = (*t).last_price.parse()?;
            vec_prices[index] = price;

            let volume: f64 = (*t).base_volume_24h.parse()?;
            vec_volumes[index] = volume;
        }
    }
    Ok((vec_prices, vec_volumes))
}
