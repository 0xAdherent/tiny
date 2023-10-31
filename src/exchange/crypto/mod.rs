use crate::misc;
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;

mod utils;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CryptoTickerResponse {
    pub id: i64,
    pub method: String,
    pub code: i64,
    pub result: CryptoResult,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CryptoResult {
    pub data: Vec<CryptoData>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CryptoData {
    pub i: String,
    pub h: String,
    pub l: String,
    pub a: String,
    pub v: String,
    pub vv: String,
    pub c: String,
    pub b: String,
    pub k: String,
    pub t: u64,
}

pub async fn get_latest_price(base: &str, currency: &str) -> Result<f64> {
    let request_url: String = utils::get_latest_price_url(base, currency);
    let response: CryptoTickerResponse = request::request(&request_url).await?;
    if response.result.data.len() == 0 {
        return Err(anyhow::anyhow!("Crypto {} missing attribute data", base));
    }
    let price: f64 = response.result.data[0].a.parse()?;
    Ok(price)
}

pub async fn get_latest_price_v2(
    bases: &Vec<String>,
    currency: &str,
    invalid_time: u64,
) -> Result<Vec<f64>> {
    let request_url: String = utils::get_latest_price_url("USDT", "USD");
    let response: CryptoTickerResponse = request::request(&request_url).await?;
    if response.result.data.len() == 0 {
        return Err(anyhow::anyhow!("Crypto USDT missing attribute data"));
    }
    let base_indexs = utils::get_pairs(bases, currency);

    let size = bases.len();
    let mut vec_prices = vec![0.0f64; size];

    let ts: u64 = response.result.data[0].t;
    let current_ts = misc::get_timestamp();
    if current_ts > ts + invalid_time {
        return Ok(vec_prices);
    }

    let index = *base_indexs.get(&response.result.data[0].i).unwrap();
    let price: f64 = response.result.data[0].a.parse()?;
    vec_prices[index] = price;

    Ok(vec_prices)
}
