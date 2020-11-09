// Usage:
//   error!("Oh no");
//   warn!("Log entry with metadata", "x": 1, "y": true);

// TODO: Current implementation works nicely for development but it should be redesigned a bit
//       to be able to route log data to different destinations (like stdout, bug trackers, etc.)
//       depending on... crate features, I guess?

pub fn init() {
    env_logger::init();
}

#[allow(unused_macros)]
macro_rules! meta {
  ($($label:literal: $value:expr),+) => {{
    let mut meta = String::new();
    $(
      meta.push_str(&format!("{}: {:#?}", $label, $value).trim());
      meta.push_str("\n");
    )*
    meta.trim().to_owned()
  }}
}

#[macro_export]
macro_rules! trace {
  ($msg:expr) => { log::trace!("{}", $msg) };
  ($msg:expr, $($label:literal: $value:expr),+) => {
    log::trace!("{msg}\n{meta}", msg = $msg, meta = meta!($($label: $value),+));
  };
}

#[macro_export]
macro_rules! debug {
  ($msg:expr) => { log::debug!("{}", $msg) };
  ($msg:expr, $($label:literal: $value:expr),+) => {
    log::debug!("{msg}\n{meta}", msg = $msg, meta = meta!($($label: $value),+));
  };
}

#[macro_export]
macro_rules! info {
  ($msg:expr) => { log::info!("{}", $msg) };
  ($msg:expr, $($label:literal: $value:expr),+) => {
    log::info!("{msg}\n{meta}", msg = $msg, meta = meta!($($label: $value),+));
  };
}

#[macro_export]
macro_rules! warn {
  ($msg:expr) => { log::warn!("{}", $msg) };
  ($msg:expr, $($label:literal: $value:expr),+) => {
    log::warn!("{msg}\n{meta}", msg = $msg, meta = meta!($($label: $value),+));
  };
}

#[macro_export]
macro_rules! error {
  ($msg:expr) => { log::error!("{}", $msg) };
  ($msg:expr, $($label:literal: $value:expr),+) => {
    log::error!("{msg}\n{meta}", msg = $msg, meta = meta!($($label: $value),+));
  };
}
