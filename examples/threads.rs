use hackerlog::*;
use std::thread;

fn function_log() {
    log_warn!("This is a warning message from a function", verbose);
}

fn function_thread_log() {
    log_warn!("This is a warning message from a function", verbose);
}

fn main() {
    log_info!(
        "This is an info message from main on the main thread",
        verbose
    );
    thread::spawn(|| {
        log_debug!("This is a debug message from another thread", verbose);
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
