use hackerlog::*;
use std::thread;

fn function_log() {
    warn!("This is a warning message from a function");
}

fn function_thread_log() {
    warn!("This is a warning message from a function");
}

fn main() {
    // Enable verbose mode globally
    logger().verbose(true);

    info!("This is an info message from main on the main thread");

    thread::spawn(|| {
        debug!("This is a debug message from another thread");
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
