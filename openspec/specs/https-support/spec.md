## ADDED Requirements

### Requirement: HTTPS support with self-signed certificate
The system SHALL support HTTPS with self-signed certificate generation.

#### Scenario: Generate self-signed certificate
- **WHEN** user runs `local-web --generate-self-signed -p 8443 -d ./site`
- **THEN** system generates a self-signed certificate and starts HTTPS server

#### Scenario: Self-signed certificate details
- **WHEN** self-signed certificate is generated
- **THEN** certificate uses RSA 2048-bit key or ECDSA P-256
- **AND** certificate is valid for 1 year
- **AND** certificate includes `localhost` and `127.0.0.1` as Subject Alternative Names

### Requirement: User-provided certificate support
The system SHALL support user-provided certificates.

#### Scenario: Provide cert and key files
- **WHEN** user runs `local-web --cert /path/to/cert.pem --key /path/to/key.pem -p 8443 -d ./site`
- **THEN** system loads the provided certificate and starts HTTPS server

#### Scenario: Invalid certificate file
- **WHEN** user provides invalid certificate file path
- **THEN** system displays error message and exits

#### Scenario: Invalid key file
- **WHEN** user provides invalid key file path
- **THEN** system displays error message and exits

#### Scenario: Certificate and key mismatch
- **WHEN** user provides certificate and key that don't match
- **THEN** system displays error message and exits

### Requirement: HTTPS and HTTP mutual exclusivity
The system SHALL NOT run both HTTP and HTTPS simultaneously.

#### Scenario: HTTPS mode
- **WHEN** user provides `--https`, `--cert`, `--key`, or `--generate-self-signed`
- **THEN** system runs HTTPS server only

#### Scenario: HTTP mode (default)
- **WHEN** user does not provide any HTTPS options
- **THEN** system runs HTTP server only

### Requirement: TLS version and cipher suite
The system SHALL use modern TLS standards.

#### Scenario: TLS version
- **WHEN** HTTPS server starts
- **THEN** it supports TLS 1.2 and TLS 1.3 only

#### Scenario: Cipher suites
- **WHEN** HTTPS server starts
- **THEN** it uses secure cipher suites (no weak ciphers like RC4, DES, 3DES)