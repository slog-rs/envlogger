extern crate slog_stdlog;
extern crate slog_envlogger;

#[macro_use]
extern crate log;

fn main() {
    let _guard = slog_envlogger::init().unwrap();

    error!("error");
    info!("info");
    trace!("trace");
}
