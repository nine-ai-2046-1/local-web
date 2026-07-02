use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::error::ServerError;

/// Configuration for HTTPS
pub struct HttpsConfig {
    pub cert: CertificateDer<'static>,
    pub key: PrivateKeyDer<'static>,
}

/// Generate a self-signed certificate for localhost
pub fn generate_self_signed_cert() -> Result<CertificateDer<'static>, ServerError> {
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];

    let cert = rcgen::generate_simple_self_signed(subject_alt_names)
        .map_err(|e| ServerError::Tls(format!("Failed to generate certificate: {}", e)))?;

    let cert_der = CertificateDer::from(cert.cert.der().to_vec());
    Ok(cert_der)
}

/// Generate a self-signed private key
pub fn generate_self_signed_key() -> Result<PrivateKeyDer<'static>, ServerError> {
    let key_pair = rcgen::KeyPair::generate()
        .map_err(|e| ServerError::Tls(format!("Failed to generate key pair: {}", e)))?;

    let key_der = PrivateKeyDer::try_from(key_pair.serialize_der())
        .map_err(|e| ServerError::Tls(format!("Failed to serialize key: {}", e)))?;

    Ok(key_der)
}

/// Generate a self-signed certificate and key pair
pub fn generate_self_signed(
) -> Result<(CertificateDer<'static>, PrivateKeyDer<'static>), ServerError> {
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];

    let cert = rcgen::generate_simple_self_signed(subject_alt_names)
        .map_err(|e| ServerError::Tls(format!("Failed to generate certificate: {}", e)))?;

    let cert_der = CertificateDer::from(cert.cert.der().to_vec());
    let key_der = PrivateKeyDer::try_from(cert.key_pair.serialize_der())
        .map_err(|e| ServerError::Tls(format!("Failed to serialize key: {}", e)))?;

    Ok((cert_der, key_der))
}

/// Load certificate from PEM file
pub fn load_cert(cert_path: &PathBuf) -> Result<CertificateDer<'static>, ServerError> {
    let cert_pem = fs::read_to_string(cert_path)
        .map_err(|e| ServerError::Tls(format!("Failed to read certificate file: {}", e)))?;

    let cert_der = rustls_pemfile::certs(&mut cert_pem.as_bytes())
        .next()
        .ok_or_else(|| ServerError::Tls("No certificate found in file".to_string()))?
        .map_err(|e| ServerError::Tls(format!("Failed to parse certificate: {}", e)))?;

    Ok(cert_der)
}

/// Load private key from PEM file
pub fn load_key(key_path: &PathBuf) -> Result<PrivateKeyDer<'static>, ServerError> {
    let key_pem = fs::read_to_string(key_path)
        .map_err(|e| ServerError::Tls(format!("Failed to read key file: {}", e)))?;

    let key_der = rustls_pemfile::private_key(&mut key_pem.as_bytes())
        .map_err(|e| ServerError::Tls(format!("Failed to parse private key: {}", e)))?
        .ok_or_else(|| ServerError::Tls("No private key found in file".to_string()))?;

    Ok(key_der)
}

/// Create a TLS server configuration
pub fn create_tls_config(
    cert: CertificateDer<'static>,
    key: PrivateKeyDer<'static>,
) -> Result<Arc<ServerConfig>, ServerError> {
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)
        .map_err(|e| ServerError::Tls(format!("Failed to create TLS config: {}", e)))?;

    Ok(Arc::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_self_signed() {
        let result = generate_self_signed();
        assert!(result.is_ok());
        let (cert, key) = result.unwrap();
        assert!(!cert.is_empty());
    }

    #[test]
    fn test_create_tls_config() {
        let (cert, key) = generate_self_signed().unwrap();
        let result = create_tls_config(cert, key);
        assert!(result.is_ok());
    }
}
