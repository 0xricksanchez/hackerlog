use hackerlog::*;

fn main() {
    info!("This is an info message");
    let a = 1337;
    info!("This is an info message with a variable: {}", a);
    log!(
        LogLevel::INFO,
        "This is an info message with different syntactic sugar: {}",
        a
    );
    debug!("This is a debug message");
    warn!("This is a warning message");
    error!("This is an error message");
    success!("This is a success message");
    failure!("This is a failure message");
}