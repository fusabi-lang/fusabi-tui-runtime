//! Integration test helpers for Fusabi TUI applications.
//!
//! This module provides a comprehensive testing framework built on top of
//! `ratatui-testlib` for testing Fusabi TUI applications.
//!
//! # Modules
//!
//! - `harness` - Test harness adapter for Fusabi TUI apps
//! - `pty_utils` - PTY-based testing utilities and command builders
//! - `snapshot` - Screenshot comparison and snapshot testing
//! - `input` - Input simulation helpers and common sequences
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use fusabi_tui_test::{FusabiTuiHarness, FusabiExampleBuilder};
//!
//! #[test]
//! fn test_my_app() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut harness = FusabiTuiHarness::new(80, 24)?;
//!     let cmd = FusabiExampleBuilder::new("basic_app").build();
//!     harness.spawn(cmd)?;
//!     harness.wait_for_title("Fusabi TUI")?;
//!     harness.send_char('q')?;
//!     Ok(())
//! }
//! ```

pub mod harness;
pub mod pty_utils;
pub mod snapshot;
pub mod input;

// Re-export commonly used items
pub use harness::FusabiTuiHarness;
pub use pty_utils::{
    FusabiExampleBuilder, FusabiBinaryBuilder, TerminalPreset, PerformanceMetrics,
    workspace_root, examples_dir, target_dir,
};
pub use snapshot::{
    ScreenSnapshot, Region, SnapshotComparison, GoldenFile, normalize_screen,
};
pub use input::{
    InputSequence, InputStep, CommonInputs, InputBuilder,
};

// Re-export from ratatui-testlib for convenience
pub use ratatui_testlib::{
    KeyCode, Result, ScreenState, TermTestError, TuiTestHarness,
};
