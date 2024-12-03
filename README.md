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

## Report an issue

To report an issue, the platform will provides you logs you can share to help us.

```bash
# Run your platform with all logs and traces
cargo run -- -l -b -t
```

Then you can find the path where logs will be stored in your computer "Log dir"

```
----------------------------------------
# Panduza Platform

- Stdout logs         : ENABLED
- Broker logs         : DISABLED
- Debug logs          : DISABLED
- Trace logs          : DISABLED

- Log dir             : "C:\\Users\\Public\\panduza\\logs"
- Tree file           : "C:\\Users\\Public\\panduza\\tree.json"
----------------------------------------
```

Take the log of the day and join it with the issue

