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

pub mod r#async;
pub mod color;
pub mod commands;
pub mod config;
pub mod document;
pub mod errors;
pub mod brush;
pub mod export;
pub mod geometry;
pub mod masks;
pub mod selections;
pub mod image_ops;
pub mod integration_tests;
pub mod layers;
pub mod limits;
pub mod logging;
pub mod metrics;
pub mod recovery;
pub mod render;
pub mod security;
pub mod tiles;

pub use color::{ColorSpace, Rgba, Rgba8, Rgba16, Rgba32};
pub use commands::{Command, CommandExecutor, Transaction};
pub use commands::base::CommandContext;
pub use config::{CacheConfig, ConfigError, MemoryConfig, RenderConfig, TileConfig, VegaConfig};
pub use document::{
    AdjustmentType, BlendMode, CommandId, Document, DocumentId, DocumentMetadata, LayerId, MaskData,
    Node, NodeId, NodeProperties, NodeType, Revision, RevisionCounter, RevisionGuard,
    RevisionMismatchError, ShapeType, Transform,
};
pub use errors::{VegaError, Result};
pub use brush::{paint_brush, paint_eraser, BrushCommand, BrushParams, EraserCommand, EraserParams};
pub use export::{export_document, export_to_jpeg, export_to_png, export_to_webp, export_with_params, ExportFormat, ExportParams, ExportQuality};
pub use geometry::{Point, Rect, Size};
pub use masks::{add_layer_mask, apply_selection_as_mask, paint_on_mask, remove_layer_mask, AddLayerMaskCommand, ApplySelectionAsMaskCommand, LayerMask, LayerMaskStorage, RemoveLayerMaskCommand};
pub use selections::{create_elliptical_selection, create_lasso_selection, create_magic_wand_selection, create_rectangular_selection, color_distance, EllipticalSelectionParams, LassoSelectionParams, MagicWandParams, RectangularSelectionCommand, RectangularSelectionParams, SelectionMask, SelectionMode, SelectionShape, SelectionStorage};
pub use image_ops::{calculate_aspect_ratio_size, crop_layer, flip_layer_horizontal, flip_layer_vertical, resize_layer, rotate_layer_ccw, rotate_layer_cw, CropLayerCommand, CropParams, ResizeLayerCommand, ResizeParams, ResamplingMethod, RotateLayerCwCommand, RotateLayerCcwCommand, FlipLayerHorizontalCommand, FlipLayerVerticalCommand};
pub use layers::{
    CreateLayerCommand, DeleteLayerCommand, DuplicateLayerCommand, MergeDownCommand,
    MoveLayerCommand, ReorderLayerCommand, TransformLayerCommand,
};
pub use limits::{LimitError, MAX_DOCUMENT_HEIGHT, MAX_DOCUMENT_WIDTH, MAX_NODES};
pub use logging::{init_tracing, log_cache_event, log_command_execution, log_document_creation, log_error, log_memory_usage, log_performance, log_render_operation, log_tile_allocation};
pub use metrics::{MetricsTracker, RenderMetrics, RenderTimer};
pub use recovery::{FallbackProvider, MemoryPressureMonitor, RecoveryContext, RecoveryStrategy, safe_get, safe_get_opt, try_with_fallback, try_with_recovery};
pub use render::{blend_pixels, BlendOp, CacheEntry, Compositor, Hsl, RenderCache, ViewportRenderer,
            apply_adjustment, blend_color, blend_hue, blend_luminosity, blend_saturation, luminance,
            inverse_transform_point, is_identity_transform, render_tiles_parallel, render_tiles_sequential,
            transform_point, transform_rect, transformed_bounds};
pub use security::{BoundsChecker, RateLimiter, TimeoutHandler, is_safe_path, sanitize_string, truncate_string, validate_size};
pub use tiles::{EvictionPolicy, MemoryStats, Tile, TileCoords, TileHandle, TileIndex, TileMemoryManager, TileStorage};
