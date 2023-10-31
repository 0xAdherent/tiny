use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BinanceTickerResponse {
    pub symbol: String,
    pub priceChange: String,
    pub priceChangePercent: String,
    pub weightedAvgPrice: String,
    pub openPrice: String,
    pub highPrice: String,
    pub lowPrice: String,
    pub lastPrice: String,
    pub volume: String,
    pub quoteVolume: String,
    pub openTime: u64,
    pub closeTime: u64,
    pub firstId: u64,
    pub lastId: u64,
    pub count: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BinanceTickerResponseV2 {
    pub symbol: String,
    pub lastPrice: String,
    pub volume: String,
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    _invalid_time: u64,
) -> Result<(Vec<f64>, Vec<f64>)> {
    let request_url: String = utils::get_latest_price_url_v2(bases, currency);
    let responses: Vec<BinanceTickerResponseV2> =
        request::request(&request_url).await?;
    let base_indexs = utils::get_pairs(bases, currency);

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];
    let mut vec_volumes = vec![0.0f64; size];

    for t in responses.iter() {
        let index = *base_indexs.get(&(*t).symbol).unwrap();
        let price: f64 = (*t).lastPrice.parse()?;
        vec_prices[index] = price;
        let volume: f64 = (*t).volume.parse()?;
        vec_volumes[index] = volume;
    }
    Ok((vec_prices, vec_volumes))
}

pub async fn get_usdt_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    _invalid_time: u64,
) -> Result<(Vec<f64>, usize)> {
    let request_url: String = utils::get_latest_price_url("USDT", "USD");
    let response: BinanceTickerResponse =
        request::request(&request_url).await?;
    let base_indexs = utils::get_pairs(bases, currency);

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];

    let index = *base_indexs.get(&response.symbol).unwrap();
    let price: f64 = response.lastPrice.parse()?;
    vec_prices[index] = price;

    Ok((vec_prices, index))
}
