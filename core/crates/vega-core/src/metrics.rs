//! Metrics and performance tracking
//!
//! Provides instrumentation for monitoring rendering performance and resource usage.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Performance metrics for rendering operations.
#[derive(Debug, Clone)]
pub struct RenderMetrics {
    /// Number of tiles rendered.
    pub tiles_rendered: u64,
    /// Number of cache hits.
    pub cache_hits: u64,
    /// Number of cache misses.
    pub cache_misses: u64,
    /// Total render time in microseconds.
    pub total_render_time_us: u64,
    /// Total pixels processed.
    pub total_pixels: u64,
}

impl RenderMetrics {
    /// Creates new zero metrics.
    pub fn new() -> Self {
        Self {
            tiles_rendered: 0,
            cache_hits: 0,
            cache_misses: 0,
            total_render_time_us: 0,
            total_pixels: 0,
        }
    }

    /// Records a tile render.
    pub fn record_tile_render(&mut self) {
        self.tiles_rendered += 1;
    }

    /// Records a cache hit.
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }

    /// Records a cache miss.
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }

    /// Records render time.
    pub fn record_render_time(&mut self, duration: Duration) {
        self.total_render_time_us += duration.as_micros() as u64;
    }

    /// Records pixels processed.
    pub fn record_pixels(&mut self, count: u64) {
        self.total_pixels += count;
    }

    /// Returns the cache hit rate (0.0 to 1.0).
    pub fn cache_hit_rate(&self) -> f32 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f32 / total as f32
        }
    }

    /// Returns the average render time per tile in microseconds.
    pub fn avg_render_time_per_tile_us(&self) -> f32 {
        if self.tiles_rendered == 0 {
            0.0
        } else {
            self.total_render_time_us as f32 / self.tiles_rendered as f32
        }
    }

    /// Returns the pixels per second (throughput).
    pub fn pixels_per_second(&self) -> f32 {
        if self.total_render_time_us == 0 {
            0.0
        } else {
            (self.total_pixels as f32 * 1_000_000.0) / self.total_render_time_us as f32
        }
    }

    /// Resets all metrics.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for RenderMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Global metrics tracker.
pub struct MetricsTracker {
    metrics: RenderMetrics,
    render_count: AtomicU64,
}

impl MetricsTracker {
    /// Creates a new metrics tracker.
    pub fn new() -> Self {
        Self {
            metrics: RenderMetrics::new(),
            render_count: AtomicU64::new(0),
        }
    }

    /// Gets a snapshot of current metrics.
    pub fn snapshot(&self) -> RenderMetrics {
        self.metrics.clone()
    }

    /// Increments the global render count.
    pub fn increment_render_count(&self) {
        self.render_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Gets the total number of renders.
    pub fn total_renders(&self) -> u64 {
        self.render_count.load(Ordering::SeqCst)
    }

    /// Updates metrics from a render operation.
    pub fn update_from_render(&mut self, metrics: RenderMetrics) {
        self.metrics.tiles_rendered += metrics.tiles_rendered;
        self.metrics.cache_hits += metrics.cache_hits;
        self.metrics.cache_misses += metrics.cache_misses;
        self.metrics.total_render_time_us += metrics.total_render_time_us;
        self.metrics.total_pixels += metrics.total_pixels;
    }

    /// Resets all metrics.
    pub fn reset(&mut self) {
        self.metrics.reset();
        self.render_count.store(0, Ordering::SeqCst);
    }
}

impl Default for MetricsTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer for measuring render operations.
pub struct RenderTimer {
    start: Instant,
}

impl RenderTimer {
    /// Starts a new timer.
    pub fn start() -> Self {
        Self { start: Instant::now() }
    }

    /// Stops the timer and returns the elapsed duration.
    pub fn stop(self) -> Duration {
        self.start.elapsed()
    }
}

impl Default for RenderTimer {
    fn default() -> Self {
        Self::start()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_metrics() {
        let mut metrics = RenderMetrics::new();
        metrics.record_tile_render();
        metrics.record_cache_hit();
        metrics.record_pixels(1000);

        assert_eq!(metrics.tiles_rendered, 1);
        assert_eq!(metrics.cache_hits, 1);
        assert_eq!(metrics.total_pixels, 1000);
        assert_eq!(metrics.cache_hit_rate(), 1.0);
    }

    #[test]
    fn test_render_timer() {
        let timer = RenderTimer::start();
        std::thread::sleep(Duration::from_millis(10));
        let duration = timer.stop();
        assert!(duration.as_millis() >= 10);
    }

    #[test]
    fn test_metrics_tracker() {
        let tracker = MetricsTracker::new();
        tracker.increment_render_count();
        tracker.increment_render_count();

        assert_eq!(tracker.total_renders(), 2);
    }
}
