extern crate log;

use slog::Level;

/// A slog powered log backend.
///
/// Basically the same as `slog-stdlog` but use `slog-global::borrow_global()`.
struct SlogBackend;

fn log_to_slog_level(level: log::LogLevel) -> Level {
    match level {
        log::LogLevel::Trace => Level::Trace,
        log::LogLevel::Debug => Level::Debug,
        log::LogLevel::Info => Level::Info,
        log::LogLevel::Warn => Level::Warning,
        log::LogLevel::Error => Level::Error,
    }
}

impl log::Log for SlogBackend {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level = log_to_slog_level(r.metadata().level());

        let args = r.args();
        let target = r.target();
        let module = r.location().__module_path;
        let file = r.location().__file;
        let line = r.location().line();

        let s = slog::RecordStatic {
            location: &slog::RecordLocation {
                file,
                line,
                column: 0,
                function: "",
                module,
            },
            level,
            tag: target,
        };
        ::borrow_global().log(&slog::Record::new(&s, args, b!()))
    }
}

/// Starts redirecting all logs from the `log` crate `slog_global::borrow_global()`.
///
/// Logs will be always outputted to the active global logger at the time of logging
/// (instead of the global logger when this function is called).
pub fn redirect_std_log(level: Option<log::LogLevel>) -> Result<(), log::SetLoggerError> {
    log::set_logger(move |max_log_level| {
        if let Some(level) = level {
            max_log_level.set(level.to_log_level_filter());
        }
        Box::new(SlogBackend)
    })
}
