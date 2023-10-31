use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct GateTicker {
    pub currency_pair: String,
    pub last: String,
    pub base_volume: String,
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    _invalid_time: u64,
) -> Result<(Vec<f64>, Vec<f64>)> {
    let request_url: String = utils::get_latest_price_url_v2();
    let base_indexs = utils::get_pairs(bases, currency);
    let responses: Vec<GateTicker> = request::request(&request_url).await?;

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];
    let mut vec_volumes = vec![0.0f64; size];

    for t in responses.iter() {
        if base_indexs.contains_key(&(*t).currency_pair) {
            let index = *base_indexs.get(&(*t).currency_pair).unwrap();
            let price: f64 = (*t).last.parse()?;
            vec_prices[index] = price;
            let volume: f64 = (*t).base_volume.parse()?;
            vec_volumes[index] = volume;
        }
    }
    Ok((vec_prices, vec_volumes))
}
