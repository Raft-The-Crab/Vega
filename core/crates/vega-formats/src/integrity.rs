//! Integrity checks and checksums
//!
//! Provides file integrity verification using SHA-256 checksums.

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use thiserror::Error;

/// Integrity check errors.
#[derive(Debug, Error)]
pub enum IntegrityError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
}

/// Result type for integrity operations.
pub type Result<T> = std::result::Result<T, IntegrityError>;

/// Computes SHA-256 checksum of a file.
pub fn compute_checksum<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Verifies a file's checksum against an expected value.
pub fn verify_checksum<P: AsRef<Path>>(path: P, expected: &str) -> Result<()> {
    let actual = compute_checksum(path)?;
    if actual.to_lowercase() != expected.to_lowercase() {
        return Err(IntegrityError::ChecksumMismatch {
            expected: expected.to_string(),
            actual,
        });
    }
    Ok(())
}

/// Computes checksum of a byte slice.
pub fn compute_checksum_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_compute_checksum() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), b"test data").unwrap();

        let checksum = compute_checksum(temp_file.path()).unwrap();
        assert_eq!(checksum.len(), 64); // SHA-256 produces 64 hex chars
    }

    #[test]
    fn test_verify_checksum() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), b"test data").unwrap();

        let checksum = compute_checksum(temp_file.path()).unwrap();
        verify_checksum(temp_file.path(), &checksum).unwrap();
    }

    #[test]
    fn test_verify_checksum_mismatch() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), b"test data").unwrap();

        let result = verify_checksum(temp_file.path(), "wrongchecksum");
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_checksum_bytes() {
        let data = b"test data";
        let checksum = compute_checksum_bytes(data);
        assert_eq!(checksum.len(), 64);
    }
}
