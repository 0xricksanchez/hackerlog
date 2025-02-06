use chrono::Local;
use std::{
    fmt,
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Mutex, OnceLock,
    },
};
use std::{process, thread};
use termion::color;

// Core types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    DEBUG = 0,
    INFO = 1,
    WARN = 2,
    ERROR = 3,
    SUCCESS = 4,
    FAILURE = 5,
}

impl LogLevel {
    const fn symbol(&self) -> &str {
        match self {
            Self::INFO => "[>]",
            Self::DEBUG => "[#]",
            Self::WARN => "[!]",
            Self::ERROR => "[x]",
            Self::SUCCESS => "[+]",
            Self::FAILURE => "[-]",
        }
    }

    fn color(&self) -> impl color::Color {
        match *self {
            Self::INFO => color::Rgb(255, 255, 255),  // White
            Self::DEBUG => color::Rgb(100, 100, 255), // Light Blue
            Self::WARN => color::Rgb(255, 165, 0),    // Orange
            Self::ERROR => color::Rgb(255, 0, 0),     // Red
            Self::SUCCESS => color::Rgb(0, 255, 0),   // Green
            Self::FAILURE => color::Rgb(139, 0, 0),   // Dark Red
        }
    }
}

// Global logger configuration
pub struct Logger {
    verbose: AtomicBool,
    min_level: AtomicU8,
    writer: Mutex<Box<dyn Write + Send>>,
    context: Mutex<Vec<(String, String)>>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            verbose: AtomicBool::new(false),
            min_level: AtomicU8::new(LogLevel::INFO as u8),
            writer: Mutex::new(Box::new(io::stdout())),
            context: Mutex::new(Vec::new()),
        }
    }
}

// Global logger instance
static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn logger() -> &'static Logger {
    LOGGER.get_or_init(Logger::default)
}

impl Logger {
    pub fn verbose(&self, enabled: bool) -> &Self {
        self.verbose.store(enabled, Ordering::Relaxed);
        self
    }

    pub fn min_level(&self, level: LogLevel) -> &Self {
        self.min_level.store(level as u8, Ordering::Relaxed);
        self
    }

    pub fn set_writer(&self, writer: Box<dyn Write + Send>) -> io::Result<()> {
        *self.writer.lock().unwrap() = writer;
        Ok(())
    }

    pub fn add_context<K, V>(&self, key: K, value: V) -> ContextGuard
    where
        K: Into<String>,
        V: Into<String>,
    {
        let mut context = self.context.lock().unwrap();
        context.push((key.into(), value.into()));
        ContextGuard(context.len() - 1)
    }

    pub fn should_log(&self, level: LogLevel) -> bool {
        let min_level = self.min_level.load(Ordering::Relaxed);
        level as u8 >= min_level
    }

    pub fn write_log(
        &self,
        level: LogLevel,
        message: &str,
        file: &str,
        line: u32,
    ) -> io::Result<()> {
        let mut output = String::new();

        // Add symbol
        output.push_str(&format!("{} ", level.symbol()));

        // Add verbose info if enabled
        if self.verbose.load(Ordering::Relaxed) {
            let time = Local::now().format("%H:%M:%S").to_string();
            let thread_name = thread::current().name().map_or_else(
                || format!("Thread-{:?}", thread::current().id()),
                ToString::to_string,
            );
            let pid = process::id();

            output.push_str(&format!(
                "({time}) - [PID: {pid} | Thread: {thread_name}] - ({file}:{line}) : "
            ));
        }

        // Add context if any
        let context = self.context.lock().unwrap();
        if !context.is_empty() {
            output.push('[');
            for (i, (key, value)) in context.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}={}", key, value));
            }
            output.push_str("] ");
        }

        // Add message
        output.push_str(message);
        output.push('\n');

        // Color the output
        let colored_output = format!(
            "{}{}{}",
            color::Fg(level.color()),
            output,
            color::Fg(color::Reset)
        );

        // Write to configured output
        let mut writer = self.writer.lock().unwrap();
        writer.write_all(colored_output.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}

// Context guard for automatic cleanup
pub struct ContextGuard(usize);

impl Drop for ContextGuard {
    fn drop(&mut self) {
        if let Some(logger) = LOGGER.get() {
            let mut context = logger.context.lock().unwrap();
            if self.0 < context.len() {
                context.remove(self.0);
            }
        }
    }
}

// Error handling
#[derive(Debug)]
pub struct LogError {
    message: String,
}

impl LogError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LogError {}

impl From<io::Error> for LogError {
    fn from(err: io::Error) -> Self {
        LogError::new(err.to_string())
    }
}

// Progress indicator
pub struct Progress {
    message: String,
    total: Option<u64>,
    current: u64,
}

impl Progress {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            total: None,
            current: 0,
        }
    }

    pub fn with_total<S: Into<String>>(message: S, total: u64) -> Self {
        Self {
            message: message.into(),
            total: Some(total),
            current: 0,
        }
    }

    pub fn inc(&mut self, amount: u64) {
        self.current += amount;
        let message = self.message.clone();
        self.update(&message);
    }

    pub fn update<S: Into<String>>(&mut self, message: S) {
        self.message = message.into();
        if let Some(total) = self.total {
            logger()
                .write_log(
                    LogLevel::INFO,
                    &format!("{} [{}/{}]", self.message, self.current, total),
                    file!(),
                    line!(),
                )
                .ok();
        } else {
            logger()
                .write_log(
                    LogLevel::INFO,
                    &format!("{} [{}]", self.message, self.current),
                    file!(),
                    line!(),
                )
                .ok();
        }
    }

    pub fn finish(self) {
        logger()
            .write_log(
                LogLevel::SUCCESS,
                &format!("{} [Complete]", self.message),
                file!(),
                line!(),
            )
            .ok();
    }

    pub fn finish_with_message<S: Into<String>>(self, message: S) {
        logger()
            .write_log(LogLevel::SUCCESS, &message.into(), file!(), line!())
            .ok();
    }
}

// Macros
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        if $crate::logger().should_log($level) {
            let message = format!($($arg)*);
            $crate::logger().write_log($level, &message, file!(), line!()).ok();
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::DEBUG, $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::INFO, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::WARN, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::ERROR, $($arg)*);
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::SUCCESS, $($arg)*);
    };
}

#[macro_export]
macro_rules! failure {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::FAILURE, $($arg)*);
    };
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_log_levels() {
        assert!(LogLevel::ERROR > LogLevel::INFO);
        assert!(LogLevel::DEBUG < LogLevel::WARN);
    }

    #[test]
    fn test_basic_logging() {
        let buffer = Cursor::new(Vec::new());
        logger().set_writer(Box::new(buffer.clone())).unwrap();

        info!("Test message");

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(output.contains("Test message"));
    }

    #[test]
    fn test_context() {
        let buffer = Cursor::new(Vec::new());
        logger().set_writer(Box::new(buffer.clone())).unwrap();

        let _guard = logger().add_context("test", "value");
        info!("Test with context");

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(output.contains("test=value"));
    }

    #[test]
    fn test_progress() {
        let buffer = Cursor::new(Vec::new());
        logger().set_writer(Box::new(buffer.clone())).unwrap();

        let mut progress = Progress::with_total("Testing", 100);
        progress.inc(50);
        progress.finish();

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(output.contains("[50/100]"));
        assert!(output.contains("[Complete]"));
    }

    #[test]
    fn test_verbose_mode() {
        let buffer = Cursor::new(Vec::new());
        logger().set_writer(Box::new(buffer.clone())).unwrap();
        logger().verbose(true);

        info!("Verbose test");

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(output.contains("PID:"));
        assert!(output.contains("Thread:"));
    }
}
