# local-web

A simple, secure local static file server written in Rust.

## Features

- **Secure**: Only serves whitelisted file extensions (HTML, CSS, JS, images, media, documents)
- **Localhost only**: Binds to 127.0.0.1 for security
- **No caching**: Every request serves fresh content
- **Directory browsing prevention**: Blocks all directory listing attempts
- **Path traversal protection**: Multi-layer security against directory traversal attacks
- **HTTPS support**: Self-signed certificate generation or user-provided certificates
- **Console logging**: Tagged logging with `[xxx]` style for easy monitoring

## Installation

Download a prebuilt binary from the
[Releases page](https://github.com/nine-ai-2046-1/local-web/releases) — no Rust toolchain needed.

| Platform | File |
|---|---|
| Linux x86_64 (static, any distro) | `local-web-<version>-x86_64-unknown-linux-musl.tar.gz` |
| Linux ARM64 (static) | `local-web-<version>-aarch64-unknown-linux-musl.tar.gz` |
| Windows x64 | `local-web-<version>-x86_64-pc-windows-msvc.zip` |
| macOS Apple Silicon | `local-web-<version>-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `local-web-<version>-x86_64-apple-darwin.tar.gz` |

Each archive ships with a matching `.sha256` file. Verify it first:

```bash
sha256sum -c local-web-<version>-<target>.tar.gz.sha256
```

Then unpack and run:

```bash
# Linux / macOS
tar -xzf local-web-<version>-<target>.tar.gz
cd local-web-<version>-<target>
./local-web -p 8080 -d ./site
```

```powershell
# Windows
Expand-Archive local-web-<version>-x86_64-pc-windows-msvc.zip -DestinationPath .
.\local-web-<version>-x86_64-pc-windows-msvc\local-web.exe -p 8080 -d .\site
```

On macOS the binary is unsigned; if Gatekeeper blocks it, run
`xattr -d com.apple.quarantine ./local-web` or allow it in System Settings.

<details>
<summary>Build from source</summary>

```bash
cargo build --release
# The binary will be in target/release/local-web
```

</details>

## Usage

```bash
# Basic HTTP server
local-web -p 8080 -d ./site

# With HTTPS (self-signed certificate)
local-web -p 8443 -d ./site --generate-self-signed

# With HTTPS (user-provided certificate)
local-web -p 8443 -d ./site --cert cert.pem --key key.pem

# Verbose logging
local-web -p 8080 -d ./site -v
```

## Command Line Options

| Option | Description |
|--------|-------------|
| `-p, --port <PORT>` | Port number to listen on (required) |
| `-d, --directory <DIR>` | Directory to serve files from (required) |
| `--https` | Enable HTTPS mode |
| `--cert <PATH>` | Path to TLS certificate file (PEM format) |
| `--key <PATH>` | Path to TLS private key file (PEM format) |
| `--generate-self-signed` | Generate self-signed certificate for HTTPS |
| `-v, --verbose` | Enable verbose logging |
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

## Supported File Types

### Web Files
- HTML: `.html`, `.htm`, `.xhtml`
- CSS: `.css`
- JavaScript: `.js`, `.mjs`

### Images
- `.jpg`, `.jpeg`, `.png`, `.gif`, `.svg`, `.webp`, `.ico`, `.bmp`, `.tiff`, `.tif`

### Video
- `.mp4`, `.webm`, `.ogg`, `.avi`, `.mov`, `.mkv`

### Audio
- `.mp3`, `.wav`, `.oga`, `.aac`, `.flac`, `.m4a`

### Fonts
- `.woff`, `.woff2`, `.ttf`, `.otf`, `.eot`

### Documents
- `.pdf`, `.doc`, `.docx`, `.xls`, `.xlsx`, `.ppt`, `.pptx`

### Text & Data
- `.txt`, `.md`, `.markdown`, `.json`, `.xml`, `.yaml`, `.yml`, `.csv`, `.tsv`

### Archives
- `.zip`, `.gz`, `.tar`, `.rar`

### Other
- `.webmanifest`, `.map`, `.log`, `.ini`, `.cfg`, `.conf`

## Security Features

### Extension Whitelist
Only files with whitelisted extensions can be served. All other file types return 403 Forbidden.

### Path Traversal Prevention
Multiple layers of protection against directory traversal attacks:
- Null byte injection prevention
- URL decoding with double-encoding prevention
- Path normalization and resolution
- Boundary verification (ensures path stays within root directory)

### Directory Browsing Prevention
- No directory listing at any level
- Requests to directories without `index.html` return 403 Forbidden
- Automatic redirect to add trailing slash for directory paths

### HTTP Method Restriction
Only `GET` and `HEAD` methods are allowed. All other methods return 405 Method Not Allowed.

### Localhost Only
Server binds to `127.0.0.1` only, preventing remote access.

### No Caching
All responses include `Cache-Control: no-store, no-cache` headers.

## Logging

The server logs to console with tagged format:

```
[YYYY-MM-DD HH:MM:SS] [TAG] message
```

### Log Tags
- `[INFO]` - Server startup and configuration
- `[REQUEST]` - HTTP requests (only in verbose mode)
- `[SECURITY]` - Blocked attempts (always logged)
- `[ERROR]` - Failures and errors (always logged)

## Examples

### Serve a static site
```bash
local-web -p 3000 -d ./my-website
```

### Serve with HTTPS
```bash
local-web -p 8443 -d ./my-website --generate-self-signed
```

### Verbose logging for debugging
```bash
local-web -p 8080 -d ./my-website -v
```

## Development

### Build
```bash
cargo build
```

### Run tests
```bash
cargo test
```

### Format code
```bash
cargo fmt
```

### Lint code
```bash
cargo clippy
```

## License

MIT
