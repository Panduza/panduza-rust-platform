# panduza-rust-platform


## Usage

You must install Rust and Cargo then execute this commands

```bash
# Enable fmt tracing (std terminal logs)
cargo run

# Log options
cargo run -- -h
# -l to log on terminal
# -b to enable broker logs on terminal
# -d to enable debug logs (terminal + file)
# -t to enable trace logs (terminal + file)
```

To embbed built-in drivers

```bash
# Enable fmt tracing (std terminal logs)
cargo run --features built-in-drivers
cargo build --features built-in-drivers
```
