use lazy_static::lazy_static;
use prometheus::{labels, register_gauge, register_int_gauge, Gauge, IntGauge};

use crate::misc;

lazy_static! {
    static ref BALANCE_GAUGE: Gauge =
        register_gauge!("balance_status", "help").unwrap();
    static ref TS_GAUGE: IntGauge =
        register_int_gauge!("push_timestamp", "help").unwrap();
}

#[derive(Debug, Clone)]
pub struct Prom {
    pub job: String,
    pub url: String,
    pub instance: String,
    pub desc: String,
    pub username: String,
    pub password: String,
}

impl Prom {
    pub fn new(
        job: String,
        url: String,
        instance: String,
        desc: String,
        username: String,
        password: String,
    ) -> Self {
        Prom {
            job,
            url,
            instance,
            desc,
            username,
            password,
        }
    }

    pub fn push(
        &self,
        v: f64,
        ip: &String,
        env: &String,
        account: &String,
        _balance: &String,
    ) {
        BALANCE_GAUGE.set(v);
        TS_GAUGE.set(misc::get_timestamp() as i64);
        let metric_families = prometheus::gather();

        match prometheus::push_metrics(
            &self.job,
            labels! {
                "ip".to_owned() => ip.to_owned(),
                "env".to_owned() => env.to_owned(),
                "account".to_owned() => account.to_owned(),
            },
            &self.url,
            metric_families,
            Some(prometheus::BasicAuthentication {
                username: self.username.to_owned(),
                password: self.password.to_owned(),
            }),
        ) {
            Ok(()) => {
                println!("push successed");
            }
            Err(e) => eprintln!("push failed: {}", e),
        }
    }
}
