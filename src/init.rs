extern crate log;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

use crate::new;
use slog::*;
use std::sync;

/// Use a default `EnvLogger` as global logging drain
///
/// This is for lazy devs that with minimal amount of work want to convert
/// software that used standard Rust `env_logger` crate to
/// `slog-env_logger`.
///
/// It's an easy first step, but using `init()` you're not gaining almost
/// anything that `slog` has to offer, so I highly encourage to use `new()`
/// instead and explicitly configure your loggers.
pub fn init() -> std::result::Result<slog_scope::GlobalLoggerGuard, log::SetLoggerError> {
    let drain =
        slog_term::CompactFormat::new(slog_term::TermDecorator::new().stderr().build()).build();
    let drain = new(drain);
    let drain = sync::Mutex::new(drain.fuse());

    let guard = slog_scope::set_global_logger(Logger::root(drain.fuse(), o!()).into_erased());
    slog_stdlog::init()?;

    Ok(guard)
}
