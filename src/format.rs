#[derive(Debug, Clone)]
pub enum FormatPlaceholder {
    Level,
    Symbol,
    Message,
    Time,
    Date,
    DateTime,
    ThreadName,
    ThreadId,
    ProcessId,
    File,
    Line,
    Context,
    Text(String),
}

#[derive(Debug, Clone, Default)]
pub struct FormatTemplate {
    pub(crate) parts: Vec<FormatPlaceholder>,
}

impl FormatTemplate {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn parse(template: &str) -> Self {
        let mut parts = Vec::new();
        let mut current = 0;

        while let Some(start) = template[current..].find('{') {
            let abs_start = current + start;

            // Add preceding text if any
            if abs_start > current {
                parts.push(FormatPlaceholder::Text(
                    template[current..abs_start].to_string(),
                ));
            }

            // Find closing brace
            if let Some(end) = template[abs_start..].find('}') {
                let placeholder = &template[abs_start + 1..abs_start + end];
                let part = match placeholder {
                    "level" => FormatPlaceholder::Level,
                    "symbol" => FormatPlaceholder::Symbol,
                    "message" => FormatPlaceholder::Message,
                    "time" => FormatPlaceholder::Time,
                    "date" => FormatPlaceholder::Date,
                    "datetime" => FormatPlaceholder::DateTime,
                    "thread" => FormatPlaceholder::ThreadName,
                    "thread_id" => FormatPlaceholder::ThreadId,
                    "pid" => FormatPlaceholder::ProcessId,
                    "file" => FormatPlaceholder::File,
                    "line" => FormatPlaceholder::Line,
                    "context" => FormatPlaceholder::Context,
                    _ => FormatPlaceholder::Text(format!("{{{}}}", placeholder)),
                };
                parts.push(part);
                current = abs_start + end + 1;
            } else {
                // No closing brace found, treat rest as text
                parts.push(FormatPlaceholder::Text(template[abs_start..].to_string()));
                break;
            }
        }

        // Add remaining text
        if current < template.len() {
            parts.push(FormatPlaceholder::Text(template[current..].to_string()));
        }

        Self { parts }
    }
}
