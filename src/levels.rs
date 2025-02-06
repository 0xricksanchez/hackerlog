use termion::color;

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
    pub const fn symbol(&self) -> &str {
        match self {
            Self::INFO => "[>]",
            Self::DEBUG => "[#]",
            Self::WARN => "[!]",
            Self::ERROR => "[x]",
            Self::SUCCESS => "[+]",
            Self::FAILURE => "[-]",
        }
    }

    pub fn color(&self) -> impl color::Color {
        match *self {
            Self::INFO => color::Rgb(255, 255, 255),
            Self::DEBUG => color::Rgb(100, 100, 255),
            Self::WARN => color::Rgb(255, 165, 0),
            Self::ERROR => color::Rgb(255, 0, 0),
            Self::SUCCESS => color::Rgb(0, 255, 0),
            Self::FAILURE => color::Rgb(139, 0, 0),
        }
    }
}
