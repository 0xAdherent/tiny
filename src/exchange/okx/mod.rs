use crate::misc;
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct OKXTickerResponse {
    pub code: String,
    pub msg: String,
    pub data: Vec<CoinPrice>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CoinPrice {
    pub instType: String,
    pub instId: String,
    pub last: String,
    pub lastSz: String,
    pub askPx: String,
    pub askSz: String,
    pub bidPx: String,
    pub bidSz: String,
    pub open24h: String,
    pub high24h: String,
    pub low24h: String,
    pub volCcy24h: String,
    pub vol24h: String,
    pub ts: String,
    pub sodUtc0: String,
    pub sodUtc8: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct OKXTickerResponseV2 {
    pub data: Vec<OkxTicker>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct OkxTicker {
    pub instId: String,
    pub last: String,
    pub vol24h: String,
    pub ts: String,
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    invalid_time: u64,
) -> Result<(Vec<f64>, Vec<f64>)> {
    let request_url: String = utils::get_latest_price_url_v2();
    let base_indexs = utils::get_pairs(bases, currency);
    let response: OKXTickerResponseV2 = request::request(&request_url).await?;

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];
    let mut vec_volumes = vec![0.0f64; size];

    for t in response.data.iter() {
        if base_indexs.contains_key(&(*t).instId) {
            let ts: u64 = (*t).ts.parse()?;
            let current_ts = misc::get_timestamp();
            if current_ts > ts + invalid_time {
                continue;
            }

            let index = *base_indexs.get(&(*t).instId).unwrap();
            let price: f64 = (*t).last.parse()?;
            vec_prices[index] = price;
            let volume: f64 = (*t).vol24h.parse()?;
            vec_volumes[index] = volume;
        }
    }
    Ok((vec_prices, vec_volumes))
}
