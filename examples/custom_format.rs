use hackerlog::*;
use std::thread;

fn main() {
    // Try different built-in formats
    logger().use_simple_format();
    info!("Simple format");
    warn!("Just symbol and message");

    // Detailed format with timestamp
    logger().use_detailed_format();
    info!("Detailed format");
    error!("With timestamp");

    // Debug format with file location
    logger().use_debug_format();
    info!("Debug format");
    warn!("With file location");

    // Custom format
    logger().set_format("{datetime} | {pid}:{thread_id} | {level} | {message}");
    info!("Custom format");

    // Format with context
    let _ctx = logger().add_context("user", "admin");
    logger().set_format("[{level}] {context}{message} ({file}:{line})");
    info!("Message with context");

    // Complex format showing all options
    logger().set_format(concat!(
        "Time: {time} | ",
        "Date: {date} | ",
        "Thread: {thread} | ",
        "PID: {pid} | ",
        "Level: {level} ({symbol}) | ",
        "{context}",
        "Message: {message}"
    ));

    thread::spawn(|| {
        info!("Shows all available placeholders");
    })
    .join()
    .unwrap();
}
