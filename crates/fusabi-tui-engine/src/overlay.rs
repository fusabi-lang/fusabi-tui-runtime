//! Development overlay for displaying diagnostics and errors during hot reload.

use crate::error::EngineError;
use fusabi_tui_core::buffer::Buffer;
use fusabi_tui_core::layout::{Alignment, Constraint, Direction, Layout, Rect};
use fusabi_tui_core::style::{Color, Modifier, Style};
use fusabi_tui_widgets::block::{Block, BorderType, Borders};
use fusabi_tui_widgets::paragraph::Paragraph;
use fusabi_tui_widgets::text::{Line, Span, Text};
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
        let error = &self.error;

        // Determine colors based on severity
        let (border_color, title_color) = match error.severity {
            ErrorSeverity::Error => (Color::Red, Color::Red),
            ErrorSeverity::Warning => (Color::Yellow, Color::Yellow),
            ErrorSeverity::Info => (Color::Blue, Color::Blue),
        };

        // Create the block with borders
        let block = Block::default()
            .title(Line::from(vec![
                Span::styled(" ", Style::default()),
                Span::styled(
                    error.severity.as_str(),
                    Style::default()
                        .fg(title_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(": ", Style::default().fg(Color::White)),
                Span::styled(
                    &error.title,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ", Style::default()),
            ]))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(Color::Black));

        let inner = block.inner(area);
        block.render(area, buf);

        // Split the inner area into sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Spacer
                Constraint::Min(5),    // Error message
                Constraint::Length(1), // Spacer
                Constraint::Length(3), // Location info
                Constraint::Length(1), // Spacer
                Constraint::Min(0),    // Hints
                Constraint::Length(2), // Footer
            ])
            .split(inner);

        // Render error message
        let mut message_text = Text::from(&error.message);
        message_text.style = Style::default().fg(Color::White);
        let message_para = Paragraph::new(message_text).alignment(Alignment::Left);
        message_para.render(chunks[1], buf);

        // Render location info if available
        if let Some(source) = &error.source {
            let location_line = if let (Some(line), Some(col)) = (error.line, error.column) {
                format!("  at {}:{}:{}", source, line, col)
            } else if let Some(line) = error.line {
                format!("  at {}:{}", source, line)
            } else {
                format!("  in {}", source)
            };

            let location_text = Text::from(Line::from(vec![
                Span::styled("Location:", Style::default().fg(Color::Gray)),
                Span::raw(" "),
                Span::styled(
                    location_line,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));

            let location_para = Paragraph::new(location_text).alignment(Alignment::Left);
            location_para.render(chunks[3], buf);
        }

        // Render hints if available
        if !error.hints.is_empty() {
            let mut hints_lines = vec![Line::from(Span::styled(
                "Hints:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ))];

            for hint in &error.hints {
                hints_lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled("•", Style::default().fg(Color::Green)),
                    Span::raw(" "),
                    Span::styled(hint, Style::default().fg(Color::White)),
                ]));
            }

            let hints_text = Text::from(hints_lines);
            let hints_para = Paragraph::new(hints_text).alignment(Alignment::Left);
            hints_para.render(chunks[5], buf);
        }

        // Render footer with instructions
        let footer = Text::from(Line::from(vec![
            Span::styled(
                "Press ",
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::styled(
                "Ctrl+D",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " to dismiss, ",
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::styled(
                "Ctrl+R",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " to reload",
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
        ]));

        let footer_para = Paragraph::new(footer).alignment(Alignment::Center);
        footer_para.render(chunks[6], buf);
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
