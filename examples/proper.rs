extern crate slog_stdlog;
extern crate slog_envlogger;
extern crate slog_term;

/// Import longer-name versions of macros only to not collide with legacy `log`
#[macro_use(slog_error, slog_info, slog_trace, slog_log, o)]
extern crate slog;

use slog::DrainExt;

#[macro_use]
extern crate log;

fn main() {
    let term = slog_term::streamer().build();
    let drain = slog_envlogger::new(term);

    let root_logger = slog::Logger::root(drain.fuse(), o!("build" => "8jdkj2df", "version" => "0.1.5"));

    slog_stdlog::set_logger(root_logger.clone()).unwrap();

    slog_error!(root_logger, "slog error");
    error!("log error");
    slog_info!(root_logger, "slog info");
    info!("log info");
    slog_trace!(root_logger, "slog trace");
    trace!("log trace");
}
