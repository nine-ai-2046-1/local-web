## Why

Developers need a simple, secure way to test static websites locally without the overhead of complex server setups or the security risks of full-featured development servers. Existing tools either expose unnecessary features (directory browsing, script execution) or lack proper security controls. A minimal, security-focused local server ensures safe testing of HTML/CSS/JS sites with media support while preventing accidental exposure of sensitive files or execution of server-side code.

## What Changes

- New Rust CLI tool `local-web` for serving static files locally
- Bind to localhost only (127.0.0.1) for security
- Extension whitelist system to control which files can be served
- Path sanitization to prevent directory traversal attacks
- Directory browsing prevention at all levels (including nested folders)
- No caching headers (fresh content on every request)
- Console logging with `[xxx]` tag style for easy monitoring
- HTTPS support with self-signed certificate generation
- Simple HTML error pages for 403/404 responses

## Capabilities

### New Capabilities

- `static-file-serving`: Core HTTP server functionality for serving static files from a specified directory
- `security-layer`: Extension whitelist, path sanitization, and directory browsing prevention
- `https-support`: TLS/HTTPS with self-signed certificate generation and user-provided certificate support
- `cli-interface`: Command-line argument parsing and user interaction

### Modified Capabilities

<!-- No existing capabilities to modify - this is a new tool -->

## Impact

- **New Codebase**: Entirely new Rust project in `/Users/raymondnodejs20/Documents/test-2046/local-http/`
- **Dependencies**: hyper, tokio, clap, rcgen, rustls, rustls-pemfile
- **Build System**: Cargo.toml with Rust 2021 edition
- **No Breaking Changes**: New tool, no existing code affected
- **No API Changes**: CLI tool, no public API
- **No External Systems**: Local-only operation, no network dependencies beyond localhost