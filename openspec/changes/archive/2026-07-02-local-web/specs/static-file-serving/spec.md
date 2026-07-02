## ADDED Requirements

### Requirement: Serve static files from specified directory
The system SHALL serve static files from a user-specified directory path.

#### Scenario: Serve HTML file
- **WHEN** user requests `GET /index.html`
- **THEN** system returns the file content with `Content-Type: text/html` header

#### Scenario: Serve CSS file
- **WHEN** user requests `GET /style.css`
- **THEN** system returns the file content with `Content-Type: text/css` header

#### Scenario: Serve JavaScript file
- **WHEN** user requests `GET /app.js`
- **THEN** system returns the file content with `Content-Type: application/javascript` header

#### Scenario: Serve image file
- **WHEN** user requests `GET /logo.png`
- **THEN** system returns the file content with `Content-Type: image/png` header

#### Scenario: Serve media file
- **WHEN** user requests `GET /video.mp4`
- **THEN** system returns the file content with `Content-Type: video/mp4` header

#### Scenario: Serve font file
- **WHEN** user requests `GET /font.woff2`
- **THEN** system returns the file content with `Content-Type: font/woff2` header

#### Scenario: Serve document file
- **WHEN** user requests `GET /readme.pdf`
- **THEN** system returns the file content with `Content-Type: application/pdf` header

#### Scenario: File not found
- **WHEN** user requests `GET /nonexistent.html`
- **THEN** system returns 404 error page

#### Scenario: Serve index.html for root path
- **WHEN** user requests `GET /`
- **THEN** system returns `index.html` from the root directory

#### Scenario: Serve index.html for directory path
- **WHEN** user requests `GET /subdir/`
- **THEN** system returns `index.html` from `/subdir/` directory

### Requirement: No caching headers
The system SHALL NOT include caching headers in responses.

#### Scenario: Response headers
- **WHEN** any file is served
- **THEN** response includes `Cache-Control: no-store, no-cache` header

### Requirement: Localhost only binding
The system SHALL bind to localhost only (127.0.0.1).

#### Scenario: Server startup
- **WHEN** server starts
- **THEN** it binds to `127.0.0.1:{port}` only

#### Scenario: Remote access blocked
- **WHEN** request comes from non-localhost address
- **THEN** connection is refused