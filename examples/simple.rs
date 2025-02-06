use hackerlog::*;

fn main() {
    log_info!("This is an info message");
    let a = 1337;
    log_info!("This is an info message with a variable: {}", a);
    log!(
        LogLevel::INFO,
        "This is an info message with different syntactic sugar: {}",
        a
    );
    log_dbg!("This is a debug message");
    log_warn!("This is a warning message");
    log_err!("This is an error message");
    log_success!("This is a success message");
    log_fail!("This is a failure message");
}
