use std::path::{Path, PathBuf};

/// List of allowed file extensions for serving
const ALLOWED_EXTENSIONS: &[&str] = &[
    // HTML & Web
    "html",
    "htm",
    "xhtml",
    // CSS
    "css",
    // JavaScript (client-side only)
    "js",
    "mjs",
    // Images
    "jpg",
    "jpeg",
    "png",
    "gif",
    "svg",
    "webp",
    "ico",
    "bmp",
    "tiff",
    "tif",
    // Video
    "mp4",
    "webm",
    "ogg",
    "avi",
    "mov",
    "mkv",
    // Audio
    "mp3",
    "wav",
    "oga",
    "aac",
    "flac",
    "m4a",
    // Fonts
    "woff",
    "woff2",
    "ttf",
    "otf",
    "eot",
    // Documents
    "pdf",
    "doc",
    "docx",
    "xls",
    "xlsx",
    "ppt",
    "pptx",
    // Text & Data
    "txt",
    "md",
    "markdown",
    "json",
    "xml",
    "yaml",
    "yml",
    "csv",
    "tsv",
    // Archives (read-only, no execution)
    "zip",
    "gz",
    "tar",
    "rar",
    // Other Common
    "webmanifest",
    "map",
    "log",
    "ini",
    "cfg",
    "conf",
];

/// Check if a file extension is allowed
pub fn is_extension_allowed(extension: &str) -> bool {
    ALLOWED_EXTENSIONS.contains(&extension.to_lowercase().as_str())
}

/// Get the file extension from a path
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

/// Validate that a path is safe (no traversal attacks)
pub fn validate_path(path: &str, root_dir: &Path) -> Result<PathBuf, String> {
    // Remove null bytes and control characters
    let sanitized = path
        .chars()
        .filter(|c| *c != '\0' && !c.is_control())
        .collect::<String>();

    // Decode URL-encoded characters (handle double encoding)
    let decoded = url_decode(&sanitized)?;

    // Check for path traversal patterns
    if decoded.contains("..") || decoded.contains("//") || decoded.contains('\\') {
        return Err("Path traversal detected".to_string());
    }

    // Resolve the path relative to root
    let clean_path = if let Some(stripped) = decoded.strip_prefix('/') {
        stripped
    } else {
        &decoded
    };

    let resolved = root_dir.join(clean_path);

    // Normalize the path
    let normalized = match std::fs::canonicalize(&resolved) {
        Ok(p) => p,
        Err(_) => return Err("Path does not exist".to_string()),
    };

    // Verify the resolved path is within root directory
    let canonical_root = match std::fs::canonicalize(root_dir) {
        Ok(p) => p,
        Err(_) => return Err("Root directory is not accessible".to_string()),
    };

    if !normalized.starts_with(&canonical_root) {
        return Err("Path escapes root directory".to_string());
    }

    Ok(normalized)
}

/// URL decode a string with double-encoding prevention
fn url_decode(input: &str) -> Result<String, String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            // Get next two characters
            let hex1 = chars.next().ok_or("Invalid URL encoding")?;
            let hex2 = chars.next().ok_or("Invalid URL encoding")?;

            // Decode the hex value
            let hex_str = format!("{}{}", hex1, hex2);
            let byte = u8::from_str_radix(&hex_str, 16)
                .map_err(|_| format!("Invalid hex value: {}", hex_str))?;

            // Check for dangerous characters
            match byte {
                // Null byte
                0x00 => return Err("Null byte detected".to_string()),
                // Control characters
                0x01..=0x1f | 0x7f => {
                    return Err("Control character detected".to_string());
                }
                // Forward slash
                0x2f => result.push('/'),
                // Backslash
                0x5c => result.push('\\'),
                // Dot
                0x2e => result.push('.'),
                // Other bytes
                _ => result.push(byte as char),
            }
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

/// Check if a path is a directory
pub fn is_directory(path: &Path) -> bool {
    path.is_dir()
}

/// Check if a path ends with a slash (directory indicator)
pub fn is_directory_request(path: &str) -> bool {
    path.ends_with('/')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_extension_allowed() {
        assert!(is_extension_allowed("html"));
        assert!(is_extension_allowed("CSS"));
        assert!(is_extension_allowed("Js"));
        assert!(is_extension_allowed("png"));
        assert!(is_extension_allowed("pdf"));
        assert!(!is_extension_allowed("exe"));
        assert!(!is_extension_allowed("sh"));
        assert!(!is_extension_allowed(""));
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(
            get_extension(Path::new("index.html")),
            Some("html".to_string())
        );
        assert_eq!(
            get_extension(Path::new("style.CSS")),
            Some("css".to_string())
        );
        assert_eq!(
            get_extension(Path::new("image.PNG")),
            Some("png".to_string())
        );
        assert_eq!(get_extension(Path::new("noext")), None);
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(url_decode("hello").unwrap(), "hello");
        assert_eq!(url_decode("hello%20world").unwrap(), "hello world");
        assert_eq!(url_decode("%2e%2e").unwrap(), "..");
        assert!(url_decode("%00").is_err());
        assert!(url_decode("%01").is_err());
    }

    #[test]
    fn test_is_directory_request() {
        assert!(is_directory_request("/"));
        assert!(is_directory_request("/subdir/"));
        assert!(!is_directory_request("/index.html"));
        assert!(!is_directory_request("/file.txt"));
    }
}
