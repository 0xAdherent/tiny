use crate::misc;
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BitgetTickerResponseV2 {
    pub data: Vec<BitgetTicker>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BitgetTicker {
    pub symbol: String,
    pub close: String,
    pub ts: String,
    pub baseVol: String,
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    invalid_time: u64,
) -> Result<(Vec<f64>, Vec<f64>)> {
    let request_url: String = utils::get_latest_price_url_v2();
    let base_indexs = utils::get_pairs(bases, currency);
    let response: BitgetTickerResponseV2 =
        request::request(&request_url).await?;

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];
    let mut vec_volumes = vec![0.0f64; size];

    for t in response.data.iter() {
        if base_indexs.contains_key(&(*t).symbol) {
            let ts: u64 = (*t).ts.parse()?;
            let current_ts = misc::get_timestamp();
            if current_ts > ts + invalid_time {
                continue;
            }
            let index = *base_indexs.get(&(*t).symbol).unwrap();
            let price: f64 = (*t).close.parse()?;
            vec_prices[index] = price;
            let volume: f64 = (*t).baseVol.parse()?;
            vec_volumes[index] = volume;
        }
    }
    Ok((vec_prices, vec_volumes))
}
