#[derive(Debug, Clone)]
pub struct Ticker {
    pub prices: Vec<f64>,
    pub volumes: Vec<f64>,
}

impl Ticker {
    pub fn new(token_size: usize) -> Self {
        Ticker {
            prices: vec![0.0f64; token_size],
            volumes: vec![0.0f64; token_size],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Storage {
    pub tickers: Vec<Ticker>,
}

impl Storage {
    pub fn new(exchange_size: usize, token_size: usize) -> Self {
        let tickers = vec![Ticker::new(token_size); exchange_size];
        Storage { tickers: tickers }
    }
}
