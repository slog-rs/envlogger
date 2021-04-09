use log::*;
use slog::Drain;
/// Import longer-name versions of macros only to not collide with legacy `log`
use slog::{slog_error, slog_info, slog_o, slog_trace};

fn main() {
    let drain = slog_async::Async::default(slog_envlogger::new(
        slog_term::CompactFormat::new(slog_term::TermDecorator::new().stderr().build())
            .build()
            .fuse(),
    ));

    let root_logger = slog::Logger::root(
        drain.fuse(),
        slog_o!("build" => "8jdkj2df", "version" => "0.1.5"),
    );

    slog_stdlog::init().unwrap();

    slog_scope::scope(&root_logger, || {
        slog_error!(root_logger, "slog error");
        error!("log error");
        slog_info!(root_logger, "slog info");
        info!("log info");
        slog_trace!(root_logger, "slog trace");
        trace!("log trace");
    });
}
