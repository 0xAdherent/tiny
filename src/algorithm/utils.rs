use anyhow::{self, Ok, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("divide by zero")]
    DivideByZero,
    #[error("data len {0} too short")]
    DataLenTooShort(usize),
    #[error("master price missing")]
    MasterPriceMissing,
    #[error("actual ratio {0} too low")]
    ActualRatioTooLow(u16),
}

#[derive(Debug, Clone)]
pub struct PriceList {
    pub data: Vec<f64>,
    pub volume: Vec<f64>,
}

impl PriceList {
    pub fn new(data: Vec<f64>, volume: Vec<f64>) -> PriceList {
        PriceList { data, volume }
    }

    pub fn median(&self) -> Result<f64> {
        let mut v: Vec<f64> = self
            .data
            .clone()
            .into_iter()
            .filter(|x| *x != 0.0)
            .collect();

        let len = v.len();
        if len == 0 {
            return Err(DataError::DivideByZero.into());
        }

        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let d = (len / 2) as usize;
        let median = match len % 2 {
            1 => v[d],
            _ => {
                let p = d - 1;
                (v[p] + v[d]) / 2.0
            }
        };
        Ok(median)
    }

    pub fn average(&self) -> Result<f64> {
        let v: Vec<f64> = self
            .data
            .clone()
            .into_iter()
            .filter(|x| *x != 0.0)
            .collect();
        let sum = v.iter().sum::<f64>();
        let len = v.len();
        if len == 0 {
            return Err(DataError::DivideByZero.into());
        }
        Ok(sum / len as f64)
    }

    pub fn weighted_average(&self) -> Result<f64> {
        let weight_sum = self.volume.iter().sum::<f64>();
        let mut weight_price = 0.0f64;

        for (i, v) in self.data.iter().enumerate() {
            if *v == 0.0 || self.volume[i] == 0.0 {
                continue;
            }
            weight_price += v * self.volume[i];
        }

        if weight_sum == 0.0 || weight_price == 0.0 {
            return Err(DataError::DivideByZero.into());
        }
        Ok(weight_price / weight_sum)
    }

    pub fn backwad(
        &self,
        diff_percent: u16,
        expected_ratio: u16,
    ) -> Result<f64> {
        let len = self.data.len();
        if len < 4 {
            return Err(DataError::DataLenTooShort(len).into());
        }

        let mut master_price = self.data[0];
        if master_price == 0.0f64 {
            master_price = self.data[1];
        }

        if master_price == 0.0f64 {
            return Err(DataError::MasterPriceMissing.into());
        }

        let v: Vec<f64> = self
            .data
            .clone()
            .into_iter()
            .filter(|x| *x != 0.0)
            .collect();

        let vlen = v.len();
        let mut actual_len: usize = 0;
        for d in v.iter() {
            let diff = master_price - *d;
            let diff_pre = (diff.abs() * 100.0 / master_price) as u16;
            if diff_pre <= diff_percent {
                actual_len += 1;
            }
        }

        let actual_ratio = (actual_len as f64 * 100.0 / vlen as f64) as u16;
        if actual_ratio < expected_ratio {
            return Err(DataError::ActualRatioTooLow(actual_ratio).into());
        }

        Ok(master_price)
    }

    pub fn max(&self) -> Result<f64> {
        let v: Vec<f64> = self
            .data
            .clone()
            .into_iter()
            .filter(|x| *x != 0.0)
            .collect();
        let len = v.len();
        if len == 0 {
            return Err(DataError::DataLenTooShort(len).into());
        }
        let mut m = 0.0f64;
        for i in 0..len {
            if v[i] > m {
                m = v[i];
            }
        }
        Ok(m)
    }
}
