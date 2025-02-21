use std::fmt;

use is_terminal::IsTerminal;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

pub use tracing::Level;
pub use tracing::debug;
pub use tracing::error;
pub use tracing::event;
pub use tracing::info;
pub use tracing::trace;
pub use tracing::warn;

pub(crate) fn init() {
    let builder = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(std::io::stderr().is_terminal())
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(EnvFilter::from_default_env());
    // We cannot write code like this:
    //
    //   let builder = match Format::get() {
    //       Format::Text => builder,
    //       Format::Json => builder.json(),
    //   };
    //
    // The following code also causes build errors:
    //
    //   builder
    //       .with_timer(match Timestamp::get() {
    //           Timestamp::Uptime => builder.with_timer(...),
    //           ...
    //       })
    //
    match (Format::get(), Timestamp::get()) {
        (Format::Text, Timestamp::Uptime) => builder.with_timer(Uptime::new()).init(),
        (Format::Text, Timestamp::Local) => builder.with_timer(LocalTime).init(),
        (Format::Text, Timestamp::Off) => builder.without_time().init(),
        (Format::Json, Timestamp::Uptime) => builder.with_timer(Uptime::new()).json().init(),
        (Format::Json, Timestamp::Local) => builder.with_timer(LocalTime).json().init(),
        (Format::Json, Timestamp::Off) => builder.without_time().json().init(),
    }
}

enum Format {
    Text,
    Json,
}

impl Format {
    fn get() -> Self {
        match std::env::var("BEE_LOG_FORMAT") {
            Ok(v) if v == "json" => Self::Json,
            _ => Self::Text,
        }
    }
}

enum Timestamp {
    Uptime,
    Local,
    Off,
}

impl Timestamp {
    fn get() -> Self {
        match std::env::var("BEE_LOG_TIMESTAMP") {
            Ok(v) if v == "off" => Self::Off,
            Ok(v) if v == "local" => Self::Local,
            _ => Self::Uptime,
        }
    }
}

struct Uptime {
    epoch: std::time::Instant,
}

impl Uptime {
    fn new() -> Self {
        Self {
            epoch: std::time::Instant::now(),
        }
    }
}

impl FormatTime for Uptime {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        let elapsed = self.epoch.elapsed();
        write!(w, "{}.{:06}", elapsed.as_secs(), elapsed.subsec_micros())
    }
}

struct LocalTime;

impl FormatTime for LocalTime {
    fn format_time(&self, w: &mut Writer) -> fmt::Result {
        let time = chrono::Local::now();
        write!(
            w,
            "{}",
            time.to_rfc3339_opts(chrono::SecondsFormat::Micros, false)
        )
    }
}
