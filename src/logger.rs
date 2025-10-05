use log::LevelFilter;
pub use log::{error, warn};
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn init() {
        env_logger::builder()
            .filter_level(LevelFilter::Warn)
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{}] {}",
                    buf.default_level_style(record.level()),
                    record.args()
                )
            })
            .init()
    }
}
