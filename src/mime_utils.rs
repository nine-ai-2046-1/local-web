use std::path::Path;

/// Get MIME type based on file extension
pub fn get_mime_type(extension: &str) -> &'static str {
    match extension.to_lowercase().as_str() {
        // HTML
        "html" | "htm" | "xhtml" => "text/html; charset=utf-8",
        // CSS
        "css" => "text/css; charset=utf-8",
        // JavaScript
        "js" | "mjs" => "application/javascript; charset=utf-8",
        // Images
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "bmp" => "image/bmp",
        "tiff" | "tif" => "image/tiff",
        // Video
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "ogg" => "video/ogg",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",
        // Audio
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "oga" => "audio/ogg",
        "aac" => "audio/aac",
        "flac" => "audio/flac",
        "m4a" => "audio/mp4",
        // Fonts
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "eot" => "application/vnd.ms-fontobject",
        // Documents
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        // Text & Data
        "txt" | "md" | "markdown" => "text/plain; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "xml" => "application/xml; charset=utf-8",
        "yaml" | "yml" => "text/yaml; charset=utf-8",
        "csv" => "text/csv; charset=utf-8",
        "tsv" => "text/tab-separated-values; charset=utf-8",
        // Archives
        "zip" => "application/zip",
        "gz" => "application/gzip",
        "tar" => "application/x-tar",
        "rar" => "application/vnd.rar",
        // Other
        "webmanifest" => "application/manifest+json",
        "map" => "application/json; charset=utf-8",
        "log" | "ini" | "cfg" | "conf" => "text/plain; charset=utf-8",
        // Default
        _ => "application/octet-stream",
    }
}

/// Get MIME type from file path
pub fn get_mime_type_from_path(path: &Path) -> &'static str {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| get_mime_type(ext))
        .unwrap_or("application/octet-stream")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_get_mime_type() {
        assert_eq!(get_mime_type("html"), "text/html; charset=utf-8");
        assert_eq!(get_mime_type("css"), "text/css; charset=utf-8");
        assert_eq!(get_mime_type("js"), "application/javascript; charset=utf-8");
        assert_eq!(get_mime_type("png"), "image/png");
        assert_eq!(get_mime_type("jpg"), "image/jpeg");
        assert_eq!(get_mime_type("pdf"), "application/pdf");
        assert_eq!(get_mime_type("txt"), "text/plain; charset=utf-8");
        assert_eq!(get_mime_type("unknown"), "application/octet-stream");
    }

    #[test]
    fn test_get_mime_type_case_insensitive() {
        assert_eq!(get_mime_type("HTML"), "text/html; charset=utf-8");
        assert_eq!(get_mime_type("CSS"), "text/css; charset=utf-8");
        assert_eq!(get_mime_type("PNG"), "image/png");
    }

    #[test]
    fn test_get_mime_type_from_path() {
        assert_eq!(
            get_mime_type_from_path(Path::new("index.html")),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            get_mime_type_from_path(Path::new("style.css")),
            "text/css; charset=utf-8"
        );
        assert_eq!(get_mime_type_from_path(Path::new("image.png")), "image/png");
    }
}
