//! Async patterns for future scalability
//!
//! Provides async/await foundations for potential future async operations.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A simple async task executor for future use.
pub struct AsyncExecutor {
    tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send>>>,
}

impl AsyncExecutor {
    /// Creates a new async executor.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Spawns a new task.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push(Box::pin(future));
    }

    /// Polls all tasks once.
    pub fn poll_once(&mut self, cx: &mut Context<'_>) {
        for task in &mut self.tasks {
            let _ = task.as_mut().poll(cx);
        }
    }

    /// Returns the number of pending tasks.
    pub fn pending_count(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for AsyncExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Async result wrapper for fallible operations.
pub type AsyncResult<T> = Result<T, AsyncError>;

/// Async operation errors.
#[derive(Debug, thiserror::Error)]
pub enum AsyncError {
    #[error("Operation timed out")]
    Timeout,

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Operation failed: {0}")]
    Failed(String),
}

/// Cancellation token for async operations.
#[derive(Clone)]
pub struct CancellationToken {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl CancellationToken {
    /// Creates a new cancellation token.
    pub fn new() -> Self {
        Self {
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Cancels the operation.
    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Checks if the operation is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Creates a child token that inherits cancellation state.
    pub fn child(&self) -> Self {
        Self {
            cancelled: self.cancelled.clone(),
        }
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

/// Async wrapper for sync operations.
pub async fn async_wrap<F, T>(f: F) -> T
where
    F: FnOnce() -> T + Send,
    T: Send,
{
    f()
}

/// Simple async sleep for testing.
pub async fn async_sleep(duration: std::time::Duration) {
    #[cfg(feature = "async")]
    {
        tokio::time::sleep(duration).await;
    }

    #[cfg(not(feature = "async"))]
    {
        std::thread::sleep(duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());

        token.cancel();
        assert!(token.is_cancelled());

        let child = token.child();
        assert!(child.is_cancelled());
    }

    #[test]
    fn test_async_executor() {
        let mut executor = AsyncExecutor::new();
        assert_eq!(executor.pending_count(), 0);

        // Would spawn tasks in a real async implementation
        // For now, just test the structure
    }

    #[test]
    fn test_async_wrap() {
        let result = async_wrap(|| 42);
        assert_eq!(result, 42);
    }
}
