use std::{process, thread};
use termion::color;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    INFO,
    DEBUG,
    WARN,
    ERROR,
    SUCCESS,
    FAILURE,
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

    fn color(&self) -> &dyn termion::color::Color {
        match *self {
            Self::INFO => &color::Rgb(255, 255, 255),  // White
            Self::DEBUG => &color::Rgb(100, 100, 255), // Light Blue
            Self::WARN => &color::Rgb(255, 165, 0),    // Orange
            Self::ERROR => &color::Rgb(255, 0, 0),     // Red
            Self::SUCCESS => &color::Rgb(0, 255, 0),   // Green
            Self::FAILURE => &color::Rgb(139, 0, 0),   // Dark Red
        }
    }
}

pub fn log<T: std::fmt::Display>(
    level: &LogLevel,
    verbose: bool,
    message: T,
    called_in: &str,
    called_from: u32,
) {
    let mut output = format!("{} ", level.symbol());

    if verbose {
        let time = chrono::Local::now().format("%H:%M:%S").to_string();
        let thread_name = thread::current().name().map_or_else(
            || format!("Thread-{:?}", thread::current().id()),
            std::string::ToString::to_string,
        );
        let pid = process::id();

        output.push_str(&format!(
            "({time}) - [PID: {pid} | Thread: {thread_name}] - ({called_in}:{called_from}) : "
        ));
    }

    output.push_str(&message.to_string());

    let colored_output = format!(
        "{}{}{}",
        color::Fg(level.color()),
        output,
        color::Fg(color::Reset)
    );

    println!("{colored_output}");
}

#[macro_export]
macro_rules! log {
    // Match a simple message without verbose
    ($level:expr, $msg:expr) => {
        $crate::log(&$level, false, $msg.to_string(), file!(), line!());
    };
    // Match a formatted message (one or more args) without verbose
    ($level:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::log(&$level, false, format!($fmt, $($arg)+), file!(), line!());
    };
    // Match a simple message with verbose
    ($level:expr, verbose => $msg:expr) => {
        $crate::log(&$level, true, $msg.to_string(), file!(), line!());
    };
    // Match a formatted message (one or more args) with verbose
    ($level:expr, verbose => $fmt:expr, $($arg:tt)+) => {
        $crate::log(&$level, true, format!($fmt, $($arg)+), file!(), line!());
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::INFO, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_dbg {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::DEBUG, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::WARN, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::ERROR, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::SUCCESS, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_fail {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::FAILURE, $($arg)*);
    };
}
