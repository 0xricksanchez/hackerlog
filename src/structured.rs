#[cfg(feature = "structured")]
pub mod structured {
    use crate::levels::LogLevel;
    use serde::Serialize;
    use serde_json::Value;
    use std::collections::BTreeMap;

    #[derive(Debug, Clone)]
    pub struct LogEvent {
        pub(crate) level: LogLevel,
        pub(crate) message: String,
        pub(crate) file: String,
        pub(crate) line: u32,
        pub(crate) fields: BTreeMap<String, Value>,
    }

    impl LogEvent {
        pub fn new(level: LogLevel, message: String, file: String, line: u32) -> Self {
            Self {
                level,
                message,
                file,
                line,
                fields: BTreeMap::new(),
            }
        }

        pub fn add_field<T: Serialize>(&mut self, key: &str, value: T) -> &mut Self {
            if let Ok(value) = serde_json::to_value(value) {
                self.fields.insert(key.to_string(), value);
            }
            self
        }
    }
}
