use crate::misc;
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct HuobiTickerResponse {
    pub ch: String,
    pub status: String,
    pub ts: u64,
    pub tick: CoinPrice,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CoinPrice {
    pub id: u64,
    pub version: u64,
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub amount: f64,
    pub vol: f64,
    pub count: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct HuobiTickerResponseV2 {
    pub data: Vec<HuobiTicker>,
    pub ts: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct HuobiTicker {
    pub symbol: String,
    pub close: f64,
    pub amount: f64,
}

pub async fn get_latest_price(base: &str, currency: &str) -> Result<f64> {
    let request_url: String = utils::get_latest_price_url(base, currency);
    let response: HuobiTickerResponse = request::request(&request_url).await?;
    let price: f64 = response.tick.close;
    Ok(price)
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    invalid_time: u64,
) -> Result<(Vec<f64>, Vec<f64>)> {
    let request_url: String = utils::get_latest_price_url_v2();
    let base_indexs = utils::get_pairs(bases, currency);
    let response: HuobiTickerResponseV2 =
        request::request(&request_url).await?;

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];
    let mut vec_volumes = vec![0.0f64; size];

    let ts: u64 = response.ts;
    let current_ts = misc::get_timestamp();
    if current_ts > ts + invalid_time {
        return Ok((vec_prices, vec_volumes));
    }

    for t in response.data.iter() {
        if base_indexs.contains_key(&(*t).symbol) {
            let index = *base_indexs.get(&(*t).symbol).unwrap();
            let price: f64 = (*t).close;
            vec_prices[index] = price;
            vec_volumes[index] = (*t).amount;
        }
    }
    Ok((vec_prices, vec_volumes))
}
