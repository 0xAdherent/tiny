use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub interval: u64,
    pub coins: Vec<String>,
    pub decimals: Vec<u64>,
    pub imitations: Option<HashMap<String, f64>>,
    pub package_id: String,
    pub oracle_cap: String,
    pub price_oracle: String,
    pub smtp: String,
    pub port: u16,
    pub from: String,
    pub to: String,
    pub username: String,
    pub password: String,
    pub algorithms: Vec<String>,
    pub active: u8,
    pub diffs: HashMap<String, f64>,
    pub ratio: f64,
    pub balance: u64,
    pub gas_budget: u64,
    pub enable_balance_alarm: bool,
    pub enable_price_alarm: bool,
    pub daemon: bool,
    pub single: bool,
    pub log_cfg: bool,
    pub invalid_time: u64,
    pub check_balance_interval: u64,
    pub job: String,
    pub url: String,
    pub instance: String,
    pub desc: String,
    pub prom_username: String,
    pub prom_password: String,
    pub ip: String,
    pub env: String,
    pub account: String,
    pub interactive: bool,
    pub use_multi: bool,
    pub multi_address: String,
    pub publickeys: Vec<String>,
    pub weights: Vec<u8>,
    pub threshold: u16,
    pub gas: String,
    pub usdt_active: u8,
    pub rpcs: Vec<String>,
}

pub struct SuiKey {
    pub key: String,
    pub mnemonic: String,
}

pub fn get_sui_config_path() -> Option<String> {
    let current_path = std::env::current_exe().ok()?;
    let parent_path = current_path.parent().unwrap();

    let mut config_path = parent_path.to_path_buf();
    config_path.push(r"sui_config");
    config_path.push(r"client.yaml");

    let path = config_path.into_os_string().into_string().unwrap();
    Some(path)
}

pub fn get_log_path(logfile: &str) -> Option<String> {
    let current_path = std::env::current_exe().ok()?;
    let parent_path = current_path.parent().unwrap();

    let mut config_path = parent_path.to_path_buf();
    config_path.push(logfile);

    let path = config_path.into_os_string().into_string().unwrap();
    Some(path)
}

pub fn read_configuration() -> Result<Configuration> {
    let current_path = std::env::current_exe()?;
    let parent_path = current_path.parent().unwrap();

    let mut config_path = parent_path.to_path_buf();
    config_path.push(r"tiny.yaml");

    let f = std::fs::File::open(config_path)?;
    let cfg: Configuration =
        serde_yaml::from_reader(f).expect("tiny.yaml read failed!");
    Ok(cfg)
}
