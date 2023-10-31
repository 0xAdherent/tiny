use mail_send::mail_builder::MessageBuilder;
use mail_send::{Error, SmtpClientBuilder};
use std::sync::atomic::{AtomicU64, Ordering};

static GID: AtomicU64 = AtomicU64::new(1);
pub const BASE_SUI_UNIT: u64 = 1000000000;

#[derive(Debug, Clone)]
pub enum AlarmType {
    Balance,
    Price,
}

#[derive(Debug, Clone)]
pub struct Alarm {
    pub message_id: u64,
    pub alarm_type: AlarmType,
    pub subject: String,
    pub message: String,
}

impl Alarm {
    pub fn new(
        message_id: u64,
        alarm_type: AlarmType,
        subject: String,
        message: String,
    ) -> Alarm {
        let mut msg_id = message_id;
        if message_id == 0 {
            let pre_gid = GID.fetch_add(1, Ordering::SeqCst);
            msg_id = pre_gid;
        }
        Alarm {
            message_id: msg_id,
            subject,
            alarm_type,
            message,
        }
    }

    pub async fn send_mail(
        &self,
        from: &str,
        to: &str,
        smtp: &str,
        port: u16,
        user: &str,
        password: &str,
    ) -> Result<(), Error> {
        let builder = MessageBuilder::new()
            .from(("", from))
            .to(("", to))
            .subject(&self.subject)
            .text_body(&self.message);
        let mut client = match SmtpClientBuilder::new(smtp, port)
            .implicit_tls(false)
            .credentials((user, password))
            .connect()
            .await
        {
            Ok(client) => client,
            Err(e) => {
                println!("connect failed: {}", e);
                return Err(e);
            }
        };
        client.send(builder).await
    }
}

pub fn new_balance_alarm(balance: u64, threshold: u64) -> Alarm {
    let balance = balance as f64 / BASE_SUI_UNIT as f64;
    let threshold = threshold as f64 / BASE_SUI_UNIT as f64;

    Alarm::new(
        0,
        AlarmType::Balance,
        "Balance Alarm".to_string(),
        format!("Balance: {}, below {}", balance, threshold),
    )
}

pub fn new_price_alarm(desc: &str) -> Alarm {
    Alarm::new(
        0,
        AlarmType::Price,
        "Price Alarm".to_string(),
        desc.to_owned(),
    )
}
