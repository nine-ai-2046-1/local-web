use std::fmt;

/// Error types for the local-web server
#[derive(Debug)]
pub enum ServerError {
    /// IO error
    Io(std::io::Error),
    /// Hyper HTTP error
    Hyper(hyper::Error),
    /// CLI argument error
    Cli(String),
    /// Security error
    Security(String),
    /// TLS/HTTPS error
    Tls(String),
    /// File not found
    NotFound(String),
    /// Forbidden access
    Forbidden(String),
    /// Method not allowed
    MethodNotAllowed(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::Io(e) => write!(f, "IO error: {}", e),
            ServerError::Hyper(e) => write!(f, "HTTP error: {}", e),
            ServerError::Cli(msg) => write!(f, "CLI error: {}", msg),
            ServerError::Security(msg) => write!(f, "Security error: {}", msg),
            ServerError::Tls(msg) => write!(f, "TLS error: {}", msg),
            ServerError::NotFound(path) => write!(f, "Not found: {}", path),
            ServerError::Forbidden(path) => write!(f, "Forbidden: {}", path),
            ServerError::MethodNotAllowed(method) => {
                write!(f, "Method not allowed: {}", method)
            }
        }
    }
}

impl std::error::Error for ServerError {}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        ServerError::Io(e)
    }
}

impl From<hyper::Error> for ServerError {
    fn from(e: hyper::Error) -> Self {
        ServerError::Hyper(e)
    }
}

/// Generate a simple HTML error page
pub fn error_page(status: u16, message: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>{} {}</title>
    <style>
        body {{ font-family: sans-serif; text-align: center; padding: 50px; }}
        h1 {{ color: #333; }}
        p {{ color: #666; }}
    </style>
</head>
<body>
    <h1>{} {}</h1>
    <p>{}</p>
</body>
</html>"#,
        status, message, status, message, message
    )
}

/// Get status text for HTTP status code
pub fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_page() {
        let page = error_page(404, "Not Found");
        assert!(page.contains("404"));
        assert!(page.contains("Not Found"));
        assert!(page.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_status_text() {
        assert_eq!(status_text(200), "OK");
        assert_eq!(status_text(404), "Not Found");
        assert_eq!(status_text(405), "Method Not Allowed");
        assert_eq!(status_text(999), "Unknown");
    }
}
