extern crate slog_stdlog;
extern crate slog_envlogger;

#[macro_use(o)]
extern crate slog;

#[macro_use]
extern crate log;

fn main() {
    slog_envlogger::init().unwrap();

    error!("log error");

    slog_stdlog::scope(
        slog_stdlog::with_current_logger(|l| l.new(o!("scope-extra-data" => "data"))),
        || foo()
    );

    trace!("log trace");
}

fn foo() {
    info!("log info inside foo");

    // scopes can be nested!
    slog_stdlog::scope(
        slog_stdlog::with_current_logger(|l| l.new(o!("even-more-scope-extra-data" => "data2"))),
        || bar()
    );
}

fn bar() {
    info!("log info inside bar");
}
