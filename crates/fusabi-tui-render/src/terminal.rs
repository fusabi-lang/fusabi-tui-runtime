//! Terminal abstraction for higher-level TUI rendering.
//!
//! This module provides a `Terminal` and `Frame` abstraction similar to ratatui's
//! pattern, making it easier to migrate applications from ratatui to fusabi-tui.

use fusabi_tui_core::buffer::Buffer;
use fusabi_tui_core::layout::Rect;
use fusabi_tui_widgets::widget::{StatefulWidget, Widget};

use crate::error::Result;
use crate::renderer::Renderer;

/// A terminal abstraction that manages the rendering lifecycle.
///
/// The `Terminal` wraps a renderer and provides a higher-level API for drawing
/// complete frames. It handles buffer management and differential rendering.
///
/// # Example
///
/// ```no_run
/// use fusabi_tui_render::prelude::*;
/// use fusabi_tui_render::terminal::Terminal;
/// use fusabi_tui_core::layout::Rect;
/// use std::io::stdout;
///
/// # fn main() -> Result<()> {
/// let renderer = CrosstermRenderer::new(stdout())?;
/// let mut terminal = Terminal::new(renderer)?;
///
/// terminal.draw(|f| {
///     // Render widgets to frame
/// })?;
/// # Ok(())
/// # }
/// ```
pub struct Terminal<R: Renderer> {
    renderer: R,
}

impl<R: Renderer> Terminal<R> {
    /// Creates a new terminal with the given renderer.
    pub fn new(renderer: R) -> Result<Self> {
        Ok(Self { renderer })
    }

    /// Draws a frame using the provided render function.
    ///
    /// The render function receives a `Frame` which can be used to render widgets.
    /// After rendering, the frame's buffer is automatically drawn to the terminal.
    pub fn draw<F>(&mut self, render_fn: F) -> Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame),
    {
        let size = self.renderer.size()?;
        let mut buffer = Buffer::new(size);
        let mut frame = Frame::new(&mut buffer, size);

        render_fn(&mut frame);

        self.renderer.draw(&buffer)?;
        self.renderer.flush()?;

        Ok(CompletedFrame {
            area: size,
        })
    }

    /// Gets the current terminal size.
    pub fn size(&self) -> Result<Rect> {
        self.renderer.size()
    }

    /// Clears the terminal screen.
    pub fn clear(&mut self) -> Result<()> {
        self.renderer.clear()
    }

    /// Shows or hides the cursor.
    pub fn show_cursor(&mut self, show: bool) -> Result<()> {
        self.renderer.show_cursor(show)
    }

    /// Gets mutable access to the underlying renderer.
    pub fn backend_mut(&mut self) -> &mut R {
        &mut self.renderer
    }

    /// Gets a reference to the underlying renderer.
    pub fn backend(&self) -> &R {
        &self.renderer
    }
}

/// A frame for rendering widgets.
///
/// The `Frame` provides methods to render widgets within a specific area.
/// It maintains a buffer that will be drawn to the terminal when the frame
/// is complete.
pub struct Frame<'a> {
    buffer: &'a mut Buffer,
    area: Rect,
}

impl<'a> Frame<'a> {
    /// Creates a new frame with the given buffer and area.
    pub fn new(buffer: &'a mut Buffer, area: Rect) -> Self {
        Self { buffer, area }
    }

    /// Returns the area of the frame.
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Returns the buffer size (alias for area for compatibility).
    pub fn size(&self) -> Rect {
        self.area
    }

    /// Renders a widget to the frame at the specified area.
    pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
        widget.render(area, self.buffer);
    }

    /// Renders a stateful widget to the frame at the specified area.
    pub fn render_stateful_widget<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidget,
    {
        widget.render(area, self.buffer, state);
    }

    /// Gets mutable access to the underlying buffer.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        self.buffer
    }

    /// Sets the cursor position for this frame.
    pub fn set_cursor(&mut self, _x: u16, _y: u16) {
        // Note: cursor position will be handled by the terminal
        // This is a compatibility shim
    }
}

/// Information about a completed frame.
pub struct CompletedFrame {
    /// The area that was rendered.
    pub area: Rect,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::TestRenderer;
    use fusabi_tui_core::style::Style;
    use fusabi_tui_widgets::paragraph::Paragraph;

    #[test]
    fn test_terminal_draw() {
        let renderer = TestRenderer::new(20, 5);
        let mut terminal = Terminal::new(renderer).unwrap();

        let result = terminal.draw(|f| {
            let para = Paragraph::new("Hello");
            f.render_widget(para, f.area());
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_frame_area() {
        let mut buffer = Buffer::new(Rect::new(0, 0, 80, 24));
        let frame = Frame::new(&mut buffer, Rect::new(0, 0, 80, 24));

        assert_eq!(frame.area(), Rect::new(0, 0, 80, 24));
        assert_eq!(frame.size(), Rect::new(0, 0, 80, 24));
    }
}
