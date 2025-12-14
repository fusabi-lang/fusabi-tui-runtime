# fusabi-tui-widgets

Widget library for building TUI interfaces in the Fusabi framework.

This crate provides a collection of reusable widgets for creating terminal user interfaces. It builds on top of `fusabi-tui-core` to provide higher-level abstractions for common UI patterns.

## Features

- **Block**: Bordered containers with titles
- **Paragraph**: Multi-line text with word wrapping
- **List**: Scrollable lists with selection
- **Table**: Tabular data display with headers
- **Gauge**: Progress bars and meters
- **Sparkline**: Inline mini-charts
- **Tabs**: Tab navigation
- **Text**: Rich text with styled spans

## Design Philosophy

Widgets in this crate are designed to be:

- **Composable**: Small widgets can be combined to build complex UIs
- **Immutable**: Widgets use builder patterns and don't mutate state
- **Efficient**: Rendering is done through zero-copy buffers
- **Flexible**: Extensive styling and customization options

## Quick Start

```rust
use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::{Color, Style}};
use fusabi_tui_widgets::{
    block::Block,
    borders::{Borders, BorderType},
    widget::Widget,
};

// Create a block with borders
let block = Block::default()
    .title("My Panel")
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded);

// Render it to a buffer
let area = Rect::new(0, 0, 20, 10);
let mut buffer = Buffer::new(area);
block.render(area, &mut buffer);
```

## Widget Types

### Block

A bordered container that can hold other widgets:

```rust
use fusabi_tui_widgets::{Block, Borders, BorderType, Widget};
use fusabi_tui_core::style::{Color, Modifier, Style};

let block = Block::default()
    .title("Title")
    .title_alignment(TitleAlignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(Style::new().fg(Color::Cyan))
    .style(Style::new().bg(Color::Black));

block.render(area, &mut buffer);
```

### Paragraph

Multi-line text with word wrapping and alignment:

```rust
use fusabi_tui_widgets::{Paragraph, Alignment, Wrap, Text, Widget};

let text = Text::from(vec![
    Line::from("First line"),
    Line::from("Second line"),
]);

let paragraph = Paragraph::new(text)
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

paragraph.render(area, &mut buffer);
```

### List

Scrollable list with selection state:

```rust
use fusabi_tui_widgets::{List, ListItem, ListState, StatefulWidget};
use fusabi_tui_core::style::{Color, Modifier, Style};

let items = vec![
    ListItem::new("Item 1"),
    ListItem::new("Item 2"),
    ListItem::new("Item 3"),
];

let list = List::new(items)
    .highlight_style(Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

let mut state = ListState::default();
state.select(Some(0));

list.render(area, &mut buffer, &mut state);
```

### Table

Tabular data with headers and column constraints:

```rust
use fusabi_tui_widgets::{Table, Row, TableCell, TableState, StatefulWidget};
use fusabi_tui_core::layout::Constraint;

let header = Row::new(vec!["Name", "Age", "City"]);
let rows = vec![
    Row::new(vec!["Alice", "30", "NYC"]),
    Row::new(vec!["Bob", "25", "LA"]),
];

let table = Table::new(rows)
    .header(header)
    .widths(&[
        Constraint::Percentage(40),
        Constraint::Length(5),
        Constraint::Fill(1),
    ])
    .column_spacing(2);

let mut state = TableState::default();
table.render(area, &mut buffer, &mut state);
```

### Gauge

Progress bars and meters:

```rust
use fusabi_tui_widgets::{Gauge, Widget};
use fusabi_tui_core::style::{Color, Style};

let gauge = Gauge::default()
    .percent(75)
    .label("75%")
    .gauge_style(Style::new().fg(Color::Green).bg(Color::Black));

gauge.render(area, &mut buffer);
```

### Sparkline

Inline mini-charts for visualizing data trends:

```rust
use fusabi_tui_widgets::{Sparkline, Widget};
use fusabi_tui_core::style::{Color, Style};

let data = vec![0, 2, 3, 4, 1, 4, 10, 8, 6, 4, 2, 1];

let sparkline = Sparkline::default()
    .data(&data)
    .style(Style::new().fg(Color::Cyan))
    .max(10);

sparkline.render(area, &mut buffer);
```

### Tabs

Tab navigation interface:

```rust
use fusabi_tui_widgets::{Tabs, Widget};
use fusabi_tui_core::style::{Color, Modifier, Style};

let titles = vec!["Tab 1", "Tab 2", "Tab 3"];

let tabs = Tabs::new(titles)
    .select(1)
    .highlight_style(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    .divider("|");

tabs.render(area, &mut buffer);
```

## Widget and StatefulWidget Traits

### Widget Trait

For stateless widgets:

```rust
pub trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer);
}
```

Example custom widget:

```rust
use fusabi_tui_widgets::Widget;
use fusabi_tui_core::{Buffer, Rect, style::Style};

struct MyWidget {
    text: String,
}

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.x, area.y, &self.text, Style::default());
    }
}
```

### StatefulWidget Trait

For widgets with state:

```rust
pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
```

Example with state:

```rust
use fusabi_tui_widgets::StatefulWidget;
use fusabi_tui_core::{Buffer, Rect, style::Style};

struct CounterWidget;

struct CounterState {
    count: u32,
}

impl StatefulWidget for CounterWidget {
    type State = CounterState;
    
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let text = format!("Count: {}", state.count);
        buf.set_string(area.x, area.y, &text, Style::default());
    }
}
```

## Styled Text

The `Text`, `Line`, and `Span` types provide rich text formatting:

```rust
use fusabi_tui_widgets::{Text, Line, Span};
use fusabi_tui_core::style::{Color, Modifier, Style};

// Create styled spans
let span1 = Span::styled("Bold ", Style::new().add_modifier(Modifier::BOLD));
let span2 = Span::styled("Red", Style::new().fg(Color::Red));

// Combine into a line
let line = Line::from(vec![span1, span2]);

// Create multi-line text
let text = Text::from(vec![
    Line::from("First line"),
    line,
    Line::from("Third line"),
]);
```

## Borders

Various border styles are available:

```rust
use fusabi_tui_widgets::{BorderType, Borders};

// Border types
BorderType::Plain
BorderType::Rounded
BorderType::Double
BorderType::Thick
BorderType::QuadrantInside
BorderType::QuadrantOutside

// Border flags (can be combined)
Borders::NONE
Borders::TOP
Borders::RIGHT
Borders::BOTTOM
Borders::LEFT
Borders::ALL
```

## Layout Integration

Widgets work seamlessly with the layout system:

```rust
use fusabi_tui_core::layout::{Constraint, Direction, Layout};
use fusabi_tui_widgets::{Block, Borders, Paragraph, Widget};

let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(&[
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .split(area);

// Header
let header = Block::default()
    .title("Header")
    .borders(Borders::ALL);
header.render(chunks[0], &mut buffer);

// Body
let body = Paragraph::new("Content");
body.render(chunks[1], &mut buffer);
```

## Examples

### Dashboard Layout

```rust
use fusabi_tui_core::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
};
use fusabi_tui_widgets::{Block, Borders, Gauge, Paragraph, Widget};

let area = Rect::new(0, 0, 80, 24);
let mut buffer = Buffer::new(area);

let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(&[
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(area);

// Header
let header = Block::default()
    .title("System Monitor")
    .borders(Borders::ALL);
header.render(chunks[0], &mut buffer);

// CPU gauge
let cpu_gauge = Gauge::default()
    .percent(75)
    .label("CPU: 75%")
    .gauge_style(Style::new().fg(Color::Green));
cpu_gauge.render(chunks[1], &mut buffer);

// Footer
let footer = Paragraph::new("Press 'q' to quit");
footer.render(chunks[2], &mut buffer);
```

## Testing

Widgets are easy to test:

```rust
use fusabi_tui_core::{buffer::Buffer, layout::Rect};
use fusabi_tui_widgets::{Block, Borders, Widget};

#[test]
fn test_block_rendering() {
    let mut buffer = Buffer::new(Rect::new(0, 0, 10, 5));
    
    let block = Block::default()
        .title("Test")
        .borders(Borders::ALL);
    
    block.render(buffer.area, &mut buffer);
    
    // Verify border characters
    assert_eq!(buffer.get(0, 0).unwrap().symbol(), "┌");
    assert_eq!(buffer.get(9, 0).unwrap().symbol(), "┐");
}
```

## Integration with Other Crates

This crate is designed to work seamlessly with:

- **fusabi-tui-core**: Provides Buffer, Rect, and Style types
- **fusabi-tui-render**: Renders buffers to terminal or shared memory
- **fusabi-tui-engine**: Manages widget lifecycle and events
- **fusabi-tui-scarab**: Enables running as Scarab plugins

## License

Licensed under either of:

- MIT license or http://opensource.org/licenses/MIT
- Apache License, Version 2.0 or http://www.apache.org/licenses/LICENSE-2.0

at your option.
