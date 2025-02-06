#[cfg(feature = "structured")]
use hackerlog::*;
use std::{thread, time::Duration};

#[cfg(feature = "structured")]
#[derive(serde::Serialize)]
struct User {
    id: u64,
    name: String,
    role: String,
}

#[cfg(feature = "structured")]
fn simulate_operation() {
    thread::sleep(Duration::from_millis(100));
}

#[cfg(feature = "structured")]
fn main() {
    // Enable structured format
    logger().structured_format();

    // Basic structured logging
    info_event!(
        "User logged in",
        {
            "user_id" => 1234,
            "ip" => "192.168.1.1",
            "success" => true
        }
    );

    // Structured logging with complex types
    let user = User {
        id: 1234,
        name: "alice".to_string(),
        role: "admin".to_string(),
    };

    info_event!(
        "User details",
        {
            "user" => &user,
            "session_id" => "abc-123",
            "login_count" => 5
        }
    );

    // Automatic timing of operations
    {
        let _timer = TimedOperation::new("database_query", LogLevel::DEBUG);
        simulate_operation();
    }

    // Timing macro
    {
        time!("api_request");
        simulate_operation();
    }

    // Timing with custom level
    {
        time!("critical_operation", LogLevel::WARN);
        simulate_operation();
    }

    // Multiple concurrent operations
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                time!(&format!("thread_operation_{}", i));
                simulate_operation();

                info_event!(
                    "Thread finished",
                    {
                        "thread_id" => i,
                        "status" => "complete"
                    }
                );
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(not(feature = "structured"))]
fn main() {
    println!("This example requires the 'structured' feature. Run with:");
    println!("    cargo run --example structured --features structured");
}
