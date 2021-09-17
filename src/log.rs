#[cfg(feature = "logging")]
pub fn enable_pretty_env_logging() {
  use log::LevelFilter;
  use pretty_env_logger::env_logger::WriteStyle;
  let mut logger = pretty_env_logger::formatted_builder();
  if let Ok(s) = ::std::env::var("RUST_LOG") {
    logger.parse_filters(&s);
  } else {
    logger.filter_level(LevelFilter::Debug);
  }
  if let Ok(s) = ::std::env::var("RUST_LOG_STYLE") {
    logger.parse_write_style(&s);
  } else {
    logger.write_style(WriteStyle::Always);
  }
  logger.init();
}

macro_rules! error {
  (target: $target:expr, $($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::error!($target, $($arg)+); }
  };
  ($($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::error!($($arg)+); }
  };
}

macro_rules! warning {
  (target: $target:expr, $($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::warn!($target, $($arg)+); }
  };
  ($($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::warn!($($arg)+); }
  };
}

macro_rules! info {
  (target: $target:expr, $($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::info!($target, $($arg)+); }
  };
  ($($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::info!($($arg)+); }
  };
}

macro_rules! debug {
  (target: $target:expr, $($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::debug!($target, $($arg)+); }
  };
  ($($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::debug!($($arg)+); }
  };
}

macro_rules! trace {
  (target: $target:expr, $($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::trace!($target, $($arg)+); }
  };
  ($($arg:tt)+) => {
    #[cfg(feature = "logging")]
    { ::log::trace!($($arg)+); }
  };
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use info;
pub(crate) use trace;
pub(crate) use warning;
