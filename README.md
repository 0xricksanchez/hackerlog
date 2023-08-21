# Hackerlog

A super simple but lightweight logging library that tries to capture the most important
(status) information. The following is supported:

- [x] Log level with colors
- [x] verbose mode:
  - [x] Timestamp
  - [x] PID
  - [x] Thread name
  - [x] Location

## Examples

The core functionality can be seen in the `examples/` folder.
You can run both of them via:

```bash
cargo run --example simply # or
cargo run --example verbose
```

```bash
cargo run --example verbose
   Compiling hackerlog v0.1.0 (/Users/0x434b/Git/private/hackerlog)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/examples/verbose`
[>] (10:36:40) - [PID: 224 | Thread: main] - (src/lib.rs:44) : This is an info message
[#] (10:36:40) - [PID: 224 | Thread: main] - (src/lib.rs:44) : This is a debug message
[!] (10:36:40) - [PID: 224 | Thread: main] - (src/lib.rs:44) : This is a warning message
[x] (10:36:40) - [PID: 224 | Thread: main] - (src/lib.rs:44) : This is an error message
[+] (10:36:40) - [PID: 224 | Thread: main] - (src/lib.rs:44) : This is a success message
[-] (10:36:40) - [PID: 224 | Thread: main] - (src/lib.rs:44) : This is a failure message
```

## Usage

Just import `hackerlog` in your application as `use hackerlog::*;` and you have access to the macros:

- log_info!
- log_debug!
- log_warn!
- log_err!
- log_success!
- log_fail!
