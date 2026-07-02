use chrono::Local;

/// Log level for messages
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Request,
    Security,
    Error,
}

impl LogLevel {
    /// Get the tag string for the log level
    pub fn tag(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Request => "REQUEST",
            LogLevel::Security => "SECURITY",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Logger for the local-web server
#[derive(Clone)]
pub struct Logger {
    verbose: bool,
}

impl Logger {
    /// Create a new logger with the specified verbosity
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Log a message with the specified level
    pub fn log(&self, level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}] [{}] {}", timestamp, level.tag(), message);
    }

    /// Log an info message
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Log a request (only in verbose mode)
    pub fn request(&self, method: &str, path: &str, status: u16) {
        if self.verbose {
            self.log(
                LogLevel::Request,
                &format!("{} {} - {}", method, path, status),
            );
        }
    }

    /// Log a security event (always logged)
    pub fn security(&self, message: &str) {
        self.log(LogLevel::Security, message);
    }

    /// Log an error (always logged)
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_tag() {
        assert_eq!(LogLevel::Info.tag(), "INFO");
        assert_eq!(LogLevel::Request.tag(), "REQUEST");
        assert_eq!(LogLevel::Security.tag(), "SECURITY");
        assert_eq!(LogLevel::Error.tag(), "ERROR");
    }

    #[test]
    fn test_logger_creation() {
        let logger = Logger::new(true);
        assert!(logger.verbose);

        let logger = Logger::new(false);
        assert!(!logger.verbose);
    }

    #[test]
    fn test_default_logger() {
        let logger = Logger::default();
        assert!(!logger.verbose);
    }
}
