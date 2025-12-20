# Fusabi TUI Runtime - Quick Start

Get a TUI application running in under 5 minutes.

## Prerequisites

- Rust 1.75+ (`rustup update stable`)
- A terminal emulator

## Installation

Add fusabi-tui-runtime to your project:

```bash
cargo add fusabi-tui-core fusabi-tui-widgets fusabi-tui-render fusabi-tui-engine
```

Or add to `Cargo.toml`:

```toml
[dependencies]
fusabi-tui-core = "0.2"
fusabi-tui-widgets = "0.2"
fusabi-tui-render = { version = "0.2", features = ["crossterm-backend"] }
fusabi-tui-engine = "0.2"
```

## Hello World

Create `src/main.rs`:

```rust
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::{Color, Style}};
use fusabi_tui_render::prelude::*;
use fusabi_tui_widgets::{block::Block, borders::Borders, paragraph::Paragraph, widget::Widget};
use std::io::stdout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut renderer = CrosstermRenderer::new(stdout())?;
    renderer.show_cursor(false)?;

    loop {
        // Get terminal size
        let size = renderer.size()?;
        let mut buffer = Buffer::new(size);

        // Render UI
        let block = Block::default()
            .title("Hello Fusabi TUI!")
            .borders(Borders::ALL)
            .style(Style::new().fg(Color::Cyan));

        let paragraph = Paragraph::new("Press 'q' to quit")
            .block(block);

        paragraph.render(size, &mut buffer);

        // Draw to screen
        renderer.draw(&buffer)?;
        renderer.flush()?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Cleanup
    renderer.show_cursor(true)?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
```

Run it:

```bash
cargo run
```

## Hot Reload Development

For dashboard development with hot reload:

```rust
use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::test::TestRenderer;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = TestRenderer::new(80, 24);
    let mut engine = DashboardEngine::new(renderer, PathBuf::from("."));

    // Enable hot reload (watches for file changes)
    engine.enable_hot_reload_with_debounce(200)?;

    // Load your dashboard
    engine.load(std::path::Path::new("dashboard.fsx"))?;

    // Main loop
    loop {
        // Check for file changes
        if let Some(changes) = engine.poll_changes() {
            if !changes.is_empty() {
                engine.reload()?;
            }
        }

        // Render
        engine.render()?;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
```

## Next Steps

- **[Tutorial: Building Your First App](tutorials/01-hello-world.md)** - Step-by-step guide
- **[Widget Gallery](../crates/fusabi-tui-widgets/README.md)** - Available widgets
- **[Examples](../crates/fusabi-tui-engine/examples/)** - Working example code
- **[FSX Dashboards](../fsx/examples/)** - Fusabi script examples

## Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                   Your Application                   │
├─────────────────────────────────────────────────────┤
│  fusabi-tui-engine (hot reload, state, file watch)  │
├─────────────────────────────────────────────────────┤
│  fusabi-tui-widgets (Block, List, Table, Gauge...)  │
├─────────────────────────────────────────────────────┤
│  fusabi-tui-render (Crossterm, Test, Scarab)        │
├─────────────────────────────────────────────────────┤
│  fusabi-tui-core (Buffer, Cell, Layout, Style)      │
└─────────────────────────────────────────────────────┘
```

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/fusabi-lang/fusabi-tui-runtime/issues)
- **Examples**: Check the `examples/` directory in each crate
- **API Docs**: `cargo doc --open`
