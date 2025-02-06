use std::time::Instant;

#[allow(unused_imports)]
use crate::log;
use crate::LogLevel;

#[cfg(feature = "structured")]
use crate::event;

pub struct TimedOperation {
    start: Instant,
    name: String,
    level: LogLevel,
}

impl TimedOperation {
    pub fn new(name: impl Into<String>, level: LogLevel) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
            level,
        }
    }
}

impl Drop for TimedOperation {
    fn drop(&mut self) {
        let duration = self.start.elapsed();

        #[cfg(feature = "structured")]
        {
            event!(
                self.level,
                "Operation timing",
                {
                    "operation" => self.name.clone(),
                    "duration_ms" => duration.as_millis(),
                    "duration_human" => format!("{:.2?}", duration)
                }
            );
        }

        #[cfg(not(feature = "structured"))]
        {
            log!(
                self.level,
                "Operation '{}' completed in {:.2?}",
                self.name,
                duration
            );
        }
    }
}
