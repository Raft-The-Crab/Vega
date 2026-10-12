//! Stable ID types for documents, layers, and nodes.
//!
//! These IDs are used throughout the core to maintain stable references
//! to document elements. They are never reused within a document session.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// A unique identifier for a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(u64);

impl DocumentId {
    /// Create a new DocumentId with the given value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying numeric value.
    pub fn value(self) -> u64 {
        self.0
    }

    /// Generate a new unique DocumentId.
    pub fn generate() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "doc-{}", self.0)
    }
}

/// A unique identifier for a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LayerId(u64);

impl LayerId {
    /// Create a new LayerId with the given value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying numeric value.
    pub fn value(self) -> u64 {
        self.0
    }

    /// Generate a new unique LayerId.
    pub fn generate() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl fmt::Display for LayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "layer-{}", self.0)
    }
}

/// A unique identifier for a node in the document graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(u64);

impl NodeId {
    /// Create a new NodeId with the given value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying numeric value.
    pub fn value(self) -> u64 {
        self.0
    }

    /// Generate a new unique NodeId.
    pub fn generate() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "node-{}", self.0)
    }
}

/// A unique identifier for a command in the history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(u64);

impl CommandId {
    /// Create a new CommandId with the given value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying numeric value.
    pub fn value(self) -> u64 {
        self.0
    }

    /// Generate a new unique CommandId.
    pub fn generate() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl fmt::Display for CommandId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmd-{}", self.0)
    }
}

#[cfg(test)]
mod tests;
