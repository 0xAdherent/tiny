use anyhow::Result;

use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::{
    policy::compound::{
        roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
        CompoundPolicy,
    },
    RollingFileAppender,
};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;

use crate::configuration;

const ROLL_PATTERN: &str = "tiny.log.{}";
const LOG_FILE_NAME: &str = "tiny.log";
const DEFAULT_DAEMON_FILE_FORMAT: &str = "{h({d(%m-%d-%Y %H:%M:%S)})} - {m}{n}";
const DEFAULT_CONSOLE_FORMAT: &str = "{h({d(%H:%M:%S)})} - {m}{n}";
const SIZE_LIMIT: u64 = 1024 * 1024 * 10;
const LOG: &str = "log";
const STDOUT: &str = "stdout";

#[derive(Debug, Clone)]
pub struct Logger {
    pub roll_name: String,
    pub file_name: String,
    pub use_cfg: bool,
    pub open: bool,
}

impl Logger {
    pub fn new(open: bool, use_cfg: bool) -> Logger {
        let roll_file = configuration::get_log_path(ROLL_PATTERN).unwrap();
        let log_file = configuration::get_log_path(LOG_FILE_NAME).unwrap();
        Logger {
            roll_name: roll_file,
            file_name: log_file,
            use_cfg,
            open,
        }
    }

    pub fn init(&self) -> Result<Handle> {
        let console = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(DEFAULT_CONSOLE_FORMAT)))
            .build();
        let fixed_window_roller =
            FixedWindowRoller::builder().build(&self.roll_name, 10)?;

        let size_trigger = SizeTrigger::new(SIZE_LIMIT);

        let compound_policy = CompoundPolicy::new(
            Box::new(size_trigger),
            Box::new(fixed_window_roller),
        );

        let encoder = PatternEncoder::new(DEFAULT_DAEMON_FILE_FORMAT);

        let appender = RollingFileAppender::builder()
            .encoder(Box::new(encoder))
            .append(true)
            .build(&self.file_name, Box::new(compound_policy))?;

        let file = Appender::builder().build(LOG, Box::new(appender));
        let stdout = Appender::builder().build(STDOUT, Box::new(console));

        let root = Root::builder()
            .appender(STDOUT)
            .appender(LOG)
            .build(LevelFilter::Info);

        let config = Config::builder()
            .appender(stdout)
            .appender(file)
            .build(root)?;

        Ok(log4rs::init_config(config)?)
    }
}

pub fn init_logger(open: bool, use_cfg: bool) {
    if use_cfg {
        let log_path = configuration::get_log_path("log4rs.yaml");
        log4rs::init_file(&log_path.unwrap(), Default::default()).unwrap();
        return;
    }

    let logger = Logger::new(open, use_cfg);
    match logger.init() {
        Ok(_) => println!("logger init succeeded"),
        Err(e) => eprint!("logger init failed: {}", e),
    };
}
