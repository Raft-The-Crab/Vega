//! Logging and tracing integration
//!
//! Provides structured logging and tracing instrumentation for the VEGA core.

use tracing::{info, warn, error, debug, instrument, span};

/// Initializes the tracing subscriber for the application.
///
/// This should be called once at application startup.
pub fn init_tracing() {
    // Use environment variables to configure tracing
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .with_thread_ids(true)
        .with_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
    info!("VEGA Core tracing initialized");
}

/// Logs document creation with instrumentation.
pub fn log_document_creation(document_id: u64, name: &str, size: (u32, u32)) {
    info!(
        document_id,
        name,
        width = size.0,
        height = size.1,
        "Document created"
    );
}

/// Logs command execution with instrumentation.
pub fn log_command_execution(command_id: u64, command_name: &str, document_id: u64) {
    info!(
        command_id,
        command_name,
        document_id,
        "Command executed"
    );
}

/// Logs tile allocation with instrumentation.
#[instrument]
pub fn log_tile_allocation(tile_index: u64, coords: (u32, u32), size: u32) {
    debug!(
        tile_index,
        tile_x = coords.0,
        tile_y = coords.1,
        size,
        "Tile allocated"
    );
}

/// Logs rendering operation with instrumentation.
pub fn log_render_operation(document_id: u64, viewport: (u32, u32, u32, u32)) {
    info!(
        document_id,
        viewport_x = viewport.0,
        viewport_y = viewport.1,
        viewport_width = viewport.2,
        viewport_height = viewport.3,
        "Render operation started"
    );
}

/// Logs cache hit/miss events.
pub fn log_cache_event(hit: bool, tile_index: u64) {
    if hit {
        debug!(tile_index, "Cache hit");
    } else {
        debug!(tile_index, "Cache miss");
    }
}

/// Logs memory usage.
pub fn log_memory_usage(current: usize, max: usize, percentage: f32) {
    info!(
        current_bytes = current,
        max_bytes = max,
        usage_percent = percentage,
        "Memory usage"
    );
}

/// Logs error events with context.
pub fn log_error(context: &str, error: &str) {
    error!(context, error, "Error occurred");
}

/// Logs performance metrics.
pub fn log_performance(operation: &str, duration_us: u64, tiles: u64) {
    info!(
        operation,
        duration_us,
        tiles,
        avg_time_per_tile_us = duration_us as f32 / tiles as f32,
        "Performance metrics"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_init() {
        // This test just ensures the function compiles
        // Actual logging is tested in integration tests
        init_tracing();
    }

    #[test]
    fn test_log_document_creation() {
        log_document_creation(123, "Test", (1920, 1080));
    }

    #[test]
    fn test_log_cache_event() {
        log_cache_event(true, 456);
        log_cache_event(false, 456);
    }
}
