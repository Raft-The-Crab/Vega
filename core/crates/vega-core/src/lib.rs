//! # VEGA Core
//!
//! The authoritative document model, editing operations, and history for the VEGA image editor.
//!
//! ## Architecture
//!
//! VEGA Core is a Rust library that provides:
//!
//! - **Document Model**: Authoritative document graph with nodes, layers, and metadata
//! - **Command System**: Undoable operations with full history support
//! - **Tile Storage**: Efficient memory management for large images using tile-based storage
//! - **CPU Rendering**: Reference rendering pipeline with blend modes and compositing
//! - **Layer Operations**: Create, delete, move, transform, and manipulate layers
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use vega_core::{
//!     Document, Size, ColorSpace,
//!     commands::CommandExecutor,
//!     layers::CreateLayerCommand,
//!     document::nodes::NodeType,
//! };
//!
//! // Create a new document
//! let mut document = Document::new(
//!     "My Image".to_string(),
//!     Size::new(1920, 1080),
//!     ColorSpace::default(),
//! );
//!
//! // Set up command executor for undo/redo
//! let mut executor = CommandExecutor::new(100);
//!
//! // Add a layer
//! let mut cmd = CreateLayerCommand::new(None, "Background".to_string(), NodeType::Raster);
//! executor.execute(&mut document, &mut cmd).unwrap();
//!
//! // Undo the operation
//! executor.undo(&mut document).unwrap();
//! ```
//!
//! ## Modules
//!
//! - [`color`]: Color types and color space definitions
//! - [`commands`]: Command pattern for undoable operations
//! - [`document`]: Document model and node graph
//! - [`errors`]: Error types and handling
//! - [`geometry`]: 2D geometry primitives
//! - [`layers`]: Layer manipulation commands
//! - [`limits`]: Resource limits and validation
//! - [`render`]: CPU rendering pipeline
//! - [`tiles`]: Tile-based pixel storage

pub mod color;
pub mod commands;
pub mod document;
pub mod errors;
pub mod geometry;
pub mod integration_tests;
pub mod layers;
pub mod limits;
pub mod metrics;
pub mod render;
pub mod tiles;

pub use color::{ColorSpace, Rgba, Rgba8, Rgba16, Rgba32};
pub use commands::{Command, CommandExecutor, Transaction};
pub use document::{
    AdjustmentType, BlendMode, CommandId, Document, DocumentId, DocumentMetadata, LayerId, MaskData,
    Node, NodeId, NodeProperties, NodeType, Revision, RevisionCounter, RevisionGuard,
    RevisionMismatchError, ShapeType, Transform,
};
pub use errors::{VegaError, Result};
pub use geometry::{Point, Rect, Size};
pub use layers::{
    CreateLayerCommand, DeleteLayerCommand, DuplicateLayerCommand, MergeDownCommand,
    MoveLayerCommand, ReorderLayerCommand, TransformLayerCommand,
};
pub use limits::{LimitError, MAX_DOCUMENT_HEIGHT, MAX_DOCUMENT_WIDTH, MAX_NODES};
pub use metrics::{MetricsTracker, RenderMetrics, RenderTimer};
pub use render::{blend_pixels, BlendOp, CacheEntry, Compositor, Hsl, RenderCache, ViewportRenderer,
            apply_adjustment, blend_color, blend_hue, blend_luminosity, blend_saturation, luminance,
            inverse_transform_point, is_identity_transform, render_tiles_parallel, render_tiles_sequential,
            transform_point, transform_rect, transformed_bounds};
pub use tiles::{EvictionPolicy, MemoryStats, Tile, TileCoords, TileHandle, TileIndex, TileMemoryManager, TileStorage};
