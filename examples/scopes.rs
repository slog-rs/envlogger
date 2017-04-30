extern crate slog_stdlog;
extern crate slog_envlogger;
extern crate slog_scope;

#[macro_use(o, kv)]
extern crate slog;

#[macro_use]
extern crate log;

fn main() {
    let _guard = slog_envlogger::init().unwrap();

    error!("log error");

    slog_scope::scope(
        &slog_scope::logger().new(o!("scope-extra-data" => "data")),
        || foo()
    );

    trace!("log trace");
}

fn foo() {
    info!("log info inside foo");

    // scopes can be nested!
    slog_scope::scope(
        &slog_scope::logger().new(o!("even-more-scope-extra-data" => "data2")),
        || bar()
    );
}

fn bar() {
    info!("log info inside bar");
}
