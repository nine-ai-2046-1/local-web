## ADDED Requirements

### Requirement: Command-line argument parsing
The system SHALL parse command-line arguments using clap.

#### Scenario: Required arguments
- **WHEN** user runs `local-web -p 8080 -d ./site`
- **THEN** system parses port as `8080` and directory as `./site`

#### Scenario: Long form arguments
- **WHEN** user runs `local-web --port 8080 --directory ./site`
- **THEN** system parses port as `8080` and directory as `./site`

#### Scenario: Missing port argument
- **WHEN** user runs `local-web -d ./site`
- **THEN** system displays error message and exits

#### Scenario: Missing directory argument
- **WHEN** user runs `local-web -p 8080`
- **THEN** system displays error message and exits

#### Scenario: Invalid port number
- **WHEN** user runs `local-web -p abc -d ./site`
- **THEN** system displays error message and exits

#### Scenario: Port out of range
- **WHEN** user runs `local-web -p 99999 -d ./site`
- **THEN** system displays error message and exits

### Requirement: Directory validation
The system SHALL validate the specified directory exists.

#### Scenario: Valid directory
- **WHEN** user specifies existing directory
- **THEN** system accepts the directory

#### Scenario: Non-existent directory
- **WHEN** user specifies non-existent directory
- **THEN** system displays error message and exits

#### Scenario: Path is a file, not directory
- **WHEN** user specifies a file path instead of directory
- **THEN** system displays error message and exits

### Requirement: Help and version information
The system SHALL display help and version information.

#### Scenario: Show help
- **WHEN** user runs `local-web --help`
- **THEN** system displays usage instructions and exits

#### Scenario: Show version
- **WHEN** user runs `local-web --version`
- **THEN** system displays version number and exits

### Requirement: Verbose logging option
The system SHALL support verbose logging mode.

#### Scenario: Verbose mode enabled
- **WHEN** user runs `local-web -v -p 8080 -d ./site`
- **THEN** system displays detailed request/response information

#### Scenario: Verbose mode disabled (default)
- **WHEN** user runs `local-web -p 8080 -d ./site`
- **THEN** system displays minimal logging (startup, errors, blocked attempts)

### Requirement: Console logging format
The system SHALL log messages to console with `[xxx]` tag style.

#### Scenario: Info message format
- **WHEN** server starts successfully
- **THEN** logs `[YYYY-MM-DD HH:MM:SS] [INFO] Server started on http://127.0.0.1:{port}`

#### Scenario: Request message format
- **WHEN** server receives a request
- **THEN** logs `[YYYY-MM-DD HH:MM:SS] [REQUEST] GET /path - {status_code}`

#### Scenario: Security message format
- **WHEN** server blocks a request
- **THEN** logs `[YYYY-MM-DD HH:MM:SS] [SECURITY] Blocked {method} /path - {reason}`

#### Scenario: Error message format
- **WHEN** server encounters an error
- **THEN** logs `[YYYY-MM-DD HH:MM:SS] [ERROR] {error_message}`