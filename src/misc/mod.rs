use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() * 1000,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
