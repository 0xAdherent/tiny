use std::collections::HashMap;

enum APIVersion {
    V1,
    V2,
}

pub const API_BASE_URL: &str = "https://api-cloud.bitmart.com/spot";

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V1 => String::from("v1"),
        APIVersion::V2 => String::from("v2"),
    }
}

pub fn get_latest_price_url(base: &str, currency: &str) -> String {
    return format!(
        "{}/{}/ticker_detail?symbol={}_{}",
        API_BASE_URL,
        get_api_version_string(APIVersion::V1),
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
        base.push('_');
        base.push_str(currency);

        symbols.insert(base, idx);
    }
    symbols
}

pub fn get_latest_price_url_v2() -> String {
    return format!(
        "{}/{}/ticker",
        API_BASE_URL,
        get_api_version_string(APIVersion::V2)
    );
}
