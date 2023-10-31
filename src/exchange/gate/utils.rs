use std::collections::HashMap;

enum APIVersion {
    V4,
}

pub const API_BASE_URL: &str = "https://api.gateio.ws/api";

fn get_api_version_string(version: APIVersion) -> String {
    match version {
        APIVersion::V4 => String::from("v4"),
    }
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
        "{}/{}/spot/tickers",
        API_BASE_URL,
        get_api_version_string(APIVersion::V4)
    );
}
