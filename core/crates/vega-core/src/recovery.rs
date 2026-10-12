//! Error recovery and fallback mechanisms
//!
//! Provides graceful degradation and recovery strategies for common error scenarios.

use crate::errors::DocumentError;
use std::sync::Arc;

/// Recovery strategy for errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Fail fast - return error immediately.
    FailFast,
    /// Retry with exponential backoff.
    Retry,
    /// Use fallback value.
    Fallback,
    /// Skip and continue.
    Skip,
}

/// Error recovery context.
pub struct RecoveryContext {
    strategy: RecoveryStrategy,
    max_retries: usize,
}

impl RecoveryContext {
    /// Creates a new recovery context with the given strategy.
    pub fn new(strategy: RecoveryStrategy) -> Self {
        Self {
            strategy,
            max_retries: 3,
        }
    }

    /// Sets the maximum number of retries.
    pub fn with_max_retries(mut self, max: usize) -> Self {
        self.max_retries = max;
        self
    }

    /// Gets the recovery strategy.
    pub fn strategy(&self) -> RecoveryStrategy {
        self.strategy
    }

    /// Gets the maximum retries.
    pub fn max_retries(&self) -> usize {
        self.max_retries
    }
}

impl Default for RecoveryContext {
    fn default() -> Self {
        Self::new(RecoveryStrategy::FailFast)
    }
}

/// Fallback value provider.
pub trait FallbackProvider<T> {
    /// Returns a fallback value when the primary operation fails.
    fn fallback(&self) -> T;
}

/// Attempts an operation with error recovery.
pub fn try_with_recovery<T, F>(
    operation: F,
    context: &RecoveryContext,
) -> Result<T, DocumentError>
where
    F: Fn() -> Result<T, DocumentError>,
{
    match context.strategy() {
        RecoveryStrategy::FailFast => operation(),
        RecoveryStrategy::Retry => {
            let mut last_error = None;
            for attempt in 0..=context.max_retries() {
                match operation() {
                    Ok(value) => return Ok(value),
                    Err(e) => {
                        last_error = Some(e);
                        if attempt < context.max_retries() {
                            // Simple exponential backoff
                            std::thread::sleep(std::time::Duration::from_millis(
                                10 * 2_u64.pow(attempt as u32),
                            ));
                        }
                    }
                }
            }
            Err(last_error.unwrap_or(DocumentError::InvalidParameter(
                "Operation failed".to_string(),
            )))
        }
        RecoveryStrategy::Skip => {
            // For Skip strategy, we return a different error type or skip entirely
            // This is context-dependent, so we just fail fast for now
            operation()
        }
        RecoveryStrategy::Fallback => {
            // Fallback requires a provider, so we fall back to the operation
            operation()
        }
    }
}

/// Attempts an operation with a fallback value.
pub fn try_with_fallback<T, F, P>(
    operation: F,
    fallback_provider: &P,
) -> T
where
    F: Fn() -> Result<T, DocumentError>,
    P: FallbackProvider<T>,
{
    match operation() {
        Ok(value) => value,
        Err(_) => fallback_provider.fallback(),
    }
}

/// Safe accessor that returns a default on error.
pub fn safe_get<T, F>(operation: F, default: T) -> T
where
    F: FnOnce() -> Result<T, DocumentError>,
{
    operation().unwrap_or(default)
}

/// Safe accessor that returns None on error.
pub fn safe_get_opt<T, F>(operation: F) -> Option<T>
where
    F: FnOnce() -> Result<T, DocumentError>,
{
    operation().ok()
}

/// Memory pressure monitor for graceful degradation.
pub struct MemoryPressureMonitor {
    threshold: f32,
    callback: Arc<dyn Fn(f32) + Send + Sync>,
}

impl MemoryPressureMonitor {
    /// Creates a new memory pressure monitor.
    pub fn new(threshold: f32, callback: Arc<dyn Fn(f32) + Send + Sync>) -> Self {
        Self { threshold, callback }
    }

    /// Checks if memory pressure exceeds the threshold.
    pub fn check_pressure(&self, current_usage: f32) -> bool {
        if current_usage > self.threshold {
            (self.callback)(current_usage);
            true
        } else {
            false
        }
    }

    /// Gets the pressure threshold.
    pub fn threshold(&self) -> f32 {
        self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fail_fast_strategy() {
        let context = RecoveryContext::new(RecoveryStrategy::FailFast);
        let result = try_with_recovery(
            || Err(DocumentError::InvalidParameter("Test error".to_string())),
            &context,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_retry_strategy_success() {
        let context = RecoveryContext::new(RecoveryStrategy::Retry).with_max_retries(2);
        let mut attempts = 0;
        let result = try_with_recovery(
            || {
                attempts += 1;
                if attempts < 2 {
                    Err(DocumentError::InvalidParameter("Test error".to_string()))
                } else {
                    Ok(42)
                }
            },
            &context,
        );
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 2);
    }

    #[test]
    fn test_safe_get() {
        let result = safe_get(|| Ok(42), 0);
        assert_eq!(result, 42);

        let result = safe_get(|| Err(DocumentError::InvalidParameter("Test".to_string())), 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_safe_get_opt() {
        let result = safe_get_opt(|| Ok(42));
        assert_eq!(result, Some(42));

        let result = safe_get_opt(|| Err(DocumentError::InvalidParameter("Test".to_string())));
        assert_eq!(result, None);
    }
}
