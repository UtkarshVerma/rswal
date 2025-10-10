pub use log::error;
use log::LevelFilter;
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn init() {
        env_logger::builder()
            .filter_level(LevelFilter::Warn)
            .format(|buf, record| {
                let level_style = buf.default_level_style(record.level());
                writeln!(
                    buf,
                    "[{level_style}{}{level_style:#}] {}",
                    record.level().to_string().to_lowercase(),
                    record.args()
                )
            })
            .init()
    }
}
