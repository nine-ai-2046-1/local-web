## Context

The `local-web` tool is a new Rust CLI application designed to serve static files locally with a focus on security and simplicity. It addresses the need for developers to test static websites locally without the risks associated with full-featured development servers.

**Current State**: No existing codebase - this is a greenfield project.

**Constraints**:
- Must bind to localhost only (127.0.0.1)
- Must prevent directory browsing at all levels
- Must only serve whitelisted file extensions
- Must not cache files
- Must be accessible only from the same machine
- Console-only logging with `[xxx]` tag style

**Stakeholders**: Developers testing static HTML/CSS/JS websites locally.

## Goals / Non-Goals

**Goals:**
- Provide a secure, minimal static file server for local development
- Prevent accidental exposure of sensitive files or directory structures
- Support modern static site features (HTML, CSS, JS, media, fonts, documents)
- Offer HTTPS support for testing secure contexts
- Deliver clear, tagged console logging for easy monitoring

**Non-Goals:**
- Not a production web server
- Not a reverse proxy
- Not a CDN or edge server
- No server-side scripting support
- No file upload capabilities
- No WebSocket support
- No HTTP/2 or HTTP/3 (keep it simple)
- No rate limiting (local use only)
- No authentication/authorization

## Decisions

### Decision 1: Use hyper + tokio (Ultra-minimal approach)

**Choice**: hyper 1.10.1 with tokio async runtime

**Alternatives Considered**:
- **actix-web**: More features, but heavier dependency tree
- **axum**: Modern, but still adds abstraction layers
- **warp**: Functional style, less familiar to most developers
- **rouille**: Synchronous, not ideal for async I/O

**Rationale**: hyper provides low-level HTTP control without unnecessary abstractions. Combined with tokio, it gives us async I/O with minimal overhead. This aligns with the "ultra-minimal" requirement.

### Decision 2: Extension Whitelist (Simple Security)

**Choice**: Static array of allowed file extensions

**Alternatives Considered**:
- **MIME type checking**: More accurate but requires external crate
- **Content sniffing**: Resource-intensive, not needed for static sites
- **Both extension + MIME**: Double validation, but overkill for this use case

**Rationale**: Extension whitelist is simple, fast, and sufficient for static site serving. Developers control what files are in the directory, so extension checking provides adequate security without complexity.

### Decision 3: Path Sanitization Strategy

**Choice**: Multiple layers of path validation

**Layers**:
1. Remove null bytes and control characters
2. Decode URL-encoded characters
3. Resolve `..` and `.` path components
4. Normalize path separators
5. Verify resolved path is within root directory

**Alternatives Considered**:
- **Simple string replacement**: Prone to bypass attacks
- **Chroot/jail**: OS-level isolation, but complex cross-platform
- **Symlink resolution**: Add symlink checks

**Rationale**: Multi-layer approach防御各种 path traversal 攻击，包括 URL encoding、double encoding、null bytes 等。Resolved path verification确保最终路径不会逃逸出 root directory。

### Decision 4: HTTPS Implementation

**Choice**: rustls with optional self-signed certificate generation

**Alternatives Considered**:
- **native-tls**: Platform-specific, harder to self-sign
- **openssl**: Heavy dependency, licensing concerns
- **No HTTPS**: Simpler, but limits testing of secure contexts

**Rationale**: rustls is pure Rust, easy to cross-compile, and supports self-signed certs via rcgen. This allows testing HTTPS sites without external certificate tools.

### Decision 5: Error Page Design

**Choice**: Minimal HTML error pages

**Format**:
```html
<!DOCTYPE html>
<html>
<head><title>{STATUS} {MESSAGE}</title></head>
<body>
<h1>{STATUS} {MESSAGE}</h1>
<p>{DETAIL}</p>
</body>
</html>
```

**Alternatives Considered**:
- **JSON errors**: Not user-friendly for browser testing
- **Custom templates**: Adds complexity, not needed
- **Empty responses**: Confusing for users

**Rationale**: Simple HTML pages are immediately visible in browsers, easy to understand, and require no external dependencies.

### Decision 6: Logging Format

**Choice**: Console-only with `[xxx]` tag style

**Format**: `[YYYY-MM-DD HH:MM:SS] [TAG] message`

**Tags**:
- `[INFO]` - Server startup, configuration
- `[REQUEST]` - HTTP requests with status codes
- `[SECURITY]` - Blocked attempts
- `[ERROR]` - Failures and errors

**Alternatives Considered**:
- **File logging**: Adds complexity, not needed for local tool
- **Structured logging (JSON)**: Harder to read in console
- **Third-party logging crates**: Overkill for this use case

**Rationale**: Tagged console output is easy to scan, filter with grep, and requires no configuration. Timestamps help with debugging timing issues.

## Risks / Trade-offs

### Risk 1: Extension Whitelist Bypass
**Risk**: Malicious files with allowed extensions (e.g., `.js` with server-side code)
**Mitigation**: 
- Localhost-only binding limits exposure
- No server-side execution environment
- File content is served as-is, not interpreted

### Risk 2: Path Traversal Variants
**Risk**: Novel path traversal techniques not covered by sanitization
**Mitigation**:
- Multi-layer sanitization approach
- Final path resolution and boundary check
- Conservative whitelist approach

### Risk 3: Self-Signed Certificate Trust
**Risk**: Browsers will show security warnings for self-signed certs
**Mitigation**:
- Clear documentation on how to trust the certificate
- Option to use user-provided trusted certificates
- This is acceptable for local development testing

### Risk 4: Performance with Large Files
**Risk**: Large media files may cause memory issues or slow responses
**Mitigation**:
- Stream file responses instead of buffering entire file
- hyper handles HTTP streaming natively
- Local disk I/O is fast enough for most use cases

### Trade-off: Security vs. Usability
**Choice**: Prioritize security over convenience
- Whitelist blocks potentially useful file types
- No directory browsing requires exact URLs
- Localhost-only prevents remote access

**Acceptable**: For local development, security is more important than convenience.

## Migration Plan

This is a new tool with no existing deployment. No migration needed.

**Rollback Strategy**: N/A - new tool, can simply stop using it.

## Open Questions

1. **File watching**: Should the tool auto-reload when files change? (Not in current scope, but could be added later)
2. **Custom MIME types**: Should users be able to extend the MIME whitelist via config? (Not in current scope)
3. **Compression**: Should the tool support gzip/brotli compression? (Adds complexity, may not be needed for local use)

---

**Decision**: Proceed with current design. Open questions can be addressed in future iterations if needed.