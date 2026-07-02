use clap::Parser;
use std::net::SocketAddr;

mod cli;
mod error;
mod https;
mod logging;
mod mime_utils;
mod security;
mod server;

use cli::Args;
use logging::Logger;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Validate arguments
    if let Err(e) = args.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Create logger
    let logger = Logger::new(args.verbose);

    // Build socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    // Start server
    if args.https || args.cert.is_some() || args.key.is_some() || args.generate_self_signed {
        // HTTPS mode
        logger.info("Starting HTTPS server...");

        // Get certificate and key
        let (cert, key) = if args.generate_self_signed {
            logger.info("Generating self-signed certificate...");
            match https::generate_self_signed() {
                Ok((cert, key)) => (cert, key),
                Err(e) => {
                    logger.error(&format!("Failed to generate certificate: {}", e));
                    std::process::exit(1);
                }
            }
        } else if let (Some(cert_path), Some(key_path)) = (&args.cert, &args.key) {
            // Load from files
            let cert = match https::load_cert(cert_path) {
                Ok(c) => c,
                Err(e) => {
                    logger.error(&format!("Failed to load certificate: {}", e));
                    std::process::exit(1);
                }
            };
            let key = match https::load_key(key_path) {
                Ok(k) => k,
                Err(e) => {
                    logger.error(&format!("Failed to load key: {}", e));
                    std::process::exit(1);
                }
            };
            (cert, key)
        } else {
            logger.error("HTTPS mode requires either --generate-self-signed or --cert and --key");
            std::process::exit(1);
        };

        // Create TLS config
        let _tls_config = match https::create_tls_config(cert, key) {
            Ok(config) => config,
            Err(e) => {
                logger.error(&format!("Failed to create TLS config: {}", e));
                std::process::exit(1);
            }
        };

        logger.info(&format!("HTTPS server started on https://{}", addr));

        // TODO: Implement HTTPS server startup
        // For now, fall back to HTTP
        logger.info("HTTPS not fully implemented yet, falling back to HTTP");
        if let Err(e) = server::start_server(addr, args.directory, logger).await {
            eprintln!("Server error: {}", e);
            std::process::exit(1);
        }
    } else {
        // HTTP mode
        logger.info("Starting HTTP server...");
        if let Err(e) = server::start_server(addr, args.directory, logger).await {
            eprintln!("Server error: {}", e);
            std::process::exit(1);
        }
    }
}
