//! Test harness adapter for Fusabi TUI applications.
//!
//! This module provides a specialized test harness that adapts `ratatui-testlib`
//! for testing Fusabi TUI applications. It wraps the core `TuiTestHarness` and
//! provides Fusabi-specific testing utilities.
//!
//! # Features
//!
//! - PTY-based testing of compiled Fusabi TUI applications
//! - Keyboard and mouse input simulation
//! - Screen state verification and assertions
//! - Snapshot testing support
//! - Custom wait conditions for Fusabi UI patterns
//!
//! # Example
//!
//! ```rust,no_run
//! use fusabi_tui_test::FusabiTuiHarness;
//! use portable_pty::CommandBuilder;
//!
//! #[test]
//! fn test_basic_app() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut harness = FusabiTuiHarness::new(80, 24)?;
//!
//!     // Spawn the basic_app example
//!     let mut cmd = CommandBuilder::new("cargo");
//!     cmd.arg("run").arg("--example").arg("basic_app");
//!     harness.spawn(cmd)?;
//!
//!     // Wait for UI to render
//!     harness.wait_for_title("Fusabi TUI - Basic Example")?;
//!
//!     // Verify initial state
//!     harness.assert_contains("Welcome to fusabi-tui-runtime")?;
//!     harness.assert_contains("Counter: 0")?;
//!
//!     // Simulate user input
//!     harness.send_char('q')?;
//!     harness.wait_for_exit()?;
//!
//!     Ok(())
//! }
//! ```

use ratatui_testlib::{
    CommandBuilder, KeyCode, Result, ScreenState, TermTestError, TuiTestHarness,
};
use std::time::Duration;

/// A test harness specialized for Fusabi TUI applications.
///
/// This wraps `ratatui_testlib::TuiTestHarness` and provides Fusabi-specific
/// testing utilities and convenience methods.
pub struct FusabiTuiHarness {
    inner: TuiTestHarness,
}

impl FusabiTuiHarness {
    /// Creates a new test harness with the specified terminal dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Terminal width in columns (typically 80)
    /// * `height` - Terminal height in rows (typically 24)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::FusabiTuiHarness;
    /// let mut harness = FusabiTuiHarness::new(80, 24)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(width: u16, height: u16) -> Result<Self> {
        let inner = TuiTestHarness::new(width, height)?;
        Ok(Self { inner })
    }

    /// Creates a new test harness with a custom timeout.
    ///
    /// # Arguments
    ///
    /// * `width` - Terminal width in columns
    /// * `height` - Terminal height in rows
    /// * `timeout` - Custom timeout for wait operations
    pub fn with_timeout(width: u16, height: u16, timeout: Duration) -> Result<Self> {
        let inner = TuiTestHarness::builder()
            .with_size(width, height)
            .with_timeout(timeout)
            .build()?;
        Ok(Self { inner })
    }

    /// Spawns a Fusabi TUI application in the test terminal.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::FusabiTuiHarness;
    /// # use portable_pty::CommandBuilder;
    /// # let mut harness = FusabiTuiHarness::new(80, 24)?;
    /// let mut cmd = CommandBuilder::new("cargo");
    /// cmd.arg("run").arg("--example").arg("basic_app");
    /// harness.spawn(cmd)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn spawn(&mut self, cmd: CommandBuilder) -> Result<()> {
        self.inner.spawn(cmd)
    }

    /// Gets the current screen contents as a string.
    ///
    /// Returns the entire terminal buffer joined with newlines.
    pub fn screen_contents(&self) -> String {
        self.inner.screen_contents()
    }

    /// Gets a reference to the underlying screen state.
    ///
    /// This allows direct access to individual cells, cursor position, etc.
    pub fn state(&self) -> &ScreenState {
        self.inner.state()
    }

    /// Gets the current cursor position as (row, col) in 0-based indexing.
    pub fn cursor_position(&self) -> (u16, u16) {
        self.inner.cursor_position()
    }

    /// Checks if the spawned process is still running.
    pub fn is_running(&mut self) -> bool {
        self.inner.is_running()
    }

    /// Updates the screen state by reading from the PTY.
    ///
    /// This should be called after input events to ensure the screen state
    /// reflects the latest output.
    pub fn update_state(&mut self) -> Result<()> {
        self.inner.update_state()
    }

    // === Input Simulation ===

    /// Sends a single character to the application.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::FusabiTuiHarness;
    /// # let mut harness = FusabiTuiHarness::new(80, 24)?;
    /// harness.send_char('q')?;  // Send 'q' to quit
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn send_char(&mut self, c: char) -> Result<()> {
        self.inner.send_key(KeyCode::Char(c))
    }

    /// Sends a text string to the application.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::FusabiTuiHarness;
    /// # let mut harness = FusabiTuiHarness::new(80, 24)?;
    /// harness.send_text("hello world")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn send_text(&mut self, text: &str) -> Result<()> {
        self.inner.send_text(text)
    }

    /// Sends a special key (e.g., Enter, Esc, Arrow keys).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::FusabiTuiHarness;
    /// # use ratatui_testlib::KeyCode;
    /// # let mut harness = FusabiTuiHarness::new(80, 24)?;
    /// harness.send_key(KeyCode::Enter)?;
    /// harness.send_key(KeyCode::Up)?;
    /// harness.send_key(KeyCode::Down)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn send_key(&mut self, key: KeyCode) -> Result<()> {
        self.inner.send_key(key)
    }

    // === Wait Conditions ===

    /// Waits for the specified text to appear on screen.
    ///
    /// Polls the screen state until the text is found or timeout is reached.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::FusabiTuiHarness;
    /// # let mut harness = FusabiTuiHarness::new(80, 24)?;
    /// harness.wait_for_text("Welcome")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn wait_for_text(&mut self, text: &str) -> Result<()> {
        self.inner.wait_for_text(text)
    }

    /// Waits for a title block containing the specified text.
    ///
    /// This is useful for Fusabi TUI apps that use Block widgets with titles.
    pub fn wait_for_title(&mut self, title: &str) -> Result<()> {
        self.wait_for_text(title)
    }

    /// Waits for a custom condition to be met.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::FusabiTuiHarness;
    /// # let mut harness = FusabiTuiHarness::new(80, 24)?;
    /// harness.wait_for(|state| {
    ///     state.contents().contains("Counter:") &&
    ///     state.cursor_position().0 > 10
    /// })?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn wait_for<F>(&mut self, condition: F) -> Result<()>
    where
        F: Fn(&ScreenState) -> bool,
    {
        self.inner.wait_for(condition)
    }

    /// Waits for the spawned process to exit.
    ///
    /// Returns an error if the process doesn't exit within the timeout.
    pub fn wait_for_exit(&mut self) -> Result<()> {
        // Simple polling approach to wait for process to exit
        let timeout = Duration::from_secs(5);
        let start = std::time::Instant::now();
        let poll_interval = Duration::from_millis(100);

        while self.inner.is_running() {
            if start.elapsed() > timeout {
                return Err(TermTestError::Timeout { timeout_ms: timeout.as_millis() as u64 });
            }
            std::thread::sleep(poll_interval);
        }
        Ok(())
    }

    // === Assertions ===

    /// Asserts that the screen contains the specified text.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::FusabiTuiHarness;
    /// # let mut harness = FusabiTuiHarness::new(80, 24)?;
    /// harness.assert_contains("Welcome to fusabi-tui-runtime")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn assert_contains(&self, text: &str) -> Result<()> {
        let contents = self.screen_contents();
        if contents.contains(text) {
            Ok(())
        } else {
            Err(TermTestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Expected text '{}' not found in screen contents:\n{}",
                    text, contents
                ),
            )))
        }
    }

    /// Asserts that the screen does NOT contain the specified text.
    pub fn assert_not_contains(&self, text: &str) -> Result<()> {
        let contents = self.screen_contents();
        if !contents.contains(text) {
            Ok(())
        } else {
            Err(TermTestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unexpected text '{}' found in screen contents:\n{}",
                    text, contents
                ),
            )))
        }
    }

    /// Asserts that the cursor is at the expected position.
    pub fn assert_cursor_at(&self, row: u16, col: u16) -> Result<()> {
        let (actual_row, actual_col) = self.cursor_position();
        if actual_row == row && actual_col == col {
            Ok(())
        } else {
            Err(TermTestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Cursor position mismatch: expected ({}, {}), got ({}, {})",
                    row, col, actual_row, actual_col
                ),
            )))
        }
    }

    /// Asserts that a Fusabi TUI block with the given title exists.
    ///
    /// This verifies that the title appears in the expected format used by
    /// the fusabi-tui-widgets Block widget.
    pub fn assert_block_title(&self, title: &str) -> Result<()> {
        self.assert_contains(title)
    }

    // === Fusabi-Specific Helpers ===

    /// Waits for the Fusabi TUI counter to reach or exceed the specified value.
    ///
    /// This is specific to apps that display a "Counter: N" pattern.
    pub fn wait_for_counter(&mut self, min_value: u32) -> Result<()> {
        self.wait_for(move |state| {
            let contents = state.contents();
            if let Some(counter_line) = contents.lines().find(|line| line.contains("Counter:")) {
                // Extract number after "Counter: "
                if let Some(num_str) = counter_line.split("Counter:").nth(1) {
                    if let Ok(value) = num_str.trim().parse::<u32>() {
                        return value >= min_value;
                    }
                }
            }
            false
        })
    }

    /// Waits for the Fusabi TUI uptime to reach or exceed the specified seconds.
    ///
    /// This is specific to apps that display an "Uptime: Ns" pattern.
    pub fn wait_for_uptime(&mut self, min_seconds: u64) -> Result<()> {
        self.wait_for(move |state| {
            let contents = state.contents();
            if let Some(uptime_line) = contents.lines().find(|line| line.contains("Uptime:")) {
                // Extract number before 's'
                if let Some(num_str) = uptime_line.split("Uptime:").nth(1) {
                    let num_str = num_str.trim().trim_end_matches('s');
                    if let Ok(value) = num_str.parse::<u64>() {
                        return value >= min_seconds;
                    }
                }
            }
            false
        })
    }

    /// Takes a snapshot of the current screen for visual regression testing.
    ///
    /// The snapshot name should be unique per test case.
    /// Note: This requires the `insta` crate to be added as a dev-dependency.
    #[allow(dead_code)]
    pub fn snapshot(&self, _name: &str) {
        // Placeholder for insta snapshot testing
        // To use: Add insta = "1.34" to dev-dependencies and enable snapshots feature
        // insta::assert_snapshot!(name, self.screen_contents());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harness_creation() {
        let harness = FusabiTuiHarness::new(80, 24);
        assert!(harness.is_ok());
    }

    #[test]
    fn test_custom_timeout() {
        let harness = FusabiTuiHarness::with_timeout(80, 24, Duration::from_secs(10));
        assert!(harness.is_ok());
    }
}
