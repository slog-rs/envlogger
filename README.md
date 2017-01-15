# slog-envlogger - Port of `env_logger` as a `slog-rs` drain

<p align="center">
  <a href="https://travis-ci.org/dpc/slog-envlogger">
      <img src="https://img.shields.io/travis/dpc/slog-envlogger/master.svg?style=flat-square" alt="Travis CI Build Status">
  </a>
  <a href="https://crates.io/crates/slog-envlogger">
      <img src="http://meritbadge.herokuapp.com/slog?style=flat-square" alt="crates.io">
  </a>
  <a href="https://gitter.im/dpc/slog-rs">
      <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg?style=flat-square" alt="Gitter Chat">
  </a>
  <br>
  <strong><a href="//dpc.github.io/slog-envlogger/">Documentation</a></strong>
</p>


`env_logger` is a de facto standard Rust logger implementation, which allows
controlling logging to `stderr` via `RUST_LOG` environment variable.

This is a fork of `env_logger` that makes it work as a `slog-rs` drain:

Notable changes:

* Support for `slog-stdlog` to provide support for legacy `info!(...)` like
  statements.
* `envlogger` does not do formatting anymore: `slog-envlogger` can be composed
  with any other `slog-rs` drains, so there's no point for it to provide it's
  own formatting. You can now output JSON to a file, controlling it via
  `RUST_LOG` environment var. `envlogger::init()` is provided for convenience
  doing formatting to `stderr`

### Status & news

**Warning**: Documentation has been been left mostly untouched, which means some
places of it might be confusing.

### How to use

See `examples` directory.

The simplest way to convert existing project to use `slog-rs`+`slog-envlogger`
is shown in
[`simple` example](https://github.com/slog-rs/envlogger/blob/master/examples/simple.rs)

For more proper (and powerful) version see
[`proper` example](https://github.com/slog-rs/envlogger/blob/master/examples/proper.rs)

Using `slog-stdlog` scopes you can make parts of the code log additional information (see [`scopes` example][scopes]):

[scopes]: https://github.com/dpc/slog-envlogger/blob/master/examples/scopes.rs
