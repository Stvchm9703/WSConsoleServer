use log4rs::{
  append::{
    console::{ConsoleAppender, Target},
    file::FileAppender,
  },
  config::{Appender, Config, Root},
  encode::pattern::PatternEncoder,
};

use chrono::{DateTime, Utc};

pub fn log_setup() {
  // log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
  let level = log::LevelFilter::Info;
  let now: DateTime<Utc> = Utc::now();
  let file_path = format!(
    "./log/{}/debug.{}.log",
    now.format("%Y-%m-%d"),
    now.format("%H-%M-%S")
  );

  // Build a stderr logger.
  let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

  // Logging to log file.
  // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(
      "{d(%Y-%m-%d %H:%M:%S.%3f %Z)} {l} - {m}{n}",
    )))
    .build(file_path)
    .unwrap();
  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .appender(Appender::builder().build("stderr", Box::new(stderr)))
    .build(
      Root::builder()
        .appender("logfile")
        .appender("stderr")
        .build(level),
    )
    .unwrap();
  log4rs::init_config(config).unwrap();
}
