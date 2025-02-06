use hackerlog::*;

fn scan_ports(host: &str) {
    // Add context that will be included in all logs in this scope
    let _host_ctx = logger().add_context("host", host);

    info!("Starting port scan");

    // Add another context
    let _scan_ctx = logger().add_context("scan_type", "TCP");
    warn!("Found open port 80"); // Will include both host and scan_type

    // scan_ctx is dropped here, removing "scan_type" from context
}

fn main() {
    let _req_id = logger().add_context("request_id", "12345");

    info!("Starting application"); // Includes request_id

    scan_ports("example.com"); // Includes request_id and temporarily host

    info!("Finished!"); // Only includes request_id again
}
