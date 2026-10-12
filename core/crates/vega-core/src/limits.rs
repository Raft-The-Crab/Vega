//! Resource limits and configuration constants.
//!
//! This module defines safe limits for document resources to prevent
//! memory exhaustion and ensure reasonable performance.

/// Maximum document dimensions in pixels.
pub const MAX_DOCUMENT_WIDTH: u32 = 100_000;
pub const MAX_DOCUMENT_HEIGHT: u32 = 100_000;

/// Minimum document dimensions in pixels.
pub const MIN_DOCUMENT_WIDTH: u32 = 1;
pub const MIN_DOCUMENT_HEIGHT: u32 = 1;

/// Maximum number of nodes in a document.
pub const MAX_NODES: usize = 10_000;

/// Maximum nesting depth for layer groups.
pub const MAX_GROUP_DEPTH: usize = 100;

/// Maximum length for node names.
pub const MAX_NAME_LENGTH: usize = 256;

/// Default tile size for pixel storage (pixels).
pub const DEFAULT_TILE_SIZE: u32 = 256;

/// Maximum tile size for pixel storage (pixels).
pub const MAX_TILE_SIZE: u32 = 4096;

/// Minimum tile size for pixel storage (pixels).
pub const MIN_TILE_SIZE: u32 = 64;

/// Maximum command history size.
pub const DEFAULT_HISTORY_SIZE: usize = 100;

/// Maximum command history size.
pub const MAX_HISTORY_SIZE: usize = 1000;

/// Default document dimensions.
pub const DEFAULT_DOCUMENT_WIDTH: u32 = 1920;
pub const DEFAULT_DOCUMENT_HEIGHT: u32 = 1080;

/// Maximum size for custom metadata values (bytes).
pub const MAX_METADATA_VALUE_SIZE: usize = 1024;

/// Maximum number of custom metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 100;

/// Validates document dimensions are within acceptable limits.
pub fn validate_dimensions(width: u32, height: u32) -> Result<(), LimitError> {
    if width < MIN_DOCUMENT_WIDTH || height < MIN_DOCUMENT_HEIGHT {
        return Err(LimitError::DimensionTooSmall {
            width,
            height,
            min_width: MIN_DOCUMENT_WIDTH,
            min_height: MIN_DOCUMENT_HEIGHT,
        });
    }

    if width > MAX_DOCUMENT_WIDTH || height > MAX_DOCUMENT_HEIGHT {
        return Err(LimitError::DimensionTooLarge {
            width,
            height,
            max_width: MAX_DOCUMENT_WIDTH,
            max_height: MAX_DOCUMENT_HEIGHT,
        });
    }

    // Check for potential overflow in area calculation
    let area = (width as u64) * (height as u64);
    if area > (MAX_DOCUMENT_WIDTH as u64) * (MAX_DOCUMENT_HEIGHT as u64) {
        return Err(LimitError::AreaTooLarge { area });
    }

    Ok(())
}

/// Validates a node name is within acceptable limits.
pub fn validate_name(name: &str) -> Result<(), LimitError> {
    if name.is_empty() {
        return Err(LimitError::NameEmpty);
    }

    if name.len() > MAX_NAME_LENGTH {
        return Err(LimitError::NameTooLong {
            length: name.len(),
            max: MAX_NAME_LENGTH,
        });
    }

    Ok(())
}

/// Validates node count is within limits.
pub fn validate_node_count(count: usize) -> Result<(), LimitError> {
    if count > MAX_NODES {
        return Err(LimitError::TooManyNodes {
            count,
            max: MAX_NODES,
        });
    }
    Ok(())
}

/// Validates group nesting depth.
pub fn validate_group_depth(depth: usize) -> Result<(), LimitError> {
    if depth > MAX_GROUP_DEPTH {
        return Err(LimitError::GroupDepthTooLarge {
            depth,
            max: MAX_GROUP_DEPTH,
        });
    }
    Ok(())
}

/// Errors related to resource limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimitError {
    /// Document dimensions are too small.
    DimensionTooSmall {
        width: u32,
        height: u32,
        min_width: u32,
        min_height: u32,
    },

    /// Document dimensions are too large.
    DimensionTooLarge {
        width: u32,
        height: u32,
        max_width: u32,
        max_height: u32,
    },

    /// Document area is too large (potential overflow).
    AreaTooLarge { area: u64 },

    /// Node name is empty.
    NameEmpty,

    /// Node name is too long.
    NameTooLong { length: usize, max: usize },

    /// Too many nodes in document.
    TooManyNodes { count: usize, max: usize },

    /// Group nesting is too deep.
    GroupDepthTooLarge { depth: usize, max: usize },

    /// Metadata value is too large.
    MetadataTooLarge { size: usize, max: usize },

    /// Too many metadata entries.
    TooManyMetadataEntries { count: usize, max: usize },
}

impl std::fmt::Display for LimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LimitError::DimensionTooSmall {
                width,
                height,
                min_width,
                min_height,
            } => write!(
                f,
                "Dimensions {}x{} are too small (minimum: {}x{})",
                width, height, min_width, min_height
            ),
            LimitError::DimensionTooLarge {
                width,
                height,
                max_width,
                max_height,
            } => write!(
                f,
                "Dimensions {}x{} are too large (maximum: {}x{})",
                width, height, max_width, max_height
            ),
            LimitError::AreaTooLarge { area } => {
                write!(f, "Document area {} exceeds maximum allowed", area)
            }
            LimitError::NameEmpty => write!(f, "Name cannot be empty"),
            LimitError::NameTooLong { length, max } => {
                write!(f, "Name length {} exceeds maximum {}", length, max)
            }
            LimitError::TooManyNodes { count, max } => {
                write!(f, "Node count {} exceeds maximum {}", count, max)
            }
            LimitError::GroupDepthTooLarge { depth, max } => {
                write!(f, "Group depth {} exceeds maximum {}", depth, max)
            }
            LimitError::MetadataTooLarge { size, max } => {
                write!(f, "Metadata size {} exceeds maximum {}", size, max)
            }
            LimitError::TooManyMetadataEntries { count, max } => {
                write!(f, "Metadata entry count {} exceeds maximum {}", count, max)
            }
        }
    }
}

impl std::error::Error for LimitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dimensions_valid() {
        assert!(validate_dimensions(1920, 1080).is_ok());
        assert!(validate_dimensions(100, 100).is_ok());
    }

    #[test]
    fn test_validate_dimensions_too_small() {
        let result = validate_dimensions(0, 100);
        assert!(matches!(result, Err(LimitError::DimensionTooSmall { .. })));
    }

    #[test]
    fn test_validate_dimensions_too_large() {
        let result = validate_dimensions(200_000, 100);
        assert!(matches!(result, Err(LimitError::DimensionTooLarge { .. })));
    }

    #[test]
    fn test_validate_name_valid() {
        assert!(validate_name("Layer 1").is_ok());
        assert!(validate_name("A").is_ok());
    }

    #[test]
    fn test_validate_name_empty() {
        let result = validate_name("");
        assert!(matches!(result, Err(LimitError::NameEmpty)));
    }

    #[test]
    fn test_validate_name_too_long() {
        let long_name = "a".repeat(300);
        let result = validate_name(&long_name);
        assert!(matches!(result, Err(LimitError::NameTooLong { .. })));
    }

    #[test]
    fn test_validate_node_count_valid() {
        assert!(validate_node_count(100).is_ok());
        assert!(validate_node_count(MAX_NODES).is_ok());
    }

    #[test]
    fn test_validate_node_count_too_many() {
        let result = validate_node_count(MAX_NODES + 1);
        assert!(matches!(result, Err(LimitError::TooManyNodes { .. })));
    }

    #[test]
    fn test_validate_group_depth_valid() {
        assert!(validate_group_depth(10).is_ok());
        assert!(validate_group_depth(MAX_GROUP_DEPTH).is_ok());
    }

    #[test]
    fn test_validate_group_depth_too_large() {
        let result = validate_group_depth(MAX_GROUP_DEPTH + 1);
        assert!(matches!(result, Err(LimitError::GroupDepthTooLarge { .. })));
    }

    #[test]
    fn test_limit_error_display() {
        let err = LimitError::NameEmpty;
        assert_eq!(format!("{}", err), "Name cannot be empty");

        let err = LimitError::DimensionTooLarge {
            width: 200_000,
            height: 100,
            max_width: MAX_DOCUMENT_WIDTH,
            max_height: MAX_DOCUMENT_HEIGHT,
        };
        assert!(format!("{}", err).contains("too large"));
    }
}
