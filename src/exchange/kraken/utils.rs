use std::collections::HashMap;

enum APIVersion {
    V0,
}

pub const API_BASE_URL: &str = "https://api.kraken.com";

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V0 => String::from("0"),
    }
}

pub fn get_latest_price_url(base: &str, currency: &str) -> String {
    return format!(
        "{}/{}/public/Ticker?pair={}{}",
        API_BASE_URL,
        get_api_version_string(APIVersion::V0),
        base,
        currency
    );
}

pub fn get_pairs(bases: &Vec<String>, _: &str) -> HashMap<String, usize> {
    let mut symbols = HashMap::new();
    for (idx, val) in bases.iter().enumerate() {
        let mut base = (*val).clone();
        base.push_str("ZUSD");
        symbols.insert(base, idx);
    }
    symbols
}
