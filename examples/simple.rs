extern crate slog_envlogger;
extern crate slog_stdlog;

#[macro_use]
extern crate log;

fn main() {
    let _guard = slog_envlogger::init().unwrap();

    error!("error");
    info!("info");
    trace!("trace");
}
