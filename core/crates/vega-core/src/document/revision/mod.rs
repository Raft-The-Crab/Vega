//! Document revision and generation tracking.
//!
//! This module provides a mechanism for tracking document revisions to ensure
//! operations work on the correct document state and to detect stale results.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// A document revision identifier.
///
/// Revisions are monotonically increasing and change whenever the document
/// is modified. This allows background operations to detect when their
/// results are stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Revision(u64);

impl Revision {
    /// Create a new revision with the given value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Get the underlying value.
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Check if this revision is before another.
    pub const fn is_before(self, other: Revision) -> bool {
        self.0 < other.0
    }

    /// Check if this revision is after another.
    pub const fn is_after(self, other: Revision) -> bool {
        self.0 > other.0
    }

    /// Check if this revision is the same as another.
    pub const fn is_same(self, other: Revision) -> bool {
        self.0 == other.0
    }
}

impl Default for Revision {
    fn default() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Revision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rev-{}", self.0)
    }
}

/// A revision counter for tracking document changes.
pub struct RevisionCounter {
    counter: AtomicU64,
}

impl RevisionCounter {
    /// Create a new revision counter starting at 0.
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }

    /// Get the current revision.
    pub fn current(&self) -> Revision {
        Revision(self.counter.load(Ordering::SeqCst))
    }

    /// Increment the revision and return the new value.
    pub fn increment(&self) -> Revision {
        Revision(self.counter.fetch_add(1, Ordering::SeqCst) + 1)
    }

    /// Reset the counter to 0.
    pub fn reset(&self) {
        self.counter.store(0, Ordering::SeqCst);
    }
}

impl Default for RevisionCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// A revision guard that ensures operations only proceed if the document
/// hasn't been modified since the guard was created.
pub struct RevisionGuard {
    expected_revision: Revision,
    counter: std::sync::Arc<RevisionCounter>,
}

impl RevisionGuard {
    /// Create a new revision guard for the current revision.
    pub fn new(counter: std::sync::Arc<RevisionCounter>) -> Self {
        Self {
            expected_revision: counter.current(),
            counter,
        }
    }

    /// Check if the document is still at the expected revision.
    pub fn is_valid(&self) -> bool {
        self.counter.current() == self.expected_revision
    }

    /// Get the expected revision.
    pub fn expected_revision(&self) -> Revision {
        self.expected_revision
    }

    /// Get the current revision.
    pub fn current_revision(&self) -> Revision {
        self.counter.current()
    }

    /// Validate that the document hasn't changed, returning an error if it has.
    pub fn validate(&self) -> Result<(), RevisionMismatchError> {
        if self.is_valid() {
            Ok(())
        } else {
            Err(RevisionMismatchError {
                expected: self.expected_revision,
                actual: self.current_revision(),
            })
        }
    }
}

/// Error returned when a revision guard detects a mismatch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevisionMismatchError {
    pub expected: Revision,
    pub actual: Revision,
}

impl std::fmt::Display for RevisionMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Revision mismatch: expected {}, found {}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for RevisionMismatchError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revision_new() {
        let rev = Revision::new(42);
        assert_eq!(rev.value(), 42);
    }

    #[test]
    fn test_revision_default() {
        let rev = Revision::default();
        assert_eq!(rev.value(), 0);
    }

    #[test]
    fn test_revision_comparisons() {
        let rev1 = Revision::new(10);
        let rev2 = Revision::new(20);
        let rev3 = Revision::new(10);

        assert!(rev1.is_before(rev2));
        assert!(rev2.is_after(rev1));
        assert!(rev1.is_same(rev3));
        assert!(!rev1.is_after(rev2));
    }

    #[test]
    fn test_revision_display() {
        let rev = Revision::new(123);
        assert_eq!(format!("{}", rev), "rev-123");
    }

    #[test]
    fn test_revision_counter_new() {
        let counter = RevisionCounter::new();
        assert_eq!(counter.current().value(), 0);
    }

    #[test]
    fn test_revision_counter_increment() {
        let counter = RevisionCounter::new();
        let rev1 = counter.increment();
        assert_eq!(rev1.value(), 1);

        let rev2 = counter.increment();
        assert_eq!(rev2.value(), 2);

        assert_eq!(counter.current().value(), 2);
    }

    #[test]
    fn test_revision_counter_reset() {
        let counter = RevisionCounter::new();
        counter.increment();
        counter.increment();
        counter.reset();
        assert_eq!(counter.current().value(), 0);
    }

    #[test]
    fn test_revision_guard_valid() {
        let counter = std::sync::Arc::new(RevisionCounter::new());
        let guard = RevisionGuard::new(counter.clone());

        assert!(guard.is_valid());
        assert!(guard.validate().is_ok());
    }

    #[test]
    fn test_revision_guard_invalid() {
        let counter = std::sync::Arc::new(RevisionCounter::new());
        let guard = RevisionGuard::new(counter.clone());

        counter.increment();

        assert!(!guard.is_valid());
        assert!(guard.validate().is_err());
    }

    #[test]
    fn test_revision_guard_expected_revision() {
        let counter = std::sync::Arc::new(RevisionCounter::new());
        let guard = RevisionGuard::new(counter.clone());

        let expected = guard.expected_revision();
        assert_eq!(expected.value(), 0);
    }

    #[test]
    fn test_revision_guard_current_revision() {
        let counter = std::sync::Arc::new(RevisionCounter::new());
        let guard = RevisionGuard::new(counter.clone());

        counter.increment();

        let current = guard.current_revision();
        assert_eq!(current.value(), 1);
    }

    #[test]
    fn test_revision_mismatch_error() {
        let error = RevisionMismatchError {
            expected: Revision::new(10),
            actual: Revision::new(20),
        };

        assert_eq!(
            format!("{}", error),
            "Revision mismatch: expected rev-10, found rev-20"
        );
    }
}
