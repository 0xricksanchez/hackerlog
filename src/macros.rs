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

#[macro_export]
macro_rules! time {
    ($name:expr) => {
        let _timer = $crate::TimedOperation::new($name, $crate::LogLevel::INFO);
    };
    ($name:expr, $level:expr) => {
        let _timer = $crate::TimedOperation::new($name, $level);
    };
}

#[cfg(feature = "structured")]
#[macro_export]
macro_rules! event {
    // Base case with just message
    ($level:expr, $msg:expr) => {{
        let event = $crate::LogEvent::new(
            $level,
            $msg.to_string(),
            file!().to_string(),
            line!(),
        );
        $crate::logger().write_structured_event(&event).ok();
    }};

    // Message with fields
    ($level:expr, $msg:expr, {$($key:expr => $value:expr),* $(,)?}) => {{
        let mut event = $crate::LogEvent::new(
            $level,
            $msg.to_string(),
            file!().to_string(),
            line!(),
        );
        $(
            event.add_field($key, $value);
        )*
        $crate::logger().write_structured_event(&event).ok();
    }};
}

#[cfg(feature = "structured")]
#[macro_export]
macro_rules! info_event {
    ($($args:tt)*) => {
        $crate::event!($crate::LogLevel::INFO, $($args)*)
    };
}

#[cfg(feature = "structured")]
#[macro_export]
macro_rules! debug_event {
    ($($args:tt)*) => {
        $crate::event!($crate::LogLevel::DEBUG, $($args)*)
    };
}

#[cfg(feature = "structured")]
#[macro_export]
macro_rules! warn_event {
    ($($args:tt)*) => {
        $crate::event!($crate::LogLevel::WARN, $($args)*)
    };
}

#[cfg(feature = "structured")]
#[macro_export]
macro_rules! error_event {
    ($($args:tt)*) => {
        $crate::event!($crate::LogLevel::ERROR, $($args)*)
    };
}
