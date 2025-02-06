use hackerlog::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Enable verbose mode to see thread information
    logger().verbose(true);

    // Spawn multiple threads that log simultaneously
    let mut handles = vec![];

    for i in 0..3 {
        let handle = thread::spawn(move || {
            let _ctx = logger().add_context("thread_id", i.to_string());

            for j in 0..3 {
                info!("Message {} from thread {}", j, i);
                thread::sleep(Duration::from_millis(100));
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
