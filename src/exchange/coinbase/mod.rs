use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CoinbaseTickerResponse {
    pub data: CoinPrice,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    _invalid_time: u64,
) -> Result<Vec<f64>> {
    let request_url: String = utils::get_latest_price_url("USDT", "USD");
    let response: CoinbaseTickerResponse =
        request::request(&request_url).await?;
    let base_indexs = utils::get_pairs(bases, currency);

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];

    let index = *base_indexs.get(&response.data.base).unwrap();
    let price: f64 = response.data.amount.parse()?;
    vec_prices[index] = price;

    Ok(vec_prices)
}
