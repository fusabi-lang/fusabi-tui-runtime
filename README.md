# fusabi-tui-runtime

A unified TUI (Text User Interface) framework for Fusabi applications that replaces ratatui dependencies, enabling apps to run standalone or as Scarab terminal plugins.

## Overview

fusabi-tui-runtime provides a complete TUI framework built specifically for Fusabi, allowing you to:

- **Run standalone** via terminal (using crossterm backend)
- **Run as Scarab plugins** via shared memory IPC
- **Hot-reload** during development for rapid iteration
- **Share widgets and themes** via package registry

This framework borrows proven core types from ratatui (~2K LOC) while replacing the terminal layer with a flexible renderer abstraction that supports multiple backends.

## Crates

The workspace consists of 5 crates:

### fusabi-tui-core

Core TUI primitives providing foundational types:
- `Cell` - Individual character cell with styling
- `Buffer` - 2D grid of cells for rendering
- `Rect` - Rectangular regions for layout
- `Layout`, `Constraint` - Flexible layout system
- `Style`, `Color` - Text styling and theming

**Purpose**: Stable, minimal core types borrowed from ratatui-core

### fusabi-tui-render

Renderer abstraction layer supporting multiple backends:
- `Terminal<R>` - High-level terminal wrapper (matches ratatui's API)
- `Frame` - Frame for rendering widgets within draw callback
- `Renderer` trait - Backend-agnostic rendering interface
- `CrosstermRenderer` - Standalone terminal rendering via crossterm
- `ScarabRenderer` - Shared memory rendering for Scarab plugins
- `TestRenderer` - In-memory rendering for testing

**Purpose**: Decouple rendering logic from specific terminal implementations

### fusabi-tui-widgets

Widget library for building TUI interfaces:
- Text rendering and formatting
- Layout containers (blocks, lists, tables)
- Interactive components (input fields, selection menus)
- Custom widget traits for extensibility

**Purpose**: Reusable UI components compatible with Fusabi hot-reload

### fusabi-tui-engine

Hot-reload engine and dashboard runtime:
- File watching for `.fsx` script changes
- State preservation across reloads
- Development overlay with diagnostics
- Async tokio integration

**Purpose**: Developer experience and runtime infrastructure

### fusabi-tui-scarab

Scarab shared memory backend:
- Zero-copy IPC via `bytemuck` and `shared_memory`
- Compatible with Scarab's split-process architecture
- Plugin system integration
- `#[repr(C)]` memory layout guarantees

**Purpose**: Run Fusabi TUI apps inside Scarab terminal emulator

## Quick Start

### Standalone TUI Application

```bash
# Create new TUI app (when fpm is available)
fpm new --template tui-app my-dashboard

# Run with hot-reload
cd my-dashboard
fpm dev
```

### Scarab Plugin

```bash
# Create plugin
fpm new --template scarab-plugin my-plugin

# Build and install
fpm build --release
scarab-daemon --load-plugin ./target/release/my-plugin.fzb
```

### Manual Rust Integration

```rust
use fusabi_tui_render::prelude::*;
use fusabi_tui_widgets::paragraph::Paragraph;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create crossterm-backed terminal
    let renderer = CrosstermRenderer::new(io::stdout())?;
    let mut terminal = Terminal::new(renderer)?;

    // Draw using Frame (matches ratatui's API pattern)
    terminal.draw(|frame| {
        let area = frame.size();
        let greeting = Paragraph::new("Hello, Fusabi TUI!");
        frame.render_widget(greeting, area);
    })?;

    Ok(())
}
```

### Low-Level Buffer Access

For fine-grained control, you can work directly with buffers:

```rust
use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::Style};
use fusabi_tui_render::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = CrosstermRenderer::new(std::io::stdout())?;
    let mut buffer = Buffer::new(Rect::new(0, 0, 80, 24));

    // Direct buffer manipulation
    buffer.set_string(0, 0, "Direct buffer access", Style::default());

    renderer.draw(&buffer)?;
    renderer.flush()?;
    Ok(())
}
```

## Migrating from Ratatui

fusabi-tui-runtime provides a compatible API for projects migrating from ratatui:

### Import Changes

```rust
// Before (ratatui)
use ratatui::{Terminal, Frame};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

// After (fusabi-tui-runtime)
use fusabi_tui_render::prelude::*;
use fusabi_tui_render::Terminal;
use fusabi_tui_core::layout::{Constraint, Direction, Layout, Rect};
use fusabi_tui_core::style::{Color, Modifier, Style};
use fusabi_tui_widgets::{
    block::Block, borders::Borders, paragraph::Paragraph, tabs::Tabs,
};
```

### Terminal Setup

```rust
// Before (ratatui)
let stdout = io::stdout();
let backend = CrosstermBackend::new(stdout);
let mut terminal = Terminal::new(backend)?;

// After (fusabi-tui-runtime)
let renderer = CrosstermRenderer::new(io::stdout())?;
let mut terminal = Terminal::new(renderer)?;
```

### Draw Pattern (unchanged)

The draw pattern is identical:

```rust
terminal.draw(|frame| {
    let area = frame.size();
    let widget = Paragraph::new("Hello!");
    frame.render_widget(widget, area);
})?;
```

### Key Differences

| ratatui | fusabi-tui-runtime | Notes |
|---------|-------------------|-------|
| `CrosstermBackend` | `CrosstermRenderer` | Same functionality |
| `backend_mut()` | `backend_mut().writer_mut()` | Access underlying writer |
| `Color::Gray` | `Color::DarkGray` | Color naming |
| `Widget::render(self, ...)` | `Widget::render(&self, ...)` | Takes reference |

## Build Instructions

### Build entire workspace

```bash
cargo build
```

### Build with optimizations

```bash
cargo build --release
```

### Build specific crate

```bash
cargo build -p fusabi-tui-core
cargo build -p fusabi-tui-render
cargo build -p fusabi-tui-widgets
cargo build -p fusabi-tui-engine
cargo build -p fusabi-tui-scarab
```

## Testing

### Run all tests

```bash
cargo test --workspace
```

### Test specific crate

```bash
cargo test -p fusabi-tui-core
```

### Check for compilation errors

```bash
cargo check --workspace
```

## Architecture

### Renderer Abstraction

The framework uses a trait-based renderer abstraction that supports multiple backends:

```
┌─────────────────────────┐
│   Fusabi TUI App        │
│   (.fsx scripts)        │
└───────────┬─────────────┘
            │
┌───────────▼─────────────┐
│   fusabi-tui-widgets    │
│   (UI Components)       │
└───────────┬─────────────┘
            │
┌───────────▼─────────────┐
│   fusabi-tui-core       │
│   (Buffer, Layout)      │
└───────────┬─────────────┘
            │
┌───────────▼─────────────┐
│   fusabi-tui-render     │
│   (Renderer Trait)      │
└───────────┬─────────────┘
            │
    ┌───────┴───────┐
    │               │
┌───▼────┐    ┌─────▼──────┐
│Crossterm│    │  Scarab    │
│Renderer │    │  Renderer  │
└────────┘    └────────────┘
```

### Dual-Mode Runtime

The same Fusabi `.fsx` code can run in two modes:

1. **Standalone**: Using CrosstermRenderer, runs in any terminal
2. **Plugin**: Using ScarabRenderer, runs inside Scarab with shared memory

This allows seamless migration between development (standalone) and production (plugin).

## Key Features

### Hot Reload

The engine supports hot-reloading `.fsx` scripts without restarting:

```fsharp
// dashboard.fsx - automatically reloaded on save
let render state =
    Text("CPU: " + formatPercent state.cpu)
    |> renderAt (0, 0)
```

### Lock-Free Synchronization

Scarab backend uses atomic operations for non-blocking reads:

```rust
#[repr(C)]
pub struct SharedState {
    sequence_number: AtomicU64,
    grid: [[Cell; 200]; 100],
}
```

### Type Safety

All shared memory structs use `#[repr(C)]` and `bytemuck` for safe zero-copy:

```rust
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub fg: u32,
    pub bg: u32,
}
```

## Project Status

| Component | Status |
|-----------|--------|
| Documentation | Complete |
| Core Types | Implemented |
| Renderers | Implemented (Crossterm, Test) |
| Terminal/Frame API | Implemented |
| Widget System | Implemented |
| Hot Reload Engine | In Progress |
| Scarab Backend | In Progress |

### Migrated Projects

| Project | Status | PR |
|---------|--------|-----|
| Hibana | Complete | [#27](https://github.com/raibid-labs/hibana/pull/27) |
| Scryforge | Complete | Uses crates.io |
| Phage | Complete | Uses crates.io |
| Sigilforge | Complete | Uses crates.io |
| Scarab | Complete | Uses crates.io |

## Related Projects

- [Fusabi](https://github.com/fusabi-lang/fusabi) - F# language runtime for Rust
- [Scarab](https://github.com/raibid-labs/scarab) - Split-process terminal emulator
- [Scryforge](https://github.com/raibid-labs/scryforge) - Information aggregator (migration target)
- [ratatui](https://github.com/ratatui-org/ratatui) - Original inspiration and core type donor

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- [Ecosystem Overview](docs/ECOSYSTEM_OVERVIEW.md) - Full ecosystem map
- [Architecture](docs/architecture/OVERVIEW.md) - Design decisions
- [Ratatui Analysis](docs/architecture/RATATUI_ANALYSIS.md) - What we borrow vs. build
- [Fusabi Enhancements](docs/design/FUSABI_ENHANCEMENTS.md) - Language gaps
- [Hot Reload Dashboard](docs/design/HOT_RELOAD_DASHBOARD.md) - Hot-reload architecture
- [Package Management](docs/design/PACKAGE_MANAGEMENT.md) - Developer experience

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

Contributions are welcome! This project is in active development. Please check the [documentation](docs/) for architecture details before submitting PRs.
