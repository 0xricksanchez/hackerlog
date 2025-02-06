use hackerlog::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    // First log to stdout
    info!("This goes to terminal");

    // Redirect to a file
    let log_file = File::create("test.log")?;
    logger().set_writer(Box::new(log_file))?;

    info!("This goes to test.log");
    warn!("This warning also goes to test.log");

    Ok(())
}
