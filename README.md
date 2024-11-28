# panduza-rust-platform


## Usage

You must install Rust and Cargo then execute this commands

```bash
# Enable fmt tracing (std terminal logs)
cargo run --features log
```

To embbed built-in drivers

```bash
# Enable fmt tracing (std terminal logs)
cargo run --features log,built-in-drivers
cargo build --features log,built-in-drivers
```

## Manage log levels

To set the log level at build time, please go to the Cargo.tml

Then edit the 'tacing' entry to set the level you want

```toml
tracing = { version = "0.1", features = [
    # "max_level_off",
    # "max_level_error",
    # "max_level_warn",
    "max_level_info",
    # "max_level_debug",
    # "max_level_trace",
    # "release_max_level_off",
    # "release_max_level_error",
    # "release_max_level_warn",
    "release_max_level_info",
    # "release_max_level_debug",
    # "release_max_level_trace"
]}
```

# Others


```bash
# Enable tokio console tracing
RUSTFLAGS="--cfg tokio_unstable" cargo run --features trac-console
```