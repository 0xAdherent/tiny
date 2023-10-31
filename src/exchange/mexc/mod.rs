use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct MEXCTickerResponse {
    pub symbol: String,
    pub priceChange: String,
    pub priceChangePercent: String,
    pub prevClosePrice: String,
    pub lastPrice: String,
    pub bidPrice: String,
    pub bidQty: String,
    pub askPrice: String,
    pub askQty: String,
    pub openPrice: String,
    pub highPrice: String,
    pub lowPrice: String,
    pub volume: String,
    pub quoteVolume: String,
    pub openTime: u64,
    pub closeTime: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct MEXCTickerResponseV2 {
    pub symbol: String,
    pub lastPrice: String,
    pub volume: String,
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    _invalid_time: u64,
) -> Result<(Vec<f64>, Vec<f64>)> {
    let request_url: String = utils::get_latest_price_url_v2();
    let base_indexs = utils::get_pairs(bases, currency);
    let responses: Vec<MEXCTickerResponseV2> =
        request::request(&request_url).await?;

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];
    let mut vec_volumes = vec![0.0f64; size];

    for t in responses.iter() {
        if base_indexs.contains_key(&(*t).symbol) {
            let index = *base_indexs.get(&(*t).symbol).unwrap();
            let price: f64 = (*t).lastPrice.parse()?;
            vec_prices[index] = price;
            let volume: f64 = (*t).volume.parse()?;
            vec_volumes[index] = volume;
        }
    }
    Ok((vec_prices, vec_volumes))
}
