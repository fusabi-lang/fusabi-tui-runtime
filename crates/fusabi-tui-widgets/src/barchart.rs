//! BarChart widget for displaying bar chart visualizations.
//!
//! This module provides a `BarChart` widget that visualizes data using vertical bars
//! with customizable styles and grouping.

use fusabi_tui_core::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::bar,
};
use unicode_width::UnicodeWidthStr;

use crate::widget::Widget;

/// Direction for rendering bars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Vertical bars (bottom to top)
    Vertical,
}

/// A single bar in a bar chart.
///
/// Represents an individual bar with a value, optional label, and styling options.
///
/// # Examples
///
/// ```
/// use fusabi_tui_core::style::{Color, Style};
/// use fusabi_tui_widgets::barchart::Bar;
///
/// let bar = Bar::default()
///     .value(42)
///     .label("Sales")
///     .style(Style::default().fg(Color::Green))
///     .value_style(Style::default().fg(Color::Yellow))
///     .text_value("42");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Bar {
    value: u64,
    label: Option<String>,
    style: Style,
    value_style: Style,
    text_value: Option<String>,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            value: 0,
            label: None,
            style: Style::default(),
            value_style: Style::default(),
            text_value: None,
        }
    }
}

impl Bar {
    /// Creates a new bar with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the numeric value of the bar.
    pub fn value(mut self, value: u64) -> Self {
        self.value = value;
        self
    }

    /// Sets the label displayed below the bar.
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    /// Sets the style for the bar.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the style for the value text displayed on the bar.
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = style;
        self
    }

    /// Sets the text representation of the value.
    pub fn text_value<T>(mut self, text: T) -> Self
    where
        T: Into<String>,
    {
        self.text_value = Some(text.into());
        self
    }
}

/// A group of bars in a bar chart.
///
/// Represents a collection of bars that should be displayed together,
/// optionally with a group label.
///
/// # Examples
///
/// ```
/// use fusabi_tui_core::style::{Color, Style};
/// use fusabi_tui_widgets::barchart::{Bar, BarGroup};
///
/// let group = BarGroup::default()
///     .label("Q1")
///     .bars(&[
///         Bar::default().value(10).style(Style::default().fg(Color::Red)),
///         Bar::default().value(20).style(Style::default().fg(Color::Blue)),
///     ]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BarGroup {
    label: Option<String>,
    bars: Vec<Bar>,
}

impl Default for BarGroup {
    fn default() -> Self {
        Self {
            label: None,
            bars: Vec::new(),
        }
    }
}

impl BarGroup {
    /// Creates a new bar group with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the label for the bar group.
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    /// Sets the bars in the group.
    pub fn bars(mut self, bars: &[Bar]) -> Self {
        self.bars = bars.to_vec();
        self
    }
}

/// A bar chart widget for displaying bar chart visualizations.
///
/// The bar chart can display single bars or groups of bars with customizable
/// width, spacing, and scaling.
///
/// # Examples
///
/// ```
/// use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::{Color, Style}};
/// use fusabi_tui_widgets::{barchart::{Bar, BarChart, Direction}, Widget};
///
/// let data = vec![
///     Bar::default()
///         .value(10)
///         .label("Jan")
///         .style(Style::default().fg(Color::Red)),
///     Bar::default()
///         .value(20)
///         .label("Feb")
///         .style(Style::default().fg(Color::Green)),
/// ];
///
/// let chart = BarChart::default()
///     .data(&data)
///     .bar_width(3)
///     .bar_gap(1)
///     .max_value(30)
///     .direction(Direction::Vertical);
///
/// let area = Rect::new(0, 0, 40, 10);
/// let mut buffer = Buffer::new(area);
/// chart.render(area, &mut buffer);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BarChart {
    data: Vec<Bar>,
    groups: Vec<BarGroup>,
    bar_width: u16,
    bar_gap: u16,
    max_value: Option<u64>,
    direction: Direction,
    bar_style: Style,
    value_style: Style,
}

impl Default for BarChart {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            groups: Vec::new(),
            bar_width: 3,
            bar_gap: 1,
            max_value: None,
            direction: Direction::Vertical,
            bar_style: Style::default(),
            value_style: Style::default(),
        }
    }
}

impl BarChart {
    /// Creates a new bar chart with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the data bars for the chart.
    pub fn data(mut self, data: &[Bar]) -> Self {
        self.data = data.to_vec();
        self
    }

    /// Sets the bar groups for the chart.
    pub fn groups(mut self, groups: &[BarGroup]) -> Self {
        self.groups = groups.to_vec();
        self
    }

    /// Sets the width of each bar in characters.
    pub fn bar_width(mut self, width: u16) -> Self {
        self.bar_width = width.max(1);
        self
    }

    /// Sets the gap between bars in characters.
    pub fn bar_gap(mut self, gap: u16) -> Self {
        self.bar_gap = gap;
        self
    }

    /// Sets the maximum value for scaling.
    ///
    /// If not set, the maximum value in the data will be used.
    pub fn max_value(mut self, max: u64) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Sets the direction for rendering bars.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the default style for bars.
    pub fn bar_style(mut self, style: Style) -> Self {
        self.bar_style = style;
        self
    }

    /// Sets the default style for value text.
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = style;
        self
    }

    /// Calculates the maximum value for scaling.
    fn calculate_max(&self) -> u64 {
        if let Some(max) = self.max_value {
            return max;
        }

        let data_max = self.data.iter().map(|b| b.value).max().unwrap_or(0);
        let groups_max = self
            .groups
            .iter()
            .flat_map(|g| g.bars.iter().map(|b| b.value))
            .max()
            .unwrap_or(0);

        data_max.max(groups_max).max(1)
    }

    /// Scales a value to a height in characters.
    fn scale_to_height(&self, value: u64, max: u64, available_height: u16) -> u16 {
        if max == 0 || available_height == 0 {
            return 0;
        }
        let ratio = value as f64 / max as f64;
        (ratio * available_height as f64).round() as u16
    }

    /// Gets the bar character for a given eighth level (0-8).
    fn get_bar_char(&self, eighths: usize) -> &'static str {
        let index = eighths.min(8);
        bar::VERTICAL_BARS[index]
    }

    /// Renders a single vertical bar.
    fn render_vertical_bar(
        &self,
        bar: &Bar,
        x: u16,
        area: Rect,
        buf: &mut Buffer,
        max_value: u64,
    ) {
        // Calculate available height (leave space for labels)
        let label_height = if bar.label.is_some() { 1 } else { 0 };
        let value_text_height = if bar.text_value.is_some() { 1 } else { 0 };
        let available_height = area
            .height
            .saturating_sub(label_height)
            .saturating_sub(value_text_height);

        if available_height == 0 {
            return;
        }

        // Calculate bar height in eighths
        let total_eighths = (available_height as u64) * 8;
        let bar_eighths = if max_value == 0 {
            0
        } else {
            ((bar.value as f64 / max_value as f64) * total_eighths as f64).round() as u64
        };

        let full_chars = (bar_eighths / 8) as u16;
        let remainder = (bar_eighths % 8) as usize;

        // Determine the style to use
        let bar_style = if bar.style.fg.is_some() || bar.style.bg.is_some() {
            bar.style
        } else {
            self.bar_style
        };

        // Render full bar characters from bottom up
        for i in 0..full_chars {
            let y = area.y + available_height - 1 - i;
            for dx in 0..self.bar_width {
                let cell_x = x + dx;
                if cell_x >= area.right() {
                    break;
                }
                if let Some(cell) = buf.get_mut(cell_x, y) {
                    cell.symbol = bar::FULL.to_string();
                    cell.set_style(bar_style);
                }
            }
        }

        // Render partial bar character if needed
        if remainder > 0 && full_chars < available_height {
            let y = area.y + available_height - 1 - full_chars;
            let partial_char = self.get_bar_char(remainder);
            for dx in 0..self.bar_width {
                let cell_x = x + dx;
                if cell_x >= area.right() {
                    break;
                }
                if let Some(cell) = buf.get_mut(cell_x, y) {
                    cell.symbol = partial_char.to_string();
                    cell.set_style(bar_style);
                }
            }
        }

        // Render value text if present
        if let Some(ref text) = bar.text_value {
            let text_width = text.width();
            if text_width <= self.bar_width as usize {
                let text_x = x + (self.bar_width.saturating_sub(text_width as u16)) / 2;
                let text_y = area.y + available_height;

                let value_style = if bar.value_style.fg.is_some() || bar.value_style.bg.is_some() {
                    bar.value_style
                } else {
                    self.value_style
                };

                for (i, ch) in text.chars().enumerate() {
                    let cell_x = text_x.saturating_add(i as u16);
                    if cell_x >= area.right() {
                        break;
                    }
                    if let Some(cell) = buf.get_mut(cell_x, text_y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(value_style);
                    }
                }
            }
        }

        // Render label if present
        if let Some(ref label) = bar.label {
            let label_width = label.width();
            if label_width <= self.bar_width as usize {
                let label_x = x + (self.bar_width.saturating_sub(label_width as u16)) / 2;
                let label_y = area.y + area.height - 1;

                for (i, ch) in label.chars().enumerate() {
                    let cell_x = label_x.saturating_add(i as u16);
                    if cell_x >= area.right() {
                        break;
                    }
                    if let Some(cell) = buf.get_mut(cell_x, label_y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(Style::default());
                    }
                }
            }
        }
    }
}

impl Widget for BarChart {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        // Clear the area
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = buf.get_mut(x, y) {
                    cell.symbol = " ".to_string();
                    cell.set_style(Style::default());
                }
            }
        }

        let max_value = self.calculate_max();
        let mut current_x = area.x;

        match self.direction {
            Direction::Vertical => {
                // Render individual bars
                for bar in &self.data {
                    if current_x >= area.right() {
                        break;
                    }

                    self.render_vertical_bar(bar, current_x, area, buf, max_value);
                    current_x = current_x.saturating_add(self.bar_width + self.bar_gap);
                }

                // Render bar groups
                for group in &self.groups {
                    if current_x >= area.right() {
                        break;
                    }

                    for bar in &group.bars {
                        if current_x >= area.right() {
                            break;
                        }

                        self.render_vertical_bar(bar, current_x, area, buf, max_value);
                        current_x = current_x.saturating_add(self.bar_width + self.bar_gap);
                    }

                    // Render group label if present
                    if let Some(ref label) = group.label {
                        let group_width = (group.bars.len() as u16)
                            .saturating_mul(self.bar_width + self.bar_gap)
                            .saturating_sub(self.bar_gap);
                        let label_width = label.width();
                        let group_start_x = current_x.saturating_sub(group_width);

                        if label_width <= group_width as usize {
                            let label_x = group_start_x
                                + (group_width.saturating_sub(label_width as u16)) / 2;
                            let label_y = area.y + area.height - 1;

                            for (i, ch) in label.chars().enumerate() {
                                let cell_x = label_x.saturating_add(i as u16);
                                if cell_x >= area.right() {
                                    break;
                                }
                                if let Some(cell) = buf.get_mut(cell_x, label_y) {
                                    cell.symbol = ch.to_string();
                                    cell.set_style(Style::default());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_tui_core::style::Color;

    #[test]
    fn test_bar_new() {
        let bar = Bar::new();
        assert_eq!(bar.value, 0);
        assert_eq!(bar.label, None);
        assert_eq!(bar.text_value, None);
    }

    #[test]
    fn test_bar_builder() {
        let bar = Bar::default()
            .value(42)
            .label("Test")
            .style(Style::default().fg(Color::Red))
            .value_style(Style::default().fg(Color::Green))
            .text_value("42");

        assert_eq!(bar.value, 42);
        assert_eq!(bar.label, Some("Test".to_string()));
        assert_eq!(bar.text_value, Some("42".to_string()));
        assert_eq!(bar.style.fg, Some(Color::Red));
        assert_eq!(bar.value_style.fg, Some(Color::Green));
    }

    #[test]
    fn test_bar_group_new() {
        let group = BarGroup::new();
        assert_eq!(group.label, None);
        assert!(group.bars.is_empty());
    }

    #[test]
    fn test_bar_group_builder() {
        let bars = vec![Bar::default().value(10), Bar::default().value(20)];
        let group = BarGroup::default().label("Group 1").bars(&bars);

        assert_eq!(group.label, Some("Group 1".to_string()));
        assert_eq!(group.bars.len(), 2);
    }

    #[test]
    fn test_barchart_new() {
        let chart = BarChart::new();
        assert!(chart.data.is_empty());
        assert!(chart.groups.is_empty());
        assert_eq!(chart.bar_width, 3);
        assert_eq!(chart.bar_gap, 1);
        assert_eq!(chart.max_value, None);
        assert_eq!(chart.direction, Direction::Vertical);
    }

    #[test]
    fn test_barchart_builder() {
        let data = vec![Bar::default().value(10), Bar::default().value(20)];
        let chart = BarChart::default()
            .data(&data)
            .bar_width(5)
            .bar_gap(2)
            .max_value(100)
            .direction(Direction::Vertical);

        assert_eq!(chart.data.len(), 2);
        assert_eq!(chart.bar_width, 5);
        assert_eq!(chart.bar_gap, 2);
        assert_eq!(chart.max_value, Some(100));
        assert_eq!(chart.direction, Direction::Vertical);
    }

    #[test]
    fn test_barchart_calculate_max_from_data() {
        let data = vec![
            Bar::default().value(10),
            Bar::default().value(50),
            Bar::default().value(30),
        ];
        let chart = BarChart::default().data(&data);
        assert_eq!(chart.calculate_max(), 50);
    }

    #[test]
    fn test_barchart_calculate_max_explicit() {
        let data = vec![Bar::default().value(10), Bar::default().value(20)];
        let chart = BarChart::default().data(&data).max_value(100);
        assert_eq!(chart.calculate_max(), 100);
    }

    #[test]
    fn test_barchart_calculate_max_empty() {
        let chart = BarChart::new();
        assert_eq!(chart.calculate_max(), 1);
    }

    #[test]
    fn test_barchart_scale_to_height() {
        let chart = BarChart::new();
        assert_eq!(chart.scale_to_height(50, 100, 10), 5);
        assert_eq!(chart.scale_to_height(100, 100, 10), 10);
        assert_eq!(chart.scale_to_height(0, 100, 10), 0);
    }

    #[test]
    fn test_barchart_get_bar_char() {
        let chart = BarChart::new();
        assert_eq!(chart.get_bar_char(0), bar::EMPTY);
        assert_eq!(chart.get_bar_char(4), bar::HALF);
        assert_eq!(chart.get_bar_char(8), bar::FULL);
    }

    #[test]
    fn test_barchart_render_empty() {
        let chart = BarChart::new();
        let area = Rect::new(0, 0, 10, 10);
        let mut buffer = Buffer::new(area);
        chart.render(area, &mut buffer);

        // Should just clear the area
        for y in 0..10 {
            for x in 0..10 {
                let cell = buffer.get(x, y).unwrap();
                assert_eq!(cell.symbol, " ");
            }
        }
    }

    #[test]
    fn test_barchart_render_with_data() {
        let data = vec![
            Bar::default()
                .value(50)
                .label("A")
                .style(Style::default().fg(Color::Red)),
            Bar::default()
                .value(100)
                .label("B")
                .style(Style::default().fg(Color::Green)),
        ];

        let chart = BarChart::default()
            .data(&data)
            .bar_width(3)
            .bar_gap(1)
            .max_value(100);

        let area = Rect::new(0, 0, 20, 10);
        let mut buffer = Buffer::new(area);
        chart.render(area, &mut buffer);

        // Check that labels are rendered at the bottom
        assert_eq!(buffer.get(1, 9).unwrap().symbol, "A");
        assert_eq!(buffer.get(5, 9).unwrap().symbol, "B");

        // Check that bars have colors
        // First bar should have some red cells
        let has_red = (0..10).any(|y| {
            (0..3).any(|x| {
                if let Some(cell) = buffer.get(x, y) {
                    cell.fg == Color::Red
                } else {
                    false
                }
            })
        });
        assert!(has_red);
    }

    #[test]
    fn test_bar_width_minimum() {
        let chart = BarChart::default().bar_width(0);
        assert_eq!(chart.bar_width, 1);
    }
}
