use hackerlog::*;

fn main() {
    // Set minimum log level to WARN - this will hide DEBUG and INFO messages
    logger().min_level(LogLevel::WARN);

    debug!("This debug message won't show");
    info!("This info message won't show");
    warn!("This warning will show!");
    error!("This error will show!");

    // Change minimum level back to DEBUG to show everything
    logger().min_level(LogLevel::DEBUG);

    debug!("Now this debug message shows");
    info!("And this info message too");
}
