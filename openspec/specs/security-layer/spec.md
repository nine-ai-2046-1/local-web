## ADDED Requirements

### Requirement: Extension whitelist validation
The system SHALL only serve files with whitelisted extensions.

#### Scenario: Allowed extension
- **WHEN** user requests a file with extension `.html`, `.css`, `.js`, `.jpg`, `.png`, `.gif`, `.svg`, `.webp`, `.ico`, `.bmp`, `.tiff`, `.tif`, `.mp4`, `.webm`, `.ogg`, `.avi`, `.mov`, `.mkv`, `.mp3`, `.wav`, `.oga`, `.aac`, `.flac`, `.m4a`, `.woff`, `.woff2`, `.ttf`, `.otf`, `.eot`, `.pdf`, `.doc`, `.docx`, `.xls`, `.xlsx`, `.ppt`, `.pptx`, `.txt`, `.md`, `.markdown`, `.json`, `.xml`, `.yaml`, `.yml`, `.csv`, `.tsv`, `.zip`, `.gz`, `.tar`, `.rar`, `.webmanifest`, `.map`, `.log`, `.ini`, `.cfg`, `.conf`
- **THEN** system serves the file

#### Scenario: Disallowed extension
- **WHEN** user requests a file with extension `.exe`, `.sh`, `.py`, `.php`, `.rb`, `.pl`, `.cgi`, `.bat`, `.cmd`, `.ps1`, `.vbs`, `.js` (server-side), or any other non-whitelisted extension
- **THEN** system returns 403 Forbidden error

#### Scenario: No extension
- **WHEN** user requests a file with no extension
- **THEN** system returns 403 Forbidden error

### Requirement: Path traversal prevention
The system SHALL prevent directory traversal attacks.

#### Scenario: Simple path traversal
- **WHEN** user requests `GET /../../etc/passwd`
- **THEN** system returns 403 Forbidden error

#### Scenario: URL-encoded path traversal
- **WHEN** user requests `GET /%2e%2e/%2e%2e/etc/passwd`
- **THEN** system returns 403 Forbidden error

#### Scenario: Double-encoded path traversal
- **WHEN** user requests `GET /%252e%252e/%252e%252e/etc/passwd`
- **THEN** system returns 403 Forbidden error

#### Scenario: Null byte injection
- **WHEN** user requests `GET /index.html%00.jpg`
- **THEN** system returns 403 Forbidden error

#### Scenario: Backslash path traversal (Windows)
- **WHEN** user requests `GET /..\\..\\etc\\passwd`
- **THEN** system returns 403 Forbidden error

### Requirement: Directory browsing prevention
The system SHALL prevent directory browsing at all levels.

#### Scenario: Root directory listing
- **WHEN** user requests `GET /` and no `index.html` exists
- **THEN** system returns 403 Forbidden error

#### Scenario: Subdirectory listing
- **WHEN** user requests `GET /subdir/` and no `index.html` exists in `/subdir/`
- **THEN** system returns 403 Forbidden error

#### Scenario: Nested subdirectory listing
- **WHEN** user requests `GET /a/b/c/` and no `index.html` exists in `/a/b/c/`
- **THEN** system returns 403 Forbidden error

#### Scenario: Directory traversal via browsing attempt
- **WHEN** user requests `GET /../`
- **THEN** system returns 403 Forbidden error

### Requirement: HTTP method restriction
The system SHALL only allow GET and HEAD methods.

#### Scenario: GET request
- **WHEN** user sends `GET` request
- **THEN** system processes the request normally

#### Scenario: HEAD request
- **WHEN** user sends `HEAD` request
- **THEN** system returns response headers without body

#### Scenario: POST request blocked
- **WHEN** user sends `POST` request
- **THEN** system returns 405 Method Not Allowed error

#### Scenario: PUT request blocked
- **WHEN** user sends `PUT` request
- **THEN** system returns 405 Method Not Allowed error

#### Scenario: DELETE request blocked
- **WHEN** user sends `DELETE` request
- **THEN** system returns 405 Method Not Allowed error

#### Scenario: PATCH request blocked
- **WHEN** user sends `PATCH` request
- **THEN** system returns 405 Method Not Allowed error

#### Scenario: OPTIONS request blocked
- **WHEN** user sends `OPTIONS` request
- **THEN** system returns 405 Method Not Allowed error