use log::*;

fn main() {
    let _guard = slog_envlogger::init().unwrap();

    error!("error");
    info!("info");
    trace!("trace");
}
