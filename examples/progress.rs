use hackerlog::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Basic progress without total
    let mut progress = Progress::new("Processing");
    for _ in 0..5 {
        thread::sleep(Duration::from_millis(300));
        progress.inc(1);
    }
    progress.finish();

    // Progress with total
    let mut progress = Progress::with_total("Downloading", 100);
    for i in 0..=10 {
        thread::sleep(Duration::from_millis(100));
        progress.inc(10);
        progress.update(format!("Downloading chunk {}/10", i));
    }
    progress.finish_with_message("Download complete!");

    // Multiple progress indicators
    let mut scan = Progress::with_total("Port scan", 1000);
    let mut analysis = Progress::new("Analyzing results");

    for i in (0..=1000).step_by(100) {
        thread::sleep(Duration::from_millis(50));
        scan.inc(100);
        if i % 200 == 0 {
            analysis.inc(1);
        }
    }

    scan.finish();
    analysis.finish_with_message("Analysis complete - Found 5 open ports");
}
