#[macro_use]
mod macros;

// NOTE: Crates using this crate will rebuild when this file changes even if only code inside the
// `tests` module changes.  For avoiding the meaningless rebuilds, separate the `tests` module to
// tests.rs.
#[cfg(test)]
mod tests;

pub mod imp;

use std::str::FromStr;

/// Initializes the log system.
///
/// This function must be called only once in a process before calling any functions that use
/// logging macros.
///
/// This function is **NOT** thread-safe.  So, it's strongly recommended to call this function
/// before creating any threads other than the main thread.
pub fn init() {
    imp::init();
}

/// Loads the logging level of a target from the `RUST_LOG` environment variable.
pub fn load_level(target: &str) -> imp::LevelFilter {
    load_level_from_env(target, "RUST_LOG")
}

/// Loads the logging level for a target from an environment variable.
pub fn load_level_from_env(target: &str, name: &str) -> imp::LevelFilter {
    let filters = std::env::var(name).unwrap_or_default();
    load_level_from_str(target, &filters)
}

/// Loads the logging level for a target from a string.
pub fn load_level_from_str(target: &str, filters: &str) -> imp::LevelFilter {
    for filter in filters.split(',').rev() {
        let filter = filter.trim();
        if filter.is_empty() {
            continue;
        }
        let (name, level) = filter
            .split_once('=')
            .map(|(name, level)| {
                let name = name.trim();
                let level = imp::LevelFilter::from_str(level.trim()).unwrap();
                (name, level)
            })
            .unwrap_or_else(|| match imp::LevelFilter::from_str(filter) {
                Ok(level) => ("", level),
                Err(_) => (filter, imp::LevelFilter::TRACE),
            });
        if target.starts_with(name) {
            return level;
        }
    }
    imp::LevelFilter::OFF
}
