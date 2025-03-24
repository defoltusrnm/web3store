use log::LevelFilter;
use log4rs::{append::console::ConsoleAppender, config::{Appender, Root}, encode::pattern::PatternEncoder, Config, Handle};

use super::errors::AppErr;


pub fn configure_logs(min_level: LevelFilter) -> Result<Handle, AppErr> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(min_level))
        .map_err(|err| AppErr::from_owned(format!("log cfg error: {err}")))?;

    log4rs::init_config(config).map_err(|err| AppErr::from_owned(format!("log init err: {err}")))
}
