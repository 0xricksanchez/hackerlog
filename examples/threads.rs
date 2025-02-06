use hackerlog::*;
use std::thread;

fn function_log() {
    log_warn!(verbose => "This is a warning message from a function");
}

fn function_thread_log() {
    log_warn!(verbose => "This is a warning message from a function");
}

fn main() {
    log_info!(
        verbose => "This is an info message from main on the main thread"
    );
    thread::spawn(|| {
        log_dbg!(verbose => "This is a debug message from another thread");
    })
    .join()
    .unwrap();
    function_log();
    thread::spawn(|| {
        function_thread_log();
    })
    .join()
    .unwrap();
}
