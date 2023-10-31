use std::collections::HashMap;

enum APIVersion {
    V2,
}

pub const API_BASE_URL: &str = "https://api.coinbase.com";

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V2 => String::from("v2"),
    }
}

pub fn get_latest_price_url(base: &str, currency: &str) -> String {
    return format!(
        "{}/{}/prices/{}-{}/spot",
        API_BASE_URL,
        get_api_version_string(APIVersion::V2),
        base,
        currency
    );
}

pub fn get_pairs(bases: &Vec<String>, _: &str) -> HashMap<String, usize> {
    let mut symbols = HashMap::new();
    for (idx, val) in bases.iter().enumerate() {
        symbols.insert(val.clone(), idx);
    }
    symbols
}
