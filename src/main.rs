#![allow(dead_code, unused_imports)]
use anyhow::Result;
use clap::Parser;
use configuration::{Configuration, SuiKey};
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::{error, info, warn};
use log4rs::filter::threshold;
use logger::Logger;
use mail::{Alarm, AlarmType};
use postage::{broadcast, broadcast::Sender, prelude::Stream, sink::Sink};
use prom::Prom;
use single_instance::SingleInstance;
use std::io;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use storage::Storage;
use sui_json::SuiJsonValue;
use sui_sdk::wallet_context::WalletContext;
use tokio::signal;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tokio::time;

mod algorithm;
mod configuration;
mod daemon;
mod exchange;
mod logger;
mod mail;
mod misc;
mod mov;
mod prom;
mod request;
mod storage;

pub const EXCHANGE_SIZE: usize = 20;
pub const ORACLE_MODULE: &str = "oracle";
pub const ORACLE_FEED_FUNCTION: &str = "update_token_price_batch";
pub const SINGLE_INSTANCE_ID: &str = "Tiny Oracle Feeder";

lazy_static! {
    static ref CFG: Configuration =
        configuration::read_configuration().unwrap();
    static ref PROM: Prom = Prom::new(
        CFG.job.clone(),
        CFG.url.clone(),
        CFG.instance.clone(),
        CFG.desc.clone(),
        CFG.prom_username.clone(),
        CFG.prom_password.clone()
    );
    static ref SUIKEY: std::sync::Mutex<SuiKey> = {
        let sk = SuiKey {
            key: "".to_owned(),
            mnemonic: "".to_owned(),
        };
        std::sync::Mutex::new(sk)
    };
    static ref RPCINDEX: std::sync::Mutex<u64> = std::sync::Mutex::new(0);
}

#[derive(Parser, Debug)]
#[clap(
    author,
    version = "0.1.0",
    about = "Tiny Oracle Feeder  - Command Line Interface (CLI) Application"
)]
struct Cli {
    #[clap(short, long, default_value = "10")]
    /// Interval. An example would be an interval of 10 seconds.
    interval: u64,

    #[clap(short, long, default_value = "")]
    /// Private key. Base64 format.
    key: String,

    #[clap(short, long, default_value = "")]
    /// Mnemonic. BIP 39.
    mnemonic: String,
}

fn is_coin_analog(symbol: &str) -> (bool, f64) {
    if CFG.imitations.is_some() {
        let imitations = CFG.imitations.clone().unwrap();
        let result = imitations.get(symbol);
        if result.is_some() {
            return (true, *result.unwrap());
        }
    }

    (false, 0.0f64)
}

fn get_coin_price(
    symbol: &str,
    idx: usize,
    locked_prices: &MutexGuard<'_, Storage>,
) -> (bool, f64) {
    let coin_price;
    let count = CFG.coins.len();
    let (coin_analog, coin_pri) = is_coin_analog(symbol);
    if coin_analog {
        coin_price = coin_pri;
    } else {
        let data = get_price_info_v2(locked_prices, idx, count);

        let len = CFG.algorithms.len();
        let idx;
        if symbol != "USDT" {
            idx = (CFG.active as usize % len) as usize;
        } else {
            idx = (CFG.usdt_active as usize % len) as usize;
        }
        let algo = &CFG.algorithms[idx];

        let mut diff = 0.001f64;
        if CFG.diffs.contains_key(symbol) {
            diff = *CFG.diffs.get(symbol).unwrap();
        }

        let (success, tmp_price) = algorithm::switch_algo(
            algo,
            data.0,
            data.1,
            Option::Some(diff),
            Option::Some(CFG.ratio),
        );

        if !success {
            error!("get {} price failed", symbol);
            return (false, 0.0);
        }
        coin_price = tmp_price;
    }
    (true, coin_price)
}

async fn get_prices(coins: &Vec<String>) -> Result<Vec<f64>> {
    let prices = Storage::new(EXCHANGE_SIZE, EXCHANGE_SIZE);
    let shared_prices = Arc::new(Mutex::new(prices));

    let binance_shared_prices = shared_prices.clone();
    let binance_task = exchange::get_binance_price_v2(
        binance_shared_prices,
        coins,
        "USDT",
        0,
        CFG.invalid_time,
    )
    .await;

    let okx_shared_prices = shared_prices.clone();
    let okx_task = exchange::get_okx_price_v2(
        okx_shared_prices,
        coins,
        "USDT",
        1,
        CFG.invalid_time,
    )
    .await;

    let huobi_shared_prices = shared_prices.clone();
    let huobi_task = exchange::get_huobi_price_v2(
        huobi_shared_prices,
        coins,
        "USDT",
        2,
        CFG.invalid_time,
    )
    .await;

    let mexc_shared_prices = shared_prices.clone();
    let mexc_task = exchange::get_mexc_price_v2(
        mexc_shared_prices,
        coins,
        "USDT",
        3,
        CFG.invalid_time,
    )
    .await;

    let bybit_shared_prices = shared_prices.clone();
    let bybit_task = exchange::get_bybit_price_v2(
        bybit_shared_prices,
        coins,
        "USDT",
        4,
        CFG.invalid_time,
    )
    .await;

    let bitget_shared_prices = shared_prices.clone();
    let bitget_task = exchange::get_bitget_price_v2(
        bitget_shared_prices,
        coins,
        "USDT",
        5,
        CFG.invalid_time,
    )
    .await;

    let gate_shared_prices = shared_prices.clone();
    let gate_task = exchange::get_gate_price_v2(
        gate_shared_prices,
        coins,
        "USDT",
        6,
        CFG.invalid_time,
    )
    .await;

    let coinbase_shared_prices = shared_prices.clone();
    let coinbase_task = exchange::get_coinbase_price_v2(
        coinbase_shared_prices,
        coins,
        "USD",
        7,
        CFG.invalid_time,
    )
    .await;

    let crypto_shared_prices = shared_prices.clone();
    let crypto_task = exchange::get_crypto_price_v2(
        crypto_shared_prices,
        coins,
        "USD",
        8,
        CFG.invalid_time,
    )
    .await;

    let kraken_shared_prices = shared_prices.clone();
    let kraken_task = exchange::get_kraken_price_v2(
        kraken_shared_prices,
        coins,
        "USD",
        9,
        CFG.invalid_time,
    )
    .await;

    let _ = tokio::try_join!(
        binance_task,
        okx_task,
        huobi_task,
        mexc_task,
        bybit_task,
        bitget_task,
        gate_task,
        coinbase_task,
        crypto_task,
        kraken_task,
    );

    let mut_locked = shared_prices.clone();
    let locked_prices = mut_locked.lock().await;

    let usdt_idx = coins.iter().position(|x| x == "USDT").unwrap();
    info!("usdt idx = {}", usdt_idx);

    let (succeed, usdt_price) =
        get_coin_price("USDT", usdt_idx, &locked_prices);
    if !succeed {
        error!("fetch usdt price failed");
        return Err(anyhow::anyhow!("fetch usdt price failed"));
    }

    let size = coins.len();
    let mut result = vec![0.0f64; size];
    result[usdt_idx] = usdt_price;

    for idx in 0..size {
        if idx == usdt_idx {
            continue;
        }

        let (succeed, coin_price) =
            get_coin_price(&coins[idx], idx, &locked_prices);
        if !succeed {
            error!("get {} price failed", coins[idx]);
            continue;
        }

        result[idx] = coin_price * usdt_price;
    }

    return Ok(result);
}

fn get_price_info(
    prices: &MutexGuard<'_, Storage>,
    index: usize,
) -> (usize, f64) {
    let mut coin_len = 0;
    let mut coin_sum = 0.0f64;
    for i in 0..EXCHANGE_SIZE {
        if prices.tickers[i].prices[index] > 0.0f64 {
            coin_len = coin_len + 1;
            coin_sum = coin_sum + prices.tickers[i].prices[index];
        }
    }
    (coin_len, coin_sum)
}

fn get_price_info_v2(
    prices: &MutexGuard<'_, Storage>,
    index: usize,
    _count: usize,
) -> (Vec<f64>, Vec<f64>) {
    let mut p = vec![0.0f64; EXCHANGE_SIZE];
    let mut v = vec![0.0f64; EXCHANGE_SIZE];
    for i in 0..EXCHANGE_SIZE {
        if prices.tickers[i].prices[index] > 0.0f64 {
            p[i] = prices.tickers[i].prices[index];
            v[i] = prices.tickers[i].volumes[index];
        }
    }
    (p, v)
}

async fn handle_alarm_messages(mut rx: impl Stream<Item = Alarm> + Unpin) {
    while let Some(alarm) = rx.recv().await {
        info!("{} got a message: {}", alarm.message_id, alarm.message);
        match alarm
            .send_mail(
                &CFG.from,
                &CFG.to,
                &CFG.smtp,
                CFG.port,
                &CFG.username,
                &CFG.password,
            )
            .await
        {
            Ok(_) => warn!("send mail successed"),
            Err(e) => error!("send mail failed: {}", e),
        }
    }
}

fn get_sui_key() -> (String, String) {
    let key = { SUIKEY.lock().unwrap().key.clone() };
    let mne = { SUIKEY.lock().unwrap().mnemonic.clone() };

    (key, mne)
}

fn get_nex_rpc() -> String {
    let mut idx = RPCINDEX.lock().unwrap();
    *idx += 1;

    let i = (*idx as usize) % CFG.rpcs.len();
    println!("rpc index: {}", i);
    CFG.rpcs[i].clone()
}

async fn handle_price_messages(
    mut rx: impl Stream<Item = (Vec<u8>, Vec<u64>, u64)> + Unpin,
) {
    let sui_config_path = configuration::get_sui_config_path().unwrap();
    let (key, mne) = get_sui_key();
    let mut wallet = mov::init_wallet(&sui_config_path, &key, &mne)
        .await
        .unwrap();
    let mut check_balance_ts = misc::get_timestamp();
    while let Some(price) = rx.recv().await {
        let current_timestamp = misc::get_timestamp();
        if current_timestamp - check_balance_ts > CFG.check_balance_interval {
            check_balance(&mut wallet).await;
            check_balance_ts = current_timestamp;
        }

        if current_timestamp - price.2 > CFG.interval {
            continue;
        }

        let json_params = mov::pack_params(
            &CFG.oracle_cap,
            &CFG.price_oracle,
            &price.0,
            &price.1,
        )
        .unwrap();

        if !send_tx(&mut wallet, json_params.clone(), &price.1).await {
            let _ = wallet.set_client(get_nex_rpc());
            send_tx(&mut wallet, json_params, &price.1).await;
        }
    }
}

async fn send_tx(
    wallet: &mut WalletContext,
    json_params_v3: Vec<SuiJsonValue>,
    price: &Vec<u64>,
) -> bool {
    if !CFG.use_multi {
        match mov::call(
            wallet,
            &CFG.package_id,
            &ORACLE_MODULE.to_string(),
            &ORACLE_FEED_FUNCTION.to_string(),
            CFG.gas_budget,
            json_params,
        )
        .await
        {
            Ok(_) => info!("ok, set prices {:?}", price),
            Err(e) => {
                error!("call: {}", e);
                return false;
            }
        };
    } else {
        match mov::multi_call(
            wallet,
            &CFG.package_id,
            &ORACLE_MODULE.to_string(),
            &ORACLE_FEED_FUNCTION.to_string(),
            &CFG.gas,
            CFG.gas_budget,
            json_params,
            &CFG.publickeys,
            &CFG.weights,
            CFG.threshold,
        )
        .await
        {
            Ok(_) => info!("ok, set prices {:?}", price),
            Err(e) => {
                error!("multi call: {}", e);
                return false;
            }
        };
    }

    true
}

async fn check_balance(wallet: &mut WalletContext) {
    if !CFG.enable_balance_alarm {
        return;
    }
    let balance;
    if !CFG.use_multi {
        balance = mov::get_balance(wallet).await;
    } else {
        balance =
            mov::get_multi_balance(wallet, &CFG.multi_address, &CFG.gas).await;
    }

    let mut v = 0u64;
    match balance {
        Ok(balance) => {
            v = balance;
        }
        Err(e) => error!("get balance failed: {}", e),
    }

    if v == 0 {
        return;
    }

    let fv = v as f64 / 1000000000.0;
    PROM.push(fv, &CFG.ip, &CFG.env, &CFG.account, &fv.to_string());
}

fn set_signal_handler(r: Arc<AtomicBool>) {
    _ = tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                r.store(false, Ordering::SeqCst);
                warn!("catch ctrl + c signal");
            }
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
            }
        };
    });
}

fn interactive() {
    if CFG.interactive {
        loop {
            println!("Please enter a private key or mnemonic:");
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                continue;
            }

            let strip_input: String = input
                .trim()
                .split_inclusive(char::is_whitespace)
                .filter(|part| !part.trim().is_empty())
                .collect();
            if strip_input.is_empty() {
                continue;
            }

            if mov::utils::is_valid_base64_key(&strip_input) {
                SUIKEY.lock().unwrap().key = strip_input;
            } else if mov::utils::is_valid_mnemonic(&strip_input) {
                SUIKEY.lock().unwrap().mnemonic = strip_input;
            } else {
                println!("The input format is incorrect!");
                continue;
            }
            break;
        }
    }
}

fn parse_args(args: &Cli) {
    let opt_key = std::env::var_os("KEY");
    if opt_key.is_some() {
        SUIKEY.lock().unwrap().key = opt_key.unwrap().into_string().unwrap();
    }

    let opt_mne = std::env::var_os("MNEMONIC");
    if opt_mne.is_some() {
        SUIKEY.lock().unwrap().mnemonic =
            opt_mne.unwrap().into_string().unwrap();
    }

    if &args.key.len() > &0 {
        SUIKEY.lock().unwrap().key = args.key.clone();
    }

    if &args.mnemonic.len() > &0 {
        SUIKEY.lock().unwrap().mnemonic = args.mnemonic.clone();
    }

    let (key, mnemonic) = get_sui_key();
    if &key.len() == &0 && &mnemonic.len() == &0 {
        eprintln!("key or mnemonic missing");
        process::exit(0);
    }
}

fn main() {
    let args = Cli::parse();
    interactive();

    parse_args(&args);

    if CFG.daemon {
        daemon::daemonize(true, false);
    }

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            _ = _main(&args).await;
        })
}

async fn _main(args: &Cli) -> Result<()> {
    dotenv().ok();

    if CFG.single {
        let instance = SingleInstance::new(SINGLE_INSTANCE_ID).unwrap();
        if !instance.is_single() {
            eprintln!("program already running...");
            process::exit(0);
        }
    }

    logger::init_logger(true, CFG.log_cfg);
    info!("tinyd started");

    let (mut tx, rx) = broadcast::channel::<mail::Alarm>(100);
    tokio::task::spawn(handle_alarm_messages(rx));

    let (mut tx2, rx2) = broadcast::channel::<(Vec<u8>, Vec<u64>, u64)>(100);
    tokio::task::spawn(handle_price_messages(rx2));

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    set_signal_handler(r);

    let mut count = 0u64;
    let mut interval = CFG.interval;
    if args.interval > interval {
        interval = args.interval;
    }

    warn!("interval: {}", interval);
    warn!("coins: {:?}", CFG.coins);
    warn!("imitations: {:?}", CFG.imitations);

    let mut interval = time::interval(time::Duration::from_secs(interval));
    loop {
        interval.tick().await;

        let coins = CFG.coins.clone();
        let result = get_prices(&coins).await;
        let prices = match result {
            Ok(res) => res,
            Err(err) => {
                error!("error {:?}", err);
                Vec::new()
            }
        };

        warn!("get coins prices: {:#?}", prices);

        if prices.len() == 0 {
            if CFG.enable_price_alarm {
                let alarm =
                    mail::new_price_alarm("Failed to obtain currency price!");
                _ = tx.send(alarm).await;
            }
            continue;
        }

        let mut coin_idxs = Vec::new();
        let mut price_vals = Vec::new();

        for i in 0..coins.len() {
            if prices[i] <= 0.0f64 {
                continue;
            }

            let dec = (10u32.pow(CFG.decimals[i] as u32)) as u64;
            let price = prices[i] * dec as f64;
            let price = price as u64;

            coin_idxs.push(i as u8);
            price_vals.push(price);
        }

        warn!("set coin idxs: {:#?}", coin_idxs);
        warn!("set coins prices: {:?}", price_vals);

        _ = tx2
            .send((coin_idxs, price_vals, misc::get_timestamp()))
            .await;

        count += 1;
        info!("count = {}", count);

        if !running.load(Ordering::SeqCst) {
            warn!("Got it! Exiting...");
            process::exit(0);
        }
    }
}
