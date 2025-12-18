//! Clear widget for resetting terminal buffer areas.
//!
//! This module provides a `Clear` widget that resets all cells in a rectangular
//! area to their default state (empty space with no styling).

use fusabi_tui_core::{buffer::Buffer, layout::Rect};

use crate::widget::Widget;

/// A widget that clears a rectangular area of the terminal buffer.
///
/// The `Clear` widget resets all cells in the specified area to their default state,
/// effectively clearing any previously rendered content.
///
/// # Examples
///
/// ```rust
/// use fusabi_tui_core::{buffer::Buffer, layout::Rect};
/// use fusabi_tui_widgets::{Clear, Widget};
///
/// let mut buffer = Buffer::new(Rect::new(0, 0, 10, 10));
/// let area = Rect::new(2, 2, 5, 3);
///
/// // Clear the specified area
/// Clear.render(area, &mut buffer);
/// ```
pub struct Clear;

impl Widget for Clear {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                if let Some(cell) = buf.get_mut(x, y) {
                    cell.reset();
                }
            }
        }
    }
}
