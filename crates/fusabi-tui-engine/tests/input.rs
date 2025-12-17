//! Input simulation helpers for Fusabi TUI testing.
//!
//! This module provides high-level input simulation utilities that abstract
//! over common input patterns used in Fusabi TUI applications.

use ratatui_testlib::{KeyCode, Result};
use std::time::Duration;

/// A sequence of input events that can be replayed.
///
/// This allows you to define complex input scenarios and replay them
/// across multiple tests.
#[derive(Debug, Clone)]
pub struct InputSequence {
    steps: Vec<InputStep>,
}

/// A single step in an input sequence.
#[derive(Debug, Clone)]
pub enum InputStep {
    /// Send a single key
    Key(KeyCode),
    /// Send text (multiple characters)
    Text(String),
    /// Wait for a duration
    Delay(Duration),
    /// Wait for text to appear on screen
    WaitForText(String),
}

impl InputSequence {
    /// Creates a new empty input sequence.
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Adds a key press to the sequence.
    pub fn key(mut self, key: KeyCode) -> Self {
        self.steps.push(InputStep::Key(key));
        self
    }

    /// Adds text input to the sequence.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.steps.push(InputStep::Text(text.into()));
        self
    }

    /// Adds a delay to the sequence.
    pub fn delay(mut self, duration: Duration) -> Self {
        self.steps.push(InputStep::Delay(duration));
        self
    }

    /// Adds a wait condition for text to appear.
    pub fn wait_for(mut self, text: impl Into<String>) -> Self {
        self.steps.push(InputStep::WaitForText(text.into()));
        self
    }

    /// Gets the steps in this sequence.
    pub fn steps(&self) -> &[InputStep] {
        &self.steps
    }
}

impl Default for InputSequence {
    fn default() -> Self {
        Self::new()
    }
}

/// Common input sequences for Fusabi TUI applications.
pub struct CommonInputs;

impl CommonInputs {
    /// Sequence to quit an application (press 'q').
    pub fn quit() -> InputSequence {
        InputSequence::new().key(KeyCode::Char('q'))
    }

    /// Sequence to confirm an action (press Enter).
    pub fn confirm() -> InputSequence {
        InputSequence::new().key(KeyCode::Enter)
    }

    /// Sequence to cancel an action (press Esc).
    pub fn cancel() -> InputSequence {
        InputSequence::new().key(KeyCode::Esc)
    }

    /// Sequence to navigate down in a list.
    pub fn navigate_down(count: usize) -> InputSequence {
        let mut seq = InputSequence::new();
        for _ in 0..count {
            seq = seq.key(KeyCode::Down);
        }
        seq
    }

    /// Sequence to navigate up in a list.
    pub fn navigate_up(count: usize) -> InputSequence {
        let mut seq = InputSequence::new();
        for _ in 0..count {
            seq = seq.key(KeyCode::Up);
        }
        seq
    }

    /// Sequence to navigate to the start of a list (Home).
    pub fn go_to_start() -> InputSequence {
        InputSequence::new().key(KeyCode::Home)
    }

    /// Sequence to navigate to the end of a list (End).
    pub fn go_to_end() -> InputSequence {
        InputSequence::new().key(KeyCode::End)
    }

    /// Sequence to page down.
    pub fn page_down() -> InputSequence {
        InputSequence::new().key(KeyCode::PageDown)
    }

    /// Sequence to page up.
    pub fn page_up() -> InputSequence {
        InputSequence::new().key(KeyCode::PageUp)
    }

    /// Sequence to navigate to next tab (Tab).
    pub fn next_tab() -> InputSequence {
        InputSequence::new().key(KeyCode::Tab)
    }

    /// Sequence to navigate to previous tab (Shift+Tab).
    /// Note: BackTab may not be available in all terminal emulators.
    pub fn prev_tab() -> InputSequence {
        // Use Tab as fallback since BackTab isn't in the KeyCode enum
        InputSequence::new().key(KeyCode::Tab)
    }

    /// Sequence to type text with delays between characters (simulates human typing).
    pub fn type_slowly(text: &str, char_delay_ms: u64) -> InputSequence {
        let mut seq = InputSequence::new();
        for c in text.chars() {
            seq = seq.key(KeyCode::Char(c))
                .delay(Duration::from_millis(char_delay_ms));
        }
        seq
    }

    /// Sequence to select an item in a list (navigate + confirm).
    pub fn select_item(index: usize) -> InputSequence {
        Self::navigate_down(index).key(KeyCode::Enter)
    }

    /// Sequence to enter search mode and type a query.
    pub fn search(query: &str) -> InputSequence {
        InputSequence::new()
            .key(KeyCode::Char('/'))  // Common search key
            .text(query)
            .key(KeyCode::Enter)
    }

    /// Sequence to clear input (Ctrl+U or Ctrl+W pattern).
    pub fn clear_input() -> InputSequence {
        InputSequence::new()
            .key(KeyCode::Home)
            .key(KeyCode::Delete)
    }

    /// Sequence to toggle a boolean option (Space).
    pub fn toggle() -> InputSequence {
        InputSequence::new().key(KeyCode::Char(' '))
    }

    /// Sequence to save (Ctrl+S).
    pub fn save() -> InputSequence {
        // Note: ratatui-testlib may need modifier support enhancement
        InputSequence::new().key(KeyCode::Char('s'))
    }

    /// Sequence to refresh (F5).
    pub fn refresh() -> InputSequence {
        InputSequence::new().key(KeyCode::F(5))
    }
}

/// Builder for creating custom input sequences with fluent API.
pub struct InputBuilder {
    sequence: InputSequence,
}

impl InputBuilder {
    /// Creates a new input builder.
    pub fn new() -> Self {
        Self {
            sequence: InputSequence::new(),
        }
    }

    /// Adds a key press.
    pub fn press(mut self, key: KeyCode) -> Self {
        self.sequence = self.sequence.key(key);
        self
    }

    /// Types text.
    pub fn type_text(mut self, text: impl Into<String>) -> Self {
        self.sequence = self.sequence.text(text);
        self
    }

    /// Adds a delay.
    pub fn pause(mut self, duration: Duration) -> Self {
        self.sequence = self.sequence.delay(duration);
        self
    }

    /// Waits for text to appear.
    pub fn wait(mut self, text: impl Into<String>) -> Self {
        self.sequence = self.sequence.wait_for(text);
        self
    }

    /// Navigates down N times.
    pub fn down(mut self, count: usize) -> Self {
        for _ in 0..count {
            self.sequence = self.sequence.key(KeyCode::Down);
        }
        self
    }

    /// Navigates up N times.
    pub fn up(mut self, count: usize) -> Self {
        for _ in 0..count {
            self.sequence = self.sequence.key(KeyCode::Up);
        }
        self
    }

    /// Navigates right N times.
    pub fn right(mut self, count: usize) -> Self {
        for _ in 0..count {
            self.sequence = self.sequence.key(KeyCode::Right);
        }
        self
    }

    /// Navigates left N times.
    pub fn left(mut self, count: usize) -> Self {
        for _ in 0..count {
            self.sequence = self.sequence.key(KeyCode::Left);
        }
        self
    }

    /// Presses Enter.
    pub fn enter(mut self) -> Self {
        self.sequence = self.sequence.key(KeyCode::Enter);
        self
    }

    /// Presses Escape.
    pub fn escape(mut self) -> Self {
        self.sequence = self.sequence.key(KeyCode::Esc);
        self
    }

    /// Presses Tab.
    pub fn tab(mut self) -> Self {
        self.sequence = self.sequence.key(KeyCode::Tab);
        self
    }

    /// Builds the input sequence.
    pub fn build(self) -> InputSequence {
        self.sequence
    }
}

impl Default for InputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_sequence_builder() {
        let seq = InputSequence::new()
            .text("hello")
            .delay(Duration::from_millis(100))
            .key(KeyCode::Enter);

        assert_eq!(seq.steps().len(), 3);
    }

    #[test]
    fn test_common_inputs() {
        let quit = CommonInputs::quit();
        assert_eq!(quit.steps().len(), 1);

        let nav_down = CommonInputs::navigate_down(3);
        assert_eq!(nav_down.steps().len(), 3);

        let select = CommonInputs::select_item(2);
        assert!(select.steps().len() > 0);
    }

    #[test]
    fn test_input_builder_fluent() {
        let seq = InputBuilder::new()
            .down(3)
            .enter()
            .wait("Result")
            .type_text("test")
            .build();

        assert!(seq.steps().len() > 0);
    }

    #[test]
    fn test_type_slowly() {
        let seq = CommonInputs::type_slowly("abc", 50);
        // Should have 3 keys + 3 delays = 6 steps
        assert_eq!(seq.steps().len(), 6);
    }
}
