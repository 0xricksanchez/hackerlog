# Hackerlog

A super simple but lightweight logging library that tries to capture the most important
(status) information. The following is supported:

- [x] Log level with colors
- [x] Verbose mode:
  - [x] Timestamp
  - [x] PID
  - [x] Thread name
  - [x] Location
- [x] Output redirection to a log
- [x] Custom formatting
- [x] Structured logging
- [x] Log-level filtering
- [x] Custom contexts

## Examples

The core functionality can be seen in the `examples/` folder.
You can run both of them via `cargo run --example <name>`:

### Simple logger

```bash
``cargo run --example simple
   Compiling hackerlog v0.1.4 (/home/krah/git/priv/hackerlog)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/simple`
[>] This is an info message
[>] This is an info message with a variable: 1337
[>] This is an info message with different syntactic sugar: 1337
[!] This is a warning message
[x] This is an error message
[+] This is a success message
[-] This is a failure message
```

### Verbose logger

```bash
cargo run --example verbose
   Compiling hackerlog v0.1.0 (/Users/0x434b/Git/private/hackerlog)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/examples/verbose`
[>] (11:48:52) - [PID: 15435 | Thread: main] - (examples/verbose.rs:4) : This is an info message
[#] (11:48:52) - [PID: 15435 | Thread: main] - (examples/verbose.rs:5) : This is a debug message
[!] (11:48:52) - [PID: 15435 | Thread: main] - (examples/verbose.rs:6) : This is a warning message
[x] (11:48:52) - [PID: 15435 | Thread: main] - (examples/verbose.rs:7) : This is an error message
[+] (11:48:52) - [PID: 15435 | Thread: main] - (examples/verbose.rs:8) : This is a success message
[-] (11:48:52) - [PID: 15435 | Thread: main] - (examples/verbose.rs:9) : This is a failure message
```

### Structured logger

Or for a more complex logging format:

```
 cargo run --example structured --features structured
   Compiling hackerlog v0.1.4 (/home/krah/git/priv/hackerlog)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/structured`
2025-02-06 11:01:15 INFO User logged in [ip="192.168.1.1", success=true, user_id=1234]
2025-02-06 11:01:15 INFO User details [login_count=5, session_id="abc-123", user={"id":1234,"name":"alice","role":"admin"}]
2025-02-06 11:01:15 DEBUG Operation timing [duration_human="100.31ms", duration_ms=100, operation="database_query"]
2025-02-06 11:01:16 INFO Operation timing [duration_human="100.30ms", duration_ms=100, operation="api_request"]
2025-02-06 11:01:16 WARN Operation timing [duration_human="100.07ms", duration_ms=100, operation="critical_operation"]
2025-02-06 11:01:16 INFO Thread finished [status="complete", thread_id=1]
2025-02-06 11:01:16 INFO Operation timing [duration_human="100.46ms", duration_ms=100, operation="thread_operation_1"]
2025-02-06 11:01:16 INFO Thread finished [status="complete", thread_id=2]
2025-02-06 11:01:16 INFO Operation timing [duration_human="100.50ms", duration_ms=100, operation="thread_operation_2"]
2025-02-06 11:01:16 INFO Thread finished [status="complete", thread_id=0]
2025-02-06 11:01:16 INFO Operation timing [duration_human="100.59ms", duration_ms=100, operation="thread_operation_0"]
```

## Usage

Run `cargo add hackerlog` in your project root and just import `hackerlog` in your application as `use hackerlog::*;` and you have access to the macros:

- info!
- debug!
- warn!
- err!
- success!
- fail!
