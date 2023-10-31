use std::collections::HashMap;

pub const API_BASE_URL: &str = "https://api.huobi.pro/market/detail/merged";
pub const API_ALL_URL: &str = "https://api.huobi.pro//market/tickers";

pub fn get_latest_price_url(base: &str, currency: &str) -> String {
    let mut pair = base.to_string();
    pair.push_str(currency);

    return format!("{}?symbol={}", API_BASE_URL, pair.to_lowercase());
}

pub fn get_pairs(
    bases: &Vec<String>,
    currency: &str,
) -> HashMap<String, usize> {
    let mut symbols = HashMap::new();
    for (idx, val) in bases.iter().enumerate() {
        let mut base = (*val).clone();
        base.push_str(currency);

        symbols.insert(base.to_lowercase(), idx);
    }
    symbols
}

pub fn get_latest_price_url_v2() -> String {
    API_ALL_URL.to_string()
}
