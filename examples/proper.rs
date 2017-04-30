extern crate slog_stdlog;
extern crate slog_envlogger;
extern crate slog_term;
extern crate slog_scope;
extern crate slog_async;

/// Import longer-name versions of macros only to not collide with legacy `log`
#[macro_use(slog_error, slog_info, slog_trace, slog_log, slog_o, slog_record,
            slog_record_static, slog_b, slog_kv)]
extern crate slog;

use slog::Drain;

#[macro_use]
extern crate log;

fn main() {
    let drain =
        slog_async::Async::default(
        slog_envlogger::new(
        slog_term::CompactFormat::new(
            slog_term::TermDecorator::new()
            .stderr().build()
            ).build().fuse()
        ));

    let root_logger = slog::Logger::root(drain.fuse(),
                                         slog_o!("build" => "8jdkj2df", "version" => "0.1.5"));

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
