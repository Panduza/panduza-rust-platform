# panduza-rust-platform


## Usage

```bash
# Enable fmt tracing
cargo run --features trac-fmt

# Enable tokio console tracing
RUSTFLAGS="--cfg tokio_unstable" cargo run --features trac-console
```
