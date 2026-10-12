//! Resource limits and validation
//!
//! Provides resource limits for memory and performance protection.

use thiserror::Error;

/// Maximum document dimensions.
pub const MAX_DOCUMENT_WIDTH: u32 = 100_000;
pub const MAX_DOCUMENT_HEIGHT: u32 = 100_000;

/// Minimum document dimensions.
pub const MIN_DOCUMENT_WIDTH: u32 = 1;
pub const MIN_DOCUMENT_HEIGHT: u32 = 1;

/// Maximum number of nodes per document.
pub const MAX_NODES: usize = 10_000;

/// Maximum group depth.
pub const MAX_GROUP_DEPTH: usize = 100;

/// Maximum name length in bytes.
pub const MAX_NAME_LENGTH: usize = 256;

/// Default tile size.
pub const DEFAULT_TILE_SIZE: u32 = 256;

/// Tile size range.
pub const MIN_TILE_SIZE: u32 = 64;
pub const MAX_TILE_SIZE: u32 = 4096;

/// Default history size.
pub const DEFAULT_HISTORY_SIZE: usize = 100;

/// Maximum history size.
pub const MAX_HISTORY_SIZE: usize = 1000;

/// Maximum metadata value size in bytes.
pub const MAX_METADATA_VALUE_SIZE: usize = 1024;

/// Maximum metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 100;

/// Validates document dimensions.
pub fn validate_dimensions(width: u32, height: u32) -> Result<(), LimitError> {
    if width < MIN_DOCUMENT_WIDTH || width > MAX_DOCUMENT_WIDTH {
        return Err(LimitError::InvalidDimensions(width, height));
    }
    if height < MIN_DOCUMENT_HEIGHT || height > MAX_DOCUMENT_HEIGHT {
        return Err(LimitError::InvalidDimensions(width, height));
    }
    Ok(())
}

/// Validates a name string.
pub fn validate_name(name: &str) -> Result<(), LimitError> {
    if name.is_empty() {
        return Err(LimitError::EmptyName);
    }
    if name.len() > MAX_NAME_LENGTH {
        return Err(LimitError::NameTooLong { len: name.len() });
    }
    Ok(())
}

/// Validates node count.
pub fn validate_node_count(count: usize) -> Result<(), LimitError> {
    if count > MAX_NODES {
        return Err(LimitError::TooManyNodes { count });
    }
    Ok(())
}

/// Validates group depth.
pub fn validate_group_depth(depth: usize) -> Result<(), LimitError> {
    if depth > MAX_GROUP_DEPTH {
        return Err(LimitError::GroupTooDeep { depth });
    }
    Ok(())
}

/// Limit errors.
#[derive(Debug, Error)]
pub enum LimitError {
    #[error("Invalid dimensions: {0}x{1}")]
    InvalidDimensions(u32, u32),

    #[error("Name cannot be empty")]
    EmptyName,

    #[error("Name too long: {len} bytes (max {MAX_NAME_LENGTH})")]
    NameTooLong { len: usize },

    #[error("Too many nodes: {count} (max {MAX_NODES})")]
    TooManyNodes { count: usize },

    #[error("Group too deep: {depth} (max {MAX_GROUP_DEPTH})")]
    GroupTooDeep { depth: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dimensions() {
        assert!(validate_dimensions(100, 100).is_ok());
        assert!(validate_dimensions(0, 100).is_err());
        assert!(validate_dimensions(100, 0).is_err());
        assert!(validate_dimensions(200_000, 100).is_err());
    }

    #[test]
    fn test_validate_name() {
        assert!(validate_name("Test").is_ok());
        assert!(validate_name("").is_err());
        assert!(validate_name(&"A".repeat(300)).is_err());
    }

    #[test]
    fn test_validate_node_count() {
        assert!(validate_node_count(100).is_ok());
        assert!(validate_node_count(10_001).is_err());
    }

    #[test]
    fn test_validate_group_depth() {
        assert!(validate_group_depth(10).is_ok());
        assert!(validate_group_depth(101).is_err());
    }
}
