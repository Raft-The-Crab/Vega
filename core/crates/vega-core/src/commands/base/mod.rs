//! Base command types and execution.
//!
//! Provides the command abstraction for implementing undoable operations
//! on documents. All modifications to documents should go through commands
//! to ensure proper history tracking and undo/redo support.

use crate::document::ids::structure::CommandId;
use crate::document::Document;
use crate::errors::{DocumentError, Result};
use crate::tiles::TileStorage;
use crate::masks::LayerMaskStorage;
use crate::selections::SelectionStorage;
use std::fmt;

/// Context for command execution.
///
/// Provides access to external resources needed for command execution,
/// such as tile storage, layer mask storage, and selection storage.
#[derive(Debug)]
pub struct CommandContext {
    /// Tile storage for pixel operations.
    pub tile_storage: Option<TileStorage>,
    /// Layer mask storage for mask operations.
    pub mask_storage: Option<LayerMaskStorage>,
    /// Selection storage for selection operations.
    pub selection_storage: Option<SelectionStorage>,
}

impl CommandContext {
    /// Creates a new command context.
    pub fn new() -> Self {
        Self {
            tile_storage: None,
            mask_storage: None,
            selection_storage: None,
        }
    }

    /// Creates a command context with tile storage.
    pub fn with_tile_storage(tile_storage: TileStorage) -> Self {
        Self {
            tile_storage: Some(tile_storage),
            mask_storage: None,
            selection_storage: None,
        }
    }

    /// Creates a command context with mask storage.
    pub fn with_mask_storage(mask_storage: LayerMaskStorage) -> Self {
        Self {
            tile_storage: None,
            mask_storage: Some(mask_storage),
            selection_storage: None,
        }
    }

    /// Creates a command context with selection storage.
    pub fn with_selection_storage(selection_storage: SelectionStorage) -> Self {
        Self {
            tile_storage: None,
            mask_storage: None,
            selection_storage: Some(selection_storage),
        }
    }

    /// Creates a command context with tile and mask storage.
    pub fn with_tile_and_mask_storage(tile_storage: TileStorage, mask_storage: LayerMaskStorage) -> Self {
        Self {
            tile_storage: Some(tile_storage),
            mask_storage: Some(mask_storage),
            selection_storage: None,
        }
    }

    /// Creates a command context with all storages.
    pub fn with_all_storages(
        tile_storage: TileStorage,
        mask_storage: LayerMaskStorage,
        selection_storage: SelectionStorage,
    ) -> Self {
        Self {
            tile_storage: Some(tile_storage),
            mask_storage: Some(mask_storage),
            selection_storage: Some(selection_storage),
        }
    }
}

impl Default for CommandContext {
    fn default() -> Self {
        Self::new()
    }
}

/// A command that can be applied to a document.
///
/// Commands encapsulate a single undoable operation on a document. Implementations
/// must provide execute, undo, and redo methods to support the full command lifecycle.
///
/// # Thread Safety
/// Commands must be Send + Sync to allow for concurrent command execution scenarios.
///
/// # Examples
///
/// ```rust,no_run
/// use vega_core::{Command, Document, errors::Result, commands::base::CommandContext};
///
/// struct MyCommand {
///     // Command state
/// }
///
/// impl Command for MyCommand {
///     fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
///         // Apply changes
///         Ok(())
///     }
///
///     fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
///         // Revert changes
///         Ok(())
///     }
///
///     fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
///         self.execute(document, context)
///     }
///
///     fn description(&self) -> String {
///         "My operation".to_string()
///     }
/// }
/// ```
pub trait Command: fmt::Debug + Send + Sync {
    /// Execute the command on the document.
    ///
    /// This method should apply the command's changes to the document and
    /// return an error if the operation cannot be completed.
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()>;

    /// Undo the command, reverting the document to its previous state.
    ///
    /// This method should restore the document to the state it was in before
    /// execute() was called.
    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()>;

    /// Redo the command, reapplying it after an undo.
    ///
    /// The default implementation simply calls execute(), but commands can
    /// override this for optimization.
    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
        self.execute(document, context)
    }

    /// Get a description of what this command does.
    ///
    /// This is used for UI display and logging.
    fn description(&self) -> String;

    /// Get the command ID.
    fn id(&self) -> CommandId;

    /// Check if this command can be undone.
    fn can_undo(&self) -> bool {
        true
    }

    /// Check if this command is destructive (modifies pixels directly).
    fn is_destructive(&self) -> bool {
        false
    }
}

/// A transaction for grouping multiple commands together.
#[derive(Debug)]
pub struct Transaction {
    id: CommandId,
    name: String,
    commands: Vec<Box<dyn Command>>,
}

impl Transaction {
    /// Create a new transaction with a name.
    pub fn new(name: String) -> Self {
        Self {
            id: CommandId::generate(),
            name,
            commands: Vec::new(),
        }
    }

    /// Add a command to the transaction.
    pub fn add(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    /// Get the number of commands in the transaction.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Check if the transaction is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl Command for Transaction {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
        for command in &mut self.commands {
            command.execute(document, context)?;
        }
        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
        // Undo in reverse order
        for command in self.commands.iter_mut().rev() {
            command.undo(document, context)?;
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
        for command in &mut self.commands {
            command.redo(document, context)?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        self.name.clone()
    }

    fn id(&self) -> CommandId {
        self.id
    }

    fn can_undo(&self) -> bool {
        self.commands.iter().all(|c| c.can_undo())
    }

    fn is_destructive(&self) -> bool {
        self.commands.iter().any(|c| c.is_destructive())
    }
}

/// Executor for commands on a document.
///
/// The CommandExecutor manages command history and provides undo/redo functionality.
/// It maintains a bounded history of executed commands and tracks the current position
/// for undo/redo operations.
///
/// # Examples
///
/// ```rust,no_run
/// use vega_core::{CommandExecutor, Document, commands::Command};
///
/// let mut executor = CommandExecutor::new(100);
/// let mut document = Document::default();
///
/// // Execute a command
/// executor.execute(&mut document, Box::new(my_command))?;
///
/// // Undo
/// executor.undo(&mut document)?;
///
/// // Redo
/// executor.redo(&mut document)?;
/// ```
pub struct CommandExecutor {
    /// History of executed commands.
    history: Vec<Box<dyn Command>>,

    /// Current position in the history (for undo/redo).
    history_position: usize,

    /// Maximum number of commands to keep in history.
    max_history: usize,
}

impl CommandExecutor {
    /// Create a new command executor.
    pub fn new(max_history: usize) -> Self {
        Self {
            history: Vec::new(),
            history_position: 0,
            max_history,
        }
    }

    /// Execute a command and add it to history.
    pub fn execute(&mut self, document: &mut Document, mut command: Box<dyn Command>, context: &mut CommandContext) -> Result<()> {
        // Execute the command
        command.execute(document, context)?;

        // Remove any commands after the current position (they will be redone)
        if self.history_position < self.history.len() {
            self.history.truncate(self.history_position);
        }

        // Add to history
        self.history.push(command);
        self.history_position = self.history.len();

        // Enforce max history size
        if self.history.len() > self.max_history {
            let remove_count = self.history.len() - self.max_history;
            for _ in 0..remove_count {
                self.history.remove(0);
            }
            self.history_position = self.history.len();
        }

        Ok(())
    }

    /// Undo the last command.
    pub fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
        if self.history_position == 0 {
            return Err(DocumentError::InvalidState("Nothing to undo".to_string()).into());
        }

        self.history_position -= 1;
        let command = &mut self.history[self.history_position];
        command.undo(document, context)?;

        Ok(())
    }

    /// Redo the next command.
    pub fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<()> {
        if self.history_position >= self.history.len() {
            return Err(DocumentError::InvalidState("Nothing to redo".to_string()).into());
        }

        let command = &mut self.history[self.history_position];
        command.redo(document, context)?;
        self.history_position += 1;

        Ok(())
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        self.history_position > 0
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        self.history_position < self.history.len()
    }

    /// Get the number of commands in history.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Clear the command history.
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.history_position = 0;
    }

    /// Get the description of the command that would be undone.
    pub fn undo_description(&self) -> Option<String> {
        if self.history_position > 0 {
            Some(self.history[self.history_position - 1].description())
        } else {
            None
        }
    }

    /// Get the description of the command that would be redone.
    pub fn redo_description(&self) -> Option<String> {
        if self.history_position < self.history.len() {
            Some(self.history[self.history_position].description())
        } else {
            None
        }
    }
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new(100) // Default to 100 commands in history
    }
}

#[cfg(test)]
mod tests;
