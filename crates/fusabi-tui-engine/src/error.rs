//! Comprehensive error types for the Fusabi TUI engine.

use fusabi_tui_render::error::RenderError;
use std::path::PathBuf;
use thiserror::Error;

/// Main error type for the dashboard engine.
#[derive(Debug, Error)]
pub enum EngineError {
    /// File loading failed.
    #[error("Failed to load file: {0}")]
    LoadError(#[from] LoadError),

    /// File watching failed.
    #[error("Failed to watch file: {0}")]
    WatchError(#[from] WatchError),

    /// Underlying render error.
    #[error("Render error: {0}")]
    Render(#[from] RenderError),

    /// Engine observed an invalid state transition.
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Requested widget was not registered.
    #[error("Widget not found: {0}")]
    WidgetNotFound(String),

    /// Standard I/O error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Fallback for caller-defined error strings.
    #[error("Custom error: {0}")]
    Custom(String),
}

/// Error type for file loading operations.
#[derive(Debug, Error)]
pub enum LoadError {
    /// Requested file does not exist.
    #[error("File not found: {path}")]
    FileNotFound {
        /// Path that could not be found.
        path: PathBuf,
    },

    /// Reading the file contents failed.
    #[error("Failed to read file: {path}: {source}")]
    ReadFailed {
        /// Path that failed to read.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },

    /// Parsing the file failed.
    #[error("Failed to parse file: {path}: {reason}")]
    ParseFailed {
        /// Path that failed to parse.
        path: PathBuf,
        /// Human-readable parse-failure reason.
        reason: String,
    },

    /// Cycle detected while resolving dependencies.
    #[error("Circular dependency detected: {0:?}")]
    CircularDependency(Vec<PathBuf>),

    /// File format is not recognized or invalid.
    #[error("Invalid file format: {path}")]
    InvalidFormat {
        /// Path with the invalid format.
        path: PathBuf,
    },

    /// A declared dependency could not be found.
    #[error("Dependency not found: {dependency} required by {dependent}")]
    DependencyNotFound {
        /// Missing dependency path.
        dependency: PathBuf,
        /// Dependent path that requested it.
        dependent: PathBuf,
    },
}

/// Error type for file watching operations.
#[derive(Debug, Error)]
pub enum WatchError {
    /// Failed to construct the underlying watcher.
    #[error("Failed to initialize file watcher: {0}")]
    InitFailed(String),

    /// Watching a specific path failed.
    #[error("Failed to watch path: {path}: {reason}")]
    WatchFailed {
        /// Path that failed to watch.
        path: PathBuf,
        /// Human-readable failure reason.
        reason: String,
    },

    /// Unwatching a specific path failed.
    #[error("Failed to unwatch path: {path}: {reason}")]
    UnwatchFailed {
        /// Path that failed to unwatch.
        path: PathBuf,
        /// Human-readable failure reason.
        reason: String,
    },

    /// The watcher's event channel closed unexpectedly.
    #[error("Watcher channel closed")]
    ChannelClosed,

    /// Underlying `notify` crate error.
    #[error("Notify error: {0}")]
    Notify(#[from] notify::Error),
}

/// Result type using EngineError.
pub type EngineResult<T> = Result<T, EngineError>;

/// Result type using LoadError.
pub type LoadResult<T> = Result<T, LoadError>;

/// Result type using WatchError.
pub type WatchResult<T> = Result<T, WatchError>;
