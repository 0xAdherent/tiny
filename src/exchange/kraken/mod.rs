use crate::misc;
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;
use std::collections::HashMap;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct KrakenTickerResponse {
    pub result: HashMap<String, KrakenTicker>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct KrakenTicker {
    pub c: Vec<String>,
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    _invalid_time: u64,
) -> Result<Vec<f64>> {
    let request_url: String = utils::get_latest_price_url("USDT", "ZUSD");
    let response: KrakenTickerResponse = request::request(&request_url).await?;
    if response.result.len() == 0 {
        return Err(anyhow::anyhow!("karken USDT missing attribute data"));
    }
    let base_indexs = utils::get_pairs(bases, currency);

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];

    let pair = "USDTZUSD";
    let op = response.result.get(pair);
    if op.is_none() {
        return Err(anyhow::anyhow!("get karken USDT price failed"));
    }

    let ticker = op.unwrap();
    if ticker.c.len() != 2 {
        return Err(anyhow::anyhow!("get karken USDT price error"));
    }

    let index = *base_indexs.get(pair).unwrap();
    let price: f64 = ticker.c[0].parse()?;
    vec_prices[index] = price;

    Ok(vec_prices)
}
