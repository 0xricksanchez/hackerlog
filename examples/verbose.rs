use hackerlog::*;

fn main() {
    log_info!(verbose => "This is an info message");
    let a = 1337;
    log_info!(verbose => "This is an info message with a variable: {}", a);
    log!(LogLevel::INFO, verbose => "This is an info message with different syntactic sugar {}", a);
    log_dbg!(verbose => "This is a debug message");
    log_warn!(verbose => "This is a warning message");
    log_err!(verbose => "This is an error message");
    log_success!(verbose => "This is a success message");
    log_fail!(verbose => "This is a failure message" );
}
