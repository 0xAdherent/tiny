use utils::PriceList;

mod utils;

pub fn average(data: Vec<f64>) -> (bool, f64) {
    let pl = PriceList::new(data, vec![]);
    let average = match pl.average() {
        Ok(a) => a,
        Err(_) => return (false, 0.0),
    };
    println!("average {:?}", average);
    (true, average)
}

pub fn median(data: Vec<f64>) -> (bool, f64) {
    let p1 = PriceList::new(data, vec![]);
    let median = match p1.median() {
        Ok(m) => m,
        Err(_) => return (false, 0.0),
    };
    println!("median {:?}", median);
    (true, median)
}

pub fn backwad(
    data: Vec<f64>,
    diff_percent: u16,
    expected_ratio: u16,
) -> (bool, f64) {
    let p1 = PriceList::new(data, vec![]);
    let backwad = match p1.backwad(diff_percent, expected_ratio) {
        Ok(b) => b,
        Err(_) => return (false, 0.0),
    };
    println!("backwad {:?}", backwad);
    (true, backwad)
}

pub fn weighted(data: Vec<f64>, volume: Vec<f64>) -> (bool, f64) {
    let pl = PriceList::new(data, volume);
    let average = match pl.weighted_average() {
        Ok(a) => a,
        Err(_) => return (false, 0.0),
    };
    println!("weighted average {:?}", average);
    (true, average)
}

pub fn max(data: Vec<f64>) -> (bool, f64) {
    let p1 = PriceList::new(data, vec![]);
    let m = match p1.max() {
        Ok(m) => m,
        Err(_) => return (false, 0.0),
    };
    println!("max {:?}", m);
    (true, m)
}

pub fn switch_algo(
    algo: &str,
    data: Vec<f64>,
    volume: Vec<f64>,
    diff_percent: Option<f64>,
    expected_ratio: Option<f64>,
) -> (bool, f64) {
    let mut diff = 0u16;
    let mut ratio = 0u16;

    if algo == "backwad" {
        if diff_percent.is_none() || expected_ratio.is_none() {
            return (false, 0.0);
        }
        diff = (diff_percent.unwrap() * 100.0) as u16;
        ratio = (expected_ratio.unwrap() * 100.0) as u16;
    }
    match algo {
        "average" => average(data),
        "median" => median(data),
        "backwad" => backwad(data, diff, ratio),
        "weighted" => weighted(data, volume),
        "max" => max(data),
        _ => (false, 0.0),
    }
}
