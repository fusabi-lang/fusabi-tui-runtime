//! Scrollbar widget for rendering scrollable content indicators.
//!
//! This module provides a `Scrollbar` widget that visualizes scrollable content
//! with customizable orientation and symbols.

use fusabi_tui_core::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::{arrow, block, line},
};

use crate::widget::StatefulWidget;

/// Orientation of the scrollbar.
///
/// Determines where the scrollbar is positioned relative to the content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScrollbarOrientation {
    /// Vertical scrollbar on the right side
    VerticalRight,
    /// Vertical scrollbar on the left side
    VerticalLeft,
    /// Horizontal scrollbar on the top
    HorizontalTop,
    /// Horizontal scrollbar on the bottom
    HorizontalBottom,
}

impl Default for ScrollbarOrientation {
    fn default() -> Self {
        Self::VerticalRight
    }
}

/// State for the scrollbar widget.
///
/// Maintains the scrollbar position and content dimensions.
///
/// # Examples
///
/// ```rust
/// use fusabi_tui_widgets::ScrollbarState;
///
/// let mut state = ScrollbarState::new(100)
///     .position(25)
///     .viewport_content_length(10);
/// ```
#[derive(Debug, Clone, Default)]
pub struct ScrollbarState {
    /// Total length of the content
    content_length: usize,
    /// Current scroll position (0-based)
    position: usize,
    /// Length of the visible viewport content
    viewport_content_length: usize,
}

impl ScrollbarState {
    /// Creates a new scrollbar state with the given content length.
    pub fn new(content_length: usize) -> Self {
        Self {
            content_length,
            position: 0,
            viewport_content_length: 0,
        }
    }

    /// Sets the total content length.
    pub fn content_length(mut self, length: usize) -> Self {
        self.content_length = length;
        self
    }

    /// Sets the current scroll position.
    pub fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Sets the viewport content length (visible area).
    pub fn viewport_content_length(mut self, length: usize) -> Self {
        self.viewport_content_length = length;
        self
    }

    /// Gets the total content length.
    pub fn get_content_length(&self) -> usize {
        self.content_length
    }

    /// Gets the current scroll position.
    pub fn get_position(&self) -> usize {
        self.position
    }

    /// Gets the viewport content length.
    pub fn get_viewport_content_length(&self) -> usize {
        self.viewport_content_length
    }

    /// Sets the content length.
    pub fn set_content_length(&mut self, length: usize) {
        self.content_length = length;
    }

    /// Sets the scroll position.
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    /// Sets the viewport content length.
    pub fn set_viewport_content_length(&mut self, length: usize) {
        self.viewport_content_length = length;
    }

    /// Scrolls down by one position.
    pub fn scroll_down(&mut self) {
        if self.position < self.content_length.saturating_sub(self.viewport_content_length) {
            self.position = self.position.saturating_add(1);
        }
    }

    /// Scrolls up by one position.
    pub fn scroll_up(&mut self) {
        self.position = self.position.saturating_sub(1);
    }

    /// Scrolls to the top.
    pub fn scroll_to_top(&mut self) {
        self.position = 0;
    }

    /// Scrolls to the bottom.
    pub fn scroll_to_bottom(&mut self) {
        self.position = self.content_length.saturating_sub(self.viewport_content_length);
    }
}

/// A scrollbar widget for visualizing scrollable content.
///
/// The scrollbar displays a track with a thumb indicator showing the current
/// scroll position within the content.
///
/// # Examples
///
/// ```rust
/// use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::{Color, Style}};
/// use fusabi_tui_widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget};
///
/// let scrollbar = Scrollbar::default()
///     .orientation(ScrollbarOrientation::VerticalRight)
///     .style(Style::default().fg(Color::White));
///
/// let mut state = ScrollbarState::new(100)
///     .position(25)
///     .viewport_content_length(10);
///
/// let area = Rect::new(0, 0, 1, 20);
/// let mut buffer = Buffer::new(area);
/// scrollbar.render(area, &mut buffer, &mut state);
/// ```
#[derive(Debug, Clone)]
pub struct Scrollbar {
    orientation: ScrollbarOrientation,
    begin_symbol: Option<String>,
    end_symbol: Option<String>,
    thumb_symbol: String,
    track_symbol: String,
    style: Style,
}

impl Default for Scrollbar {
    fn default() -> Self {
        Self {
            orientation: ScrollbarOrientation::VerticalRight,
            begin_symbol: Some(arrow::UP.to_string()),
            end_symbol: Some(arrow::DOWN.to_string()),
            thumb_symbol: block::FULL.to_string(),
            track_symbol: line::VERTICAL.to_string(),
            style: Style::default(),
        }
    }
}

impl Scrollbar {
    /// Creates a new scrollbar with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the orientation of the scrollbar.
    pub fn orientation(mut self, orientation: ScrollbarOrientation) -> Self {
        self.orientation = orientation;
        // Update default symbols based on orientation
        match orientation {
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft => {
                if self.begin_symbol.as_deref() == Some(arrow::LEFT)
                    || self.begin_symbol.as_deref() == Some(arrow::RIGHT)
                {
                    self.begin_symbol = Some(arrow::UP.to_string());
                }
                if self.end_symbol.as_deref() == Some(arrow::LEFT)
                    || self.end_symbol.as_deref() == Some(arrow::RIGHT)
                {
                    self.end_symbol = Some(arrow::DOWN.to_string());
                }
                if self.track_symbol == line::HORIZONTAL {
                    self.track_symbol = line::VERTICAL.to_string();
                }
            }
            ScrollbarOrientation::HorizontalTop | ScrollbarOrientation::HorizontalBottom => {
                if self.begin_symbol.as_deref() == Some(arrow::UP)
                    || self.begin_symbol.as_deref() == Some(arrow::DOWN)
                {
                    self.begin_symbol = Some(arrow::LEFT.to_string());
                }
                if self.end_symbol.as_deref() == Some(arrow::UP)
                    || self.end_symbol.as_deref() == Some(arrow::DOWN)
                {
                    self.end_symbol = Some(arrow::RIGHT.to_string());
                }
                if self.track_symbol == line::VERTICAL {
                    self.track_symbol = line::HORIZONTAL.to_string();
                }
            }
        }
        self
    }

    /// Sets the symbol displayed at the beginning of the scrollbar.
    pub fn begin_symbol<T>(mut self, symbol: Option<T>) -> Self
    where
        T: Into<String>,
    {
        self.begin_symbol = symbol.map(Into::into);
        self
    }

    /// Sets the symbol displayed at the end of the scrollbar.
    pub fn end_symbol<T>(mut self, symbol: Option<T>) -> Self
    where
        T: Into<String>,
    {
        self.end_symbol = symbol.map(Into::into);
        self
    }

    /// Sets the symbol used for the thumb (position indicator).
    pub fn thumb_symbol<T>(mut self, symbol: T) -> Self
    where
        T: Into<String>,
    {
        self.thumb_symbol = symbol.into();
        self
    }

    /// Sets the symbol used for the track.
    pub fn track_symbol<T>(mut self, symbol: T) -> Self
    where
        T: Into<String>,
    {
        self.track_symbol = symbol.into();
        self
    }

    /// Sets the style for the scrollbar.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Calculates the thumb position and size based on the state.
    fn calculate_thumb(&self, track_length: usize, state: &ScrollbarState) -> (usize, usize) {
        if state.content_length == 0 || state.viewport_content_length >= state.content_length {
            // No scrolling needed
            return (0, track_length);
        }

        // Calculate thumb size proportional to viewport/content ratio
        let thumb_size = ((state.viewport_content_length as f64 / state.content_length as f64)
            * track_length as f64)
            .max(1.0) as usize;

        // Calculate thumb position based on scroll position
        let scrollable_content = state.content_length.saturating_sub(state.viewport_content_length);
        let scrollable_track = track_length.saturating_sub(thumb_size);

        let thumb_position = if scrollable_content > 0 {
            ((state.position as f64 / scrollable_content as f64) * scrollable_track as f64)
                .round() as usize
        } else {
            0
        };

        (thumb_position, thumb_size)
    }
}

impl StatefulWidget for Scrollbar {
    type State = ScrollbarState;

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        match self.orientation {
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft => {
                self.render_vertical(area, buf, state);
            }
            ScrollbarOrientation::HorizontalTop | ScrollbarOrientation::HorizontalBottom => {
                self.render_horizontal(area, buf, state);
            }
        }
    }
}

impl Scrollbar {
    fn render_vertical(&self, area: Rect, buf: &mut Buffer, state: &ScrollbarState) {
        let x = match self.orientation {
            ScrollbarOrientation::VerticalRight => area.right().saturating_sub(1),
            ScrollbarOrientation::VerticalLeft => area.x,
            _ => area.x,
        };

        let mut y = area.y;
        let mut available_height = area.height as usize;

        // Render begin symbol
        if let Some(ref symbol) = self.begin_symbol {
            if available_height > 0 {
                buf.set_string(x, y, symbol, self.style);
                y = y.saturating_add(1);
                available_height = available_height.saturating_sub(1);
            }
        }

        // Render end symbol (calculate position first)
        let end_y = if self.end_symbol.is_some() && available_height > 0 {
            available_height = available_height.saturating_sub(1);
            Some(area.y.saturating_add((area.height as usize).saturating_sub(1) as u16))
        } else {
            None
        };

        // Calculate thumb position and size
        let track_length = available_height;
        let (thumb_pos, thumb_size) = self.calculate_thumb(track_length, state);

        // Render track and thumb
        for i in 0..track_length {
            let cell_y = y.saturating_add(i as u16);
            if cell_y >= area.bottom() {
                break;
            }

            let symbol = if i >= thumb_pos && i < thumb_pos.saturating_add(thumb_size) {
                &self.thumb_symbol
            } else {
                &self.track_symbol
            };

            buf.set_string(x, cell_y, symbol, self.style);
        }

        // Render end symbol
        if let (Some(end_y), Some(ref symbol)) = (end_y, &self.end_symbol) {
            if end_y < area.bottom() {
                buf.set_string(x, end_y, symbol, self.style);
            }
        }
    }

    fn render_horizontal(&self, area: Rect, buf: &mut Buffer, state: &ScrollbarState) {
        let y = match self.orientation {
            ScrollbarOrientation::HorizontalBottom => area.bottom().saturating_sub(1),
            ScrollbarOrientation::HorizontalTop => area.y,
            _ => area.y,
        };

        let mut x = area.x;
        let mut available_width = area.width as usize;

        // Render begin symbol
        if let Some(ref symbol) = self.begin_symbol {
            if available_width > 0 {
                buf.set_string(x, y, symbol, self.style);
                x = x.saturating_add(1);
                available_width = available_width.saturating_sub(1);
            }
        }

        // Render end symbol (calculate position first)
        let end_x = if self.end_symbol.is_some() && available_width > 0 {
            available_width = available_width.saturating_sub(1);
            Some(area.x.saturating_add((area.width as usize).saturating_sub(1) as u16))
        } else {
            None
        };

        // Calculate thumb position and size
        let track_length = available_width;
        let (thumb_pos, thumb_size) = self.calculate_thumb(track_length, state);

        // Render track and thumb
        for i in 0..track_length {
            let cell_x = x.saturating_add(i as u16);
            if cell_x >= area.right() {
                break;
            }

            let symbol = if i >= thumb_pos && i < thumb_pos.saturating_add(thumb_size) {
                &self.thumb_symbol
            } else {
                &self.track_symbol
            };

            buf.set_string(cell_x, y, symbol, self.style);
        }

        // Render end symbol
        if let (Some(end_x), Some(ref symbol)) = (end_x, &self.end_symbol) {
            if end_x < area.right() {
                buf.set_string(end_x, y, symbol, self.style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_tui_core::style::Color;

    #[test]
    fn test_scrollbar_state_new() {
        let state = ScrollbarState::new(100);
        assert_eq!(state.content_length, 100);
        assert_eq!(state.position, 0);
        assert_eq!(state.viewport_content_length, 0);
    }

    #[test]
    fn test_scrollbar_state_builder() {
        let state = ScrollbarState::new(100)
            .position(25)
            .viewport_content_length(10);
        assert_eq!(state.content_length, 100);
        assert_eq!(state.position, 25);
        assert_eq!(state.viewport_content_length, 10);
    }

    #[test]
    fn test_scrollbar_state_getters() {
        let state = ScrollbarState::new(100)
            .position(25)
            .viewport_content_length(10);
        assert_eq!(state.get_content_length(), 100);
        assert_eq!(state.get_position(), 25);
        assert_eq!(state.get_viewport_content_length(), 10);
    }

    #[test]
    fn test_scrollbar_state_setters() {
        let mut state = ScrollbarState::new(100);
        state.set_position(25);
        state.set_viewport_content_length(10);
        state.set_content_length(200);
        assert_eq!(state.get_content_length(), 200);
        assert_eq!(state.get_position(), 25);
        assert_eq!(state.get_viewport_content_length(), 10);
    }

    #[test]
    fn test_scrollbar_state_scroll_down() {
        let mut state = ScrollbarState::new(100)
            .position(0)
            .viewport_content_length(10);
        state.scroll_down();
        assert_eq!(state.position, 1);
        state.scroll_down();
        assert_eq!(state.position, 2);
    }

    #[test]
    fn test_scrollbar_state_scroll_up() {
        let mut state = ScrollbarState::new(100)
            .position(25)
            .viewport_content_length(10);
        state.scroll_up();
        assert_eq!(state.position, 24);
        state.scroll_up();
        assert_eq!(state.position, 23);
    }

    #[test]
    fn test_scrollbar_state_scroll_to_top() {
        let mut state = ScrollbarState::new(100)
            .position(50)
            .viewport_content_length(10);
        state.scroll_to_top();
        assert_eq!(state.position, 0);
    }

    #[test]
    fn test_scrollbar_state_scroll_to_bottom() {
        let mut state = ScrollbarState::new(100)
            .position(0)
            .viewport_content_length(10);
        state.scroll_to_bottom();
        assert_eq!(state.position, 90);
    }

    #[test]
    fn test_scrollbar_orientation_default() {
        let orientation = ScrollbarOrientation::default();
        assert_eq!(orientation, ScrollbarOrientation::VerticalRight);
    }

    #[test]
    fn test_scrollbar_new() {
        let scrollbar = Scrollbar::new();
        assert_eq!(scrollbar.orientation, ScrollbarOrientation::VerticalRight);
        assert_eq!(scrollbar.begin_symbol, Some(arrow::UP.to_string()));
        assert_eq!(scrollbar.end_symbol, Some(arrow::DOWN.to_string()));
        assert_eq!(scrollbar.thumb_symbol, block::FULL.to_string());
        assert_eq!(scrollbar.track_symbol, line::VERTICAL.to_string());
    }

    #[test]
    fn test_scrollbar_builder() {
        let scrollbar = Scrollbar::new()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("X")
            .track_symbol("-")
            .style(Style::default().fg(Color::Green));

        assert_eq!(scrollbar.orientation, ScrollbarOrientation::HorizontalBottom);
        assert_eq!(scrollbar.thumb_symbol, "X");
        assert_eq!(scrollbar.track_symbol, "-");
        assert_eq!(scrollbar.style.fg, Some(Color::Green));
    }

    #[test]
    fn test_scrollbar_calculate_thumb() {
        let scrollbar = Scrollbar::new();
        let state = ScrollbarState::new(100)
            .position(0)
            .viewport_content_length(10);

        let (pos, size) = scrollbar.calculate_thumb(20, &state);
        // Thumb size should be proportional: 10/100 * 20 = 2
        assert_eq!(size, 2);
        assert_eq!(pos, 0);
    }

    #[test]
    fn test_scrollbar_calculate_thumb_middle() {
        let scrollbar = Scrollbar::new();
        let state = ScrollbarState::new(100)
            .position(45)
            .viewport_content_length(10);

        let (pos, size) = scrollbar.calculate_thumb(20, &state);
        // Thumb size: 10/100 * 20 = 2
        // Position: 45/90 * 18 = 9
        assert_eq!(size, 2);
        assert_eq!(pos, 9);
    }

    #[test]
    fn test_scrollbar_render_vertical() {
        let scrollbar = Scrollbar::new();
        let mut state = ScrollbarState::new(100)
            .position(0)
            .viewport_content_length(10);

        let area = Rect::new(0, 0, 1, 10);
        let mut buffer = Buffer::new(area);
        scrollbar.render(area, &mut buffer, &mut state);

        // First cell should be begin symbol
        assert_eq!(buffer.get(0, 0).unwrap().symbol, arrow::UP);
        // Last cell should be end symbol
        assert_eq!(buffer.get(0, 9).unwrap().symbol, arrow::DOWN);
    }

    #[test]
    fn test_scrollbar_render_horizontal() {
        let scrollbar = Scrollbar::new()
            .orientation(ScrollbarOrientation::HorizontalBottom);
        let mut state = ScrollbarState::new(100)
            .position(0)
            .viewport_content_length(10);

        let area = Rect::new(0, 0, 10, 1);
        let mut buffer = Buffer::new(area);
        scrollbar.render(area, &mut buffer, &mut state);

        // First cell should be begin symbol (left arrow)
        assert_eq!(buffer.get(0, 0).unwrap().symbol, arrow::LEFT);
        // Last cell should be end symbol (right arrow)
        assert_eq!(buffer.get(9, 0).unwrap().symbol, arrow::RIGHT);
    }
}
