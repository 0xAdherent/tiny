use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::storage::Storage;

mod binance;
mod bitget;
mod bitmart;
mod bybit;
mod coinbase;
mod crypto;
mod gate;
mod huobi;
mod kraken;
mod mexc;
mod okx;

pub async fn get_binance_price_v2(
    binance_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let binance_task = tokio::spawn(async move {
        match binance::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("binance: {:#?}", prices);
                {
                    let mut locked_prices = binance_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices.0[i];
                        locked_prices.tickers[index].volumes[i] = prices.1[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    binance_task
}

pub async fn get_binance_usdt_price_v2(
    binance_usdt_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let binance_usdt_task = tokio::spawn(async move {
        match binance::get_usdt_latest_price_v2(&bases, &cur, invalid_time)
            .await
        {
            Ok((prices, usdt_index)) => {
                println!("binance: {:#?}", prices);
                {
                    let mut locked_prices =
                        binance_usdt_shared_prices.lock().await;
                    locked_prices.tickers[index].prices[usdt_index] =
                        prices[usdt_index];
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    binance_usdt_task
}

pub async fn get_huobi_price_v2(
    huobi_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let huobi_task = tokio::spawn(async move {
        match huobi::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("huobi: {:#?}", prices);
                {
                    let mut locked_prices = huobi_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices.0[i];
                        locked_prices.tickers[index].volumes[i] = prices.1[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    huobi_task
}

pub async fn get_okx_price_v2(
    okx_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let okx_task = tokio::spawn(async move {
        match okx::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("okx: {:#?}", prices);
                {
                    let mut locked_prices = okx_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices.0[i];
                        locked_prices.tickers[index].volumes[i] = prices.1[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    okx_task
}

pub async fn get_mexc_price_v2(
    mexc_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let mexc_task = tokio::spawn(async move {
        match mexc::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("mexc: {:#?}", prices);
                {
                    let mut locked_prices = mexc_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices.0[i];
                        locked_prices.tickers[index].volumes[i] = prices.1[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    mexc_task
}

pub async fn get_bybit_price_v2(
    bybit_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let bybit_task = tokio::spawn(async move {
        match bybit::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("bybit: {:#?}", prices);
                {
                    let mut locked_prices = bybit_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices.0[i];
                        locked_prices.tickers[index].volumes[i] = prices.1[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    bybit_task
}

pub async fn get_bitmart_price_v2(
    bitmart_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let bitmart_task = tokio::spawn(async move {
        match bitmart::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("bitmart: {:#?}", prices);
                {
                    let mut locked_prices = bitmart_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices.0[i];
                        locked_prices.tickers[index].volumes[i] = prices.1[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    bitmart_task
}

pub async fn get_bitget_price_v2(
    bitget_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let bitget_task = tokio::spawn(async move {
        match bitget::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("bitget: {:#?}", prices);
                {
                    let mut locked_prices = bitget_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices.0[i];
                        locked_prices.tickers[index].volumes[i] = prices.1[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    bitget_task
}

pub async fn get_gate_price_v2(
    gate_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let gate_task = tokio::spawn(async move {
        match gate::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("gate: {:#?}", prices);
                {
                    let mut locked_prices = gate_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices.0[i];
                        locked_prices.tickers[index].volumes[i] = prices.1[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    gate_task
}

pub async fn get_coinbase_price_v2(
    coinbase_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let coinbase_task = tokio::spawn(async move {
        match coinbase::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("coinbase: {:#?}", prices);
                {
                    let mut locked_prices = coinbase_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    coinbase_task
}

pub async fn get_crypto_price_v2(
    crypto_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let crypto_task = tokio::spawn(async move {
        match crypto::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("crypto: {:#?}", prices);
                {
                    let mut locked_prices = crypto_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    crypto_task
}

pub async fn get_kraken_price_v2(
    kraken_shared_prices: Arc<Mutex<Storage>>,
    coins: &Vec<String>,
    currency: &str,
    index: usize,
    invalid_time: u64,
) -> JoinHandle<()> {
    let bases = coins.clone();
    let cur = currency.to_string();
    let len = coins.len();
    let kraken_task = tokio::spawn(async move {
        match kraken::get_latest_price_v2(&bases, &cur, invalid_time).await {
            Ok(prices) => {
                println!("kraken: {:#?}", prices);
                {
                    let mut locked_prices = kraken_shared_prices.lock().await;
                    for i in 0..len {
                        locked_prices.tickers[index].prices[i] = prices[i];
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
    kraken_task
}
