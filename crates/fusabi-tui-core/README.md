# fusabi-tui-core

Core TUI primitives for the Fusabi framework.

This crate provides the foundational building blocks for creating terminal user interfaces in the Fusabi ecosystem. It includes primitives for styling, layout, buffering, and rendering terminal content.

## Features

- **Cell**: Individual character cells with styling information
- **Buffer**: 2D grid of cells for efficient rendering
- **Layout**: Constraint-based layout system using Cassowary solver
- **Style**: Rich text styling with colors and modifiers
- **Symbols**: Unicode characters for drawing borders and UI elements

## Design Philosophy

This crate is designed to be:

- **Lightweight**: Minimal dependencies and overhead
- **Type-safe**: Leveraging Rust's type system for correctness
- **Composable**: Small, focused primitives that combine well
- **Performance-oriented**: Zero-cost abstractions where possible

## Quick Start

```rust
use fusabi_tui_core::{
    buffer::{Buffer, Cell},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
};

// Create a buffer for a terminal area
let area = Rect::new(0, 0, 80, 24);
let mut buffer = Buffer::new(area);

// Create a styled cell
let style = Style::new()
    .fg(Color::Green)
    .bg(Color::Black)
    .add_modifier(Modifier::BOLD);

// Write text to the buffer
buffer.set_string(0, 0, "Hello, Fusabi!", style);

// Split the area into sections
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(&[
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(area);
```

## Modules

### buffer

The buffer module provides types for managing a 2D grid of terminal cells:

- **Cell**: Represents a single character cell with a symbol, foreground color, background color, and modifiers
- **Buffer**: A rectangular grid of cells that can be efficiently rendered

### layout

The layout module provides a constraint-based layout system:

- **Rect**: Represents a rectangular area with position and size
- **Layout**: Constraint-based layout calculator using the Cassowary algorithm
- **Constraint**: Various constraint types (Min, Max, Length, Percentage, Ratio, Fill)
- **Direction**: Layout direction (Horizontal, Vertical)

### style

The style module provides rich text styling capabilities:

- **Style**: Combines foreground color, background color, and modifiers
- **Color**: Terminal colors (16 basic colors, 256-color palette, RGB)
- **Modifier**: Text modifiers (BOLD, ITALIC, UNDERLINED, etc.)

### symbols

The symbols module provides Unicode characters for drawing UI elements:

- Border sets for various border styles
- Block symbols for progress bars and charts
- Line symbols for drawing lines and arrows

## License

Licensed under either of:

- MIT license or http://opensource.org/licenses/MIT
- Apache License, Version 2.0 or http://www.apache.org/licenses/LICENSE-2.0

at your option.
