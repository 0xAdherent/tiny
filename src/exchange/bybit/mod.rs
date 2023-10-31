use crate::misc;
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BybitTickerResponse {
    pub ret_code: u64,
    pub ret_msg: String,
    pub result: Vec<CoinPrice>,
    pub ext_code: String,
    pub ext_info: String,
    pub time_now: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CoinPrice {
    pub symbol: String,
    pub bid_price: String,
    pub ask_price: String,
    pub last_price: String,
    pub last_tick_direction: String,
    pub prev_price_24h: String,
    pub price_24h_pcnt: String,
    pub high_price_24h: String,
    pub low_price_24h: String,
    pub prev_price_1h: String,
    pub mark_price: String,
    pub index_price: String,
    pub open_interest: f64,
    pub countdown_hour: u64,
    pub turnover_24h: String,
    pub volume_24h: f64,
    pub funding_rate: String,
    pub predicted_funding_rate: String,
    pub next_funding_time: String,
    pub predicted_delivery_price: String,
    pub total_turnover: String,
    pub total_volume: f64,
    pub delivery_fee_rate: String,
    pub delivery_time: String,
    pub price_1h_pcnt: String,
    pub open_value: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BybitTickerResponseV2 {
    pub result: BybitTickerResult,
    pub time: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BybitTickerResult {
    pub list: Vec<BybitTicker>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BybitTicker {
    pub symbol: String,
    pub lastPrice: String,
    pub volume24h: String,
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    invalid_time: u64,
) -> Result<(Vec<f64>, Vec<f64>)> {
    let request_url: String = utils::get_latest_price_url_v2();
    let base_indexs = utils::get_pairs(bases, currency);
    let response: BybitTickerResponseV2 =
        request::request(&request_url).await?;

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];
    let mut vec_volumes = vec![0.0f64; size];

    let ts: u64 = response.time;
    let current_ts = misc::get_timestamp();
    if current_ts > ts + invalid_time {
        return Ok((vec_prices, vec_volumes));
    }

    for t in response.result.list.iter() {
        if base_indexs.contains_key(&(*t).symbol) {
            let index = *base_indexs.get(&(*t).symbol).unwrap();
            let price: f64 = (*t).lastPrice.parse()?;
            vec_prices[index] = price;

            let volume: f64 = (*t).volume24h.parse()?;
            vec_volumes[index] = volume;
        }
    }
    Ok((vec_prices, vec_volumes))
}
