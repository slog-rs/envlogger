<p align="center">

  <a href="https://github.com/slog-rs/slog">
  <img src="https://cdn.rawgit.com/slog-rs/misc/master/media/slog.svg" alt="slog-rs logo">
  </a>
  <br>

  <a href="https://travis-ci.org/slog-rs/envlogger">
      <img src="https://img.shields.io/travis/slog-rs/envlogger/master.svg?style=flat-square" alt="Travis CI Build Status">
  </a>
  <a href="https://crates.io/crates/slog-envlogger">
      <img src="http://meritbadge.herokuapp.com/slog-envlogger?style=flat-square" alt="crates.io">
  </a>
  <a href="https://gitter.im/slog-rs/slog">
      <img src="https://img.shields.io/gitter/room/slog-rs/slog.svg" alt="slog-rs Gitter Chat">
  </a>
  <br>
  <strong><a href="https://docs.rs/slog-envlogger/">Documentation</a></strong>
</p>

# `slog-envlogger` - Port of `env_logger` as a [`slog-rs`][slog-rs] drain

`env_logger` is a de facto standard Rust logger implementation, which allows
controlling logging to `stderr` via the `RUST_LOG` environment variable.

This is a fork of `env_logger` that makes it work as a `slog-rs` drain.

Notable changes:

* Support for `slog-stdlog` to provide support for legacy `info!(...)` like
  statements.
* `envlogger` does not do any formatting anymore: `slog-envlogger` can be composed
  with any other `slog-rs` drains, so there's no point for it to provide it's
  own formatting. You can now output to a file, use JSON, have color output
  or any other future that `slog` ecosystem provides, controlling it via
  `RUST_LOG` environment var.

### Status & news

**Warning**: Documentation has been been left mostly untouched, which means some
places of it might be confusing.

### How to use

See `examples` directory.

The simplest way to convert existing project to use `slog-rs`+`slog-envlogger`
is shown in
[`simple` example](examples/simple.rs)

For more proper (and powerful) version see
[`proper` example](examples/proper.rs)

Using `slog-stdlog` scopes you can make parts of the code log additional information (see [`scopes` example][scopes]):

[scopes]: examples/scopes.rs
[slog-rs]: //github.com/slog-rs/slog
