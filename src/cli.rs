use clap::Parser;
use std::path::PathBuf;

/// A simple, secure local static file server
#[derive(Parser, Debug)]
#[command(name = "local-web")]
#[command(about = "Serve static files locally with security controls")]
#[command(version)]
pub struct Args {
    /// Port number to listen on
    #[arg(short, long)]
    pub port: u16,

    /// Directory path to serve files from
    #[arg(short, long)]
    pub directory: PathBuf,

    /// Enable HTTPS
    #[arg(long)]
    pub https: bool,

    /// Path to TLS certificate file (PEM format)
    #[arg(long)]
    pub cert: Option<PathBuf>,

    /// Path to TLS private key file (PEM format)
    #[arg(long)]
    pub key: Option<PathBuf>,

    /// Generate self-signed certificate for HTTPS
    #[arg(long)]
    pub generate_self_signed: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    /// Validate the parsed arguments
    pub fn validate(&self) -> Result<(), String> {
        // Validate port range
        if self.port == 0 {
            return Err("Port must be greater than 0".to_string());
        }

        // Validate directory exists and is a directory
        if !self.directory.exists() {
            return Err(format!(
                "Directory does not exist: {}",
                self.directory.display()
            ));
        }

        if !self.directory.is_dir() {
            return Err(format!(
                "Path is not a directory: {}",
                self.directory.display()
            ));
        }

        // Validate HTTPS options
        if self.https || self.cert.is_some() || self.key.is_some() || self.generate_self_signed {
            // HTTPS mode - validate cert/key if provided
            if let (Some(cert), Some(key)) = (&self.cert, &self.key) {
                if !cert.exists() {
                    return Err(format!(
                        "Certificate file does not exist: {}",
                        cert.display()
                    ));
                }
                if !key.exists() {
                    return Err(format!("Key file does not exist: {}", key.display()));
                }
            } else if self.cert.is_some() || self.key.is_some() {
                return Err("Both --cert and --key must be provided together".to_string());
            }
        }

        Ok(())
    }
}
