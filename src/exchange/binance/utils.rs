use std::collections::HashMap;

enum APIVersion {
    V3,
}

pub const API_BASE_URL: &str = "https://api.binance.com";
pub const API_BASE_US_URL: &str = "https://api.binance.us";

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V3 => String::from("v3"),
    }
}

pub fn get_latest_price_url(base: &str, currency: &str) -> String {
    return format!(
        "{}/api/{}/ticker?symbol={}{}",
        API_BASE_US_URL,
        get_api_version_string(APIVersion::V3),
        base,
        currency
    );
}

pub fn get_pairs(
    bases: &Vec<String>,
    currency: &str,
) -> HashMap<String, usize> {
    let mut symbols = HashMap::new();
    for (idx, val) in bases.iter().enumerate() {
        let mut base = (*val).clone();
        base.push_str(currency);

        symbols.insert(base, idx);
    }
    symbols
}

pub fn get_latest_price_url_v2(bases: &Vec<String>, currency: &str) -> String {
    let mut symbols: Vec<String> = Vec::new();
    for i in bases.iter() {
        let mut base = (*i).clone();
        if base == "USDT" {
            continue;
        }
        base.push_str(currency);

        symbols.push(base);
    }

    format!(
        "{}/api/{}/ticker?type=MINI&symbols=[\"{}\"]",
        API_BASE_URL,
        get_api_version_string(APIVersion::V3),
        symbols.join("\",\"")
    )
}
