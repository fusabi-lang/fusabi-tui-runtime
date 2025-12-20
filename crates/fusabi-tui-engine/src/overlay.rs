//! Development overlay for displaying diagnostics and errors during hot reload.

use crate::error::EngineError;
use fusabi_tui_core::buffer::Buffer;
use fusabi_tui_core::layout::Rect;
use fusabi_tui_core::style::{Color, Modifier, Style};
use fusabi_tui_widgets::block::Block;
use fusabi_tui_widgets::borders::{BorderType, Borders};
use fusabi_tui_widgets::paragraph::Paragraph;
use fusabi_tui_widgets::widget::Widget;
use std::time::{Duration, Instant};

/// An overlay widget that displays error messages and diagnostics during development.
///
/// The error overlay appears on top of the regular dashboard when compilation or
/// runtime errors occur, providing immediate feedback to developers without crashing
/// the application.
#[derive(Debug, Clone)]
pub struct ErrorOverlay {
    /// The error to display.
    error: ErrorMessage,

    /// When the error was first shown.
    timestamp: Instant,

    /// Whether to show the overlay (can be dismissed).
    visible: bool,

    /// Auto-dismiss after this duration (None = manual dismiss only).
    auto_dismiss_after: Option<Duration>,
}

/// A displayable error message with context.
#[derive(Debug, Clone)]
pub struct ErrorMessage {
    /// The main error title/summary.
    pub title: String,

    /// Detailed error message.
    pub message: String,

    /// Optional source file path where the error occurred.
    pub source: Option<String>,

    /// Optional line number where the error occurred.
    pub line: Option<usize>,

    /// Optional column number where the error occurred.
    pub column: Option<usize>,

    /// Error severity level.
    pub severity: ErrorSeverity,

    /// Additional context or hints for fixing the error.
    pub hints: Vec<String>,
}

/// Error severity levels for visual styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Critical error that prevents execution.
    Error,

    /// Warning that might indicate a problem.
    Warning,

    /// Informational message.
    Info,
}

impl ErrorOverlay {
    /// Create a new error overlay from an error.
    pub fn new(error: ErrorMessage) -> Self {
        Self {
            error,
            timestamp: Instant::now(),
            visible: true,
            auto_dismiss_after: None,
        }
    }

    /// Create an error overlay from an EngineError.
    pub fn from_engine_error(error: &EngineError) -> Self {
        let error_msg = ErrorMessage::from_engine_error(error);
        Self::new(error_msg)
    }

    /// Create an overlay with auto-dismiss timer.
    pub fn with_auto_dismiss(mut self, duration: Duration) -> Self {
        self.auto_dismiss_after = Some(duration);
        self
    }

    /// Check if the overlay is visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Dismiss the overlay.
    pub fn dismiss(&mut self) {
        self.visible = false;
    }

    /// Show the overlay again.
    pub fn show(&mut self) {
        self.visible = true;
        self.timestamp = Instant::now();
    }

    /// Update the overlay state (handle auto-dismiss).
    pub fn update(&mut self) {
        if let Some(duration) = self.auto_dismiss_after {
            if self.timestamp.elapsed() >= duration {
                self.visible = false;
            }
        }
    }

    /// Get the error message.
    pub fn error(&self) -> &ErrorMessage {
        &self.error
    }

    /// Get the time since the error was shown.
    pub fn elapsed(&self) -> Duration {
        self.timestamp.elapsed()
    }

    /// Render the overlay to a buffer.
    ///
    /// This creates a semi-transparent overlay effect by rendering
    /// a centered error panel on top of the existing buffer content.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        // Create a centered area for the error dialog
        let overlay_area = Self::centered_rect(80, 60, area);

        // Render the error panel
        self.render_error_panel(overlay_area, buf);
    }

    /// Render the error panel content.
    fn render_error_panel(&self, area: Rect, buf: &mut Buffer) {
        use fusabi_tui_widgets::block::Title;

        let error = &self.error;

        // Determine colors based on severity
        let (border_color, title_color) = match error.severity {
            ErrorSeverity::Error => (Color::Red, Color::Red),
            ErrorSeverity::Warning => (Color::Yellow, Color::Yellow),
            ErrorSeverity::Info => (Color::Blue, Color::Blue),
        };

        // Create title string
        let title_str = format!(" {}: {} ", error.severity.as_str(), error.title);
        let title = Title::new(title_str)
            .style(Style::default().fg(title_color).add_modifier(Modifier::BOLD));

        // Create the block with borders
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(Color::Black));

        let inner = block.inner(area);
        block.render(area, buf);

        // Build content as a single string for simplicity
        let mut content = String::new();

        // Error message
        content.push_str(&error.message);
        content.push_str("\n\n");

        // Location info if available
        if let Some(source) = &error.source {
            if let (Some(line), Some(col)) = (error.line, error.column) {
                content.push_str(&format!("Location: {}:{}:{}\n\n", source, line, col));
            } else if let Some(line) = error.line {
                content.push_str(&format!("Location: {}:{}\n\n", source, line));
            } else {
                content.push_str(&format!("Location: {}\n\n", source));
            }
        }

        // Hints
        if !error.hints.is_empty() {
            content.push_str("Hints:\n");
            for hint in &error.hints {
                content.push_str(&format!("  * {}\n", hint));
            }
            content.push('\n');
        }

        // Footer
        content.push_str("Press Ctrl+D to dismiss, Ctrl+R to reload");

        let para = Paragraph::new(content)
            .style(Style::default().fg(Color::White));
        para.render(inner, buf);
    }

    /// Helper function to create a centered rectangle.
    fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
        let horizontal_margin = (area.width.saturating_sub((area.width * percent_x) / 100)) / 2;
        let vertical_margin = (area.height.saturating_sub((area.height * percent_y) / 100)) / 2;

        Rect {
            x: area.x + horizontal_margin,
            y: area.y + vertical_margin,
            width: area.width.saturating_sub(horizontal_margin * 2),
            height: area.height.saturating_sub(vertical_margin * 2),
        }
    }
}

impl ErrorMessage {
    /// Create a new error message.
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            source: None,
            line: None,
            column: None,
            severity: ErrorSeverity::Error,
            hints: Vec::new(),
        }
    }

    /// Create an error message from an EngineError.
    pub fn from_engine_error(error: &EngineError) -> Self {
        use crate::error::LoadError;

        match error {
            EngineError::LoadError(load_err) => match load_err {
                LoadError::FileNotFound { path } => Self::new(
                    "File Not Found",
                    format!("Could not find file: {}", path.display()),
                )
                .with_source(path.display().to_string())
                .with_hint("Check that the file path is correct")
                .with_hint("Make sure the file exists in the expected location"),

                LoadError::ReadFailed { path, source } => Self::new(
                    "Failed to Read File",
                    format!("Could not read file: {}", source),
                )
                .with_source(path.display().to_string())
                .with_hint("Check file permissions")
                .with_hint("Ensure the file is not locked by another process"),

                LoadError::ParseFailed { path, reason } => Self::new(
                    "Parse Error",
                    format!("Failed to parse file: {}", reason),
                )
                .with_source(path.display().to_string())
                .with_hint("Check the syntax of your .fsx file")
                .with_hint("Look for unclosed brackets, quotes, or other syntax errors"),

                _ => Self::new("Load Error", format!("{}", load_err)),
            },

            EngineError::WatchError(watch_err) => {
                Self::new("Watch Error", format!("{}", watch_err))
                    .with_hint("Try restarting the application")
                    .with_severity(ErrorSeverity::Warning)
            }

            EngineError::Render(render_err) => {
                Self::new("Render Error", format!("{}", render_err))
                    .with_hint("This may be a temporary issue")
                    .with_hint("Try resizing the terminal or reloading")
                    .with_severity(ErrorSeverity::Warning)
            }

            EngineError::InvalidState(msg) => {
                Self::new("Invalid State", msg.clone()).with_hint("Try reloading the dashboard")
            }

            _ => Self::new("Error", format!("{}", error)),
        }
    }

    /// Set the source file.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set the line number.
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Set the column number.
    pub fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    /// Set the severity level.
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Add a hint.
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }
}

impl ErrorSeverity {
    /// Get the string representation of the severity.
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorSeverity::Error => "ERROR",
            ErrorSeverity::Warning => "WARNING",
            ErrorSeverity::Info => "INFO",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message_new() {
        let msg = ErrorMessage::new("Test Error", "This is a test error message");
        assert_eq!(msg.title, "Test Error");
        assert_eq!(msg.message, "This is a test error message");
        assert_eq!(msg.severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_error_message_with_source() {
        let msg = ErrorMessage::new("Error", "Message").with_source("test.fsx");
        assert_eq!(msg.source, Some("test.fsx".to_string()));
    }

    #[test]
    fn test_error_message_with_location() {
        let msg = ErrorMessage::new("Error", "Message")
            .with_line(42)
            .with_column(10);
        assert_eq!(msg.line, Some(42));
        assert_eq!(msg.column, Some(10));
    }

    #[test]
    fn test_error_message_with_hints() {
        let msg = ErrorMessage::new("Error", "Message")
            .with_hint("Hint 1")
            .with_hint("Hint 2");
        assert_eq!(msg.hints.len(), 2);
        assert_eq!(msg.hints[0], "Hint 1");
        assert_eq!(msg.hints[1], "Hint 2");
    }

    #[test]
    fn test_error_severity_as_str() {
        assert_eq!(ErrorSeverity::Error.as_str(), "ERROR");
        assert_eq!(ErrorSeverity::Warning.as_str(), "WARNING");
        assert_eq!(ErrorSeverity::Info.as_str(), "INFO");
    }

    #[test]
    fn test_error_overlay_new() {
        let msg = ErrorMessage::new("Test", "Message");
        let overlay = ErrorOverlay::new(msg);
        assert!(overlay.is_visible());
    }

    #[test]
    fn test_error_overlay_dismiss() {
        let msg = ErrorMessage::new("Test", "Message");
        let mut overlay = ErrorOverlay::new(msg);
        assert!(overlay.is_visible());

        overlay.dismiss();
        assert!(!overlay.is_visible());
    }

    #[test]
    fn test_error_overlay_auto_dismiss() {
        let msg = ErrorMessage::new("Test", "Message");
        let mut overlay = ErrorOverlay::new(msg).with_auto_dismiss(Duration::from_millis(1));

        assert!(overlay.is_visible());

        // Wait for auto-dismiss
        std::thread::sleep(Duration::from_millis(10));
        overlay.update();

        assert!(!overlay.is_visible());
    }
}
