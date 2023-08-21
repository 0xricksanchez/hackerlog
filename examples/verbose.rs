use hackerlog::*;

fn main() {
    log_info!("This is an info message", verbose);
    log_debug!("This is a debug message", verbose);
    log_warn!("This is a warning message", verbose);
    log_err!("This is an error message", verbose);
    log_success!("This is a success message", verbose);
    log_fail!("This is a failure message", verbose);
}
