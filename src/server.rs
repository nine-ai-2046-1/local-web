use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::net::TcpListener;

use crate::error::{error_page, ServerError};
use crate::logging::Logger;
use crate::mime_utils::get_mime_type_from_path;
use crate::security::{get_extension, is_directory_request, is_extension_allowed, validate_path};

/// Shared state for the server
pub struct ServerState {
    pub root_dir: PathBuf,
    pub logger: Logger,
}

/// Type alias for the response body
pub type BoxBody = Full<Bytes>;

/// Start the HTTP server
pub async fn start_server(
    addr: SocketAddr,
    root_dir: PathBuf,
    logger: Logger,
) -> Result<(), ServerError> {
    let listener = TcpListener::bind(addr).await?;
    logger.info(&format!("Server started on http://{}", addr));
    logger.info(&format!("Serving directory: {}", root_dir.display()));

    let state = Arc::new(ServerState {
        root_dir: root_dir.clone(),
        logger: logger.clone(),
    });

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let state = state.clone();
        let logger = state.logger.clone();

        tokio::spawn(async move {
            let service = service_fn(move |req| {
                let state = state.clone();
                async move { handle_request(req, &state).await }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                logger.error(&format!("HTTP error: {}", err));
            }
        });
    }
}

/// Handle an incoming HTTP request
async fn handle_request(
    req: Request<Incoming>,
    state: &ServerState,
) -> Result<Response<BoxBody>, ServerError> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path();

    // Log the request
    state.logger.request(method.as_ref(), path, 0);

    // Check HTTP method (only GET and HEAD allowed)
    if method != hyper::Method::GET && method != hyper::Method::HEAD {
        state
            .logger
            .security(&format!("Blocked {} {} - Method not allowed", method, path));
        return Ok(create_response(
            StatusCode::METHOD_NOT_ALLOWED,
            error_page(405, "Method Not Allowed"),
        ));
    }

    // Validate and sanitize the path
    let safe_path = match validate_path(path, &state.root_dir) {
        Ok(p) => p,
        Err(e) => {
            state.logger.security(&format!("Blocked {} - {}", path, e));
            return Ok(create_response(
                StatusCode::FORBIDDEN,
                error_page(403, "Forbidden"),
            ));
        }
    };

    // Check if it's a directory request
    if is_directory_request(path) {
        // Try to serve index.html
        let index_path = safe_path.join("index.html");
        if index_path.exists() {
            return serve_file(&index_path, &method, state).await;
        } else {
            // No index.html - prevent directory browsing
            state
                .logger
                .security(&format!("Blocked directory listing attempt: {}", path));
            return Ok(create_response(
                StatusCode::FORBIDDEN,
                error_page(403, "Forbidden"),
            ));
        }
    }

    // Check if path exists
    if !safe_path.exists() {
        state.logger.error(&format!("File not found: {}", path));
        return Ok(create_response(
            StatusCode::NOT_FOUND,
            error_page(404, "Not Found"),
        ));
    }

    // Check if it's a directory (without trailing slash)
    if safe_path.is_dir() {
        // Redirect to directory with trailing slash
        let redirect_path = format!("{}/", path);
        let mut response = Response::new(Full::new(Bytes::from(format!(
            "Redirecting to {}",
            redirect_path
        ))));
        *response.status_mut() = StatusCode::FOUND;
        response
            .headers_mut()
            .insert("location", redirect_path.parse().unwrap());
        return Ok(response);
    }

    // Check file extension
    let extension = match get_extension(&safe_path) {
        Some(ext) => ext,
        None => {
            state
                .logger
                .security(&format!("Blocked {} - No file extension", path));
            return Ok(create_response(
                StatusCode::FORBIDDEN,
                error_page(403, "Forbidden"),
            ));
        }
    };

    if !is_extension_allowed(&extension) {
        state.logger.security(&format!(
            "Blocked {} - Extension not allowed: {}",
            path, extension
        ));
        return Ok(create_response(
            StatusCode::FORBIDDEN,
            error_page(403, "Forbidden"),
        ));
    }

    // Serve the file
    serve_file(&safe_path, &method, state).await
}

/// Serve a file from disk
async fn serve_file(
    path: &Path,
    method: &hyper::Method,
    state: &ServerState,
) -> Result<Response<BoxBody>, ServerError> {
    // Read the file
    let contents = match fs::read(path).await {
        Ok(c) => c,
        Err(e) => {
            state
                .logger
                .error(&format!("Failed to read file: {} - {}", path.display(), e));
            return Ok(create_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                error_page(500, "Internal Server Error"),
            ));
        }
    };

    // Get MIME type
    let mime_type = get_mime_type_from_path(path);

    // Build response
    let mut response = Response::new(Full::new(Bytes::from(contents)));
    *response.status_mut() = StatusCode::OK;

    // Set Content-Type header
    response
        .headers_mut()
        .insert("content-type", mime_type.parse().unwrap());

    // Disable caching
    response.headers_mut().insert(
        "cache-control",
        "no-store, no-cache, must-revalidate".parse().unwrap(),
    );

    // Set Connection header
    response
        .headers_mut()
        .insert("connection", "close".parse().unwrap());

    // Log the successful response
    state
        .logger
        .request(method.as_ref(), &path.display().to_string(), 200);

    Ok(response)
}

/// Create a response with the given status code and body
fn create_response(status: StatusCode, body: String) -> Response<BoxBody> {
    let mut response = Response::new(Full::new(Bytes::from(body)));
    *response.status_mut() = status;
    response
        .headers_mut()
        .insert("content-type", "text/html; charset=utf-8".parse().unwrap());
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_response() {
        let response = create_response(StatusCode::OK, "test body".to_string());
        assert_eq!(response.status(), StatusCode::OK);
    }
}
