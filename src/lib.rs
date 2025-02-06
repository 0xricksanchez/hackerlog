mod format;
mod levels;
mod macros;
mod timing;

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

#[cfg(feature = "structured")]
mod structured;

pub use format::{FormatPlaceholder, FormatTemplate};
pub use levels::LogLevel;
pub use timing::TimedOperation;

#[cfg(feature = "structured")]
pub use structured::structured::LogEvent;

// Global logger configuration
pub struct Logger {
    verbose: AtomicBool,
    min_level: AtomicU8,
    writer: Mutex<Box<dyn Write + Send>>,
    context: Mutex<Vec<(String, String)>>,
    format: Mutex<FormatTemplate>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            verbose: AtomicBool::new(false),
            min_level: AtomicU8::new(LogLevel::INFO as u8),
            writer: Mutex::new(Box::new(io::stdout())),
            context: Mutex::new(Vec::new()),
            format: Mutex::new(FormatTemplate::parse("{symbol} {context}{message}")),
        }
    }
}

// Global logger instance
static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn logger() -> &'static Logger {
    LOGGER.get_or_init(Logger::default)
}

impl Logger {
    pub fn set_format(&self, template: &str) -> &Self {
        *self.format.lock().unwrap() = FormatTemplate::parse(template);
        self
    }

    // Default formats for different styles
    pub fn use_simple_format(&self) -> &Self {
        self.set_format("{symbol} {message}")
    }

    pub fn use_detailed_format(&self) -> &Self {
        self.set_format("[{level}] {datetime} {message}")
    }

    pub fn use_debug_format(&self) -> &Self {
        self.set_format("{datetime} [{level}] <{file}:{line}> {message}")
    }

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
        let format = self.format.lock().unwrap();
        let mut output = String::new();

        for part in &format.parts {
            match part {
                FormatPlaceholder::Level => {
                    output.push_str(&format!("{:?}", level));
                }
                FormatPlaceholder::Symbol => {
                    output.push_str(level.symbol());
                }
                FormatPlaceholder::Message => {
                    output.push_str(message);
                }
                FormatPlaceholder::Time => {
                    output.push_str(&Local::now().format("%H:%M:%S").to_string());
                }
                FormatPlaceholder::Date => {
                    output.push_str(&Local::now().format("%Y-%m-%d").to_string());
                }
                FormatPlaceholder::DateTime => {
                    output.push_str(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                }
                FormatPlaceholder::ThreadName => {
                    let name = thread::current().name().map_or_else(
                        || format!("Thread-{:?}", thread::current().id()),
                        ToString::to_string,
                    );
                    output.push_str(&name);
                }
                FormatPlaceholder::ThreadId => {
                    output.push_str(&format!("{:?}", thread::current().id()));
                }
                FormatPlaceholder::ProcessId => {
                    output.push_str(&process::id().to_string());
                }
                FormatPlaceholder::File => {
                    output.push_str(file);
                }
                FormatPlaceholder::Line => {
                    output.push_str(&line.to_string());
                }
                FormatPlaceholder::Context => {
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
                }
                FormatPlaceholder::Text(text) => {
                    output.push_str(text);
                }
            }
        }

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

    #[cfg(feature = "structured")]
    pub fn structured_format(&self) -> &Self {
        self.set_format("{datetime} {level} {message} {fields}")
    }

    #[cfg(feature = "structured")]
    pub fn write_structured_event(&self, event: &LogEvent) -> io::Result<()> {
        let mut fields_str = String::new();
        if !event.fields.is_empty() {
            fields_str.push('[');
            for (i, (key, value)) in event.fields.iter().enumerate() {
                if i > 0 {
                    fields_str.push_str(", ");
                }
                fields_str.push_str(&format!("{}={}", key, value));
            }
            fields_str.push(']');
        }

        self.write_log(
            event.level,
            &format!("{} {}", event.message, fields_str),
            &event.file,
            event.line,
        )
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
