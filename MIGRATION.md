# Migration Guide

This guide helps you migrate to `fusabi-tui-runtime` from either `ratatui` or the old `fusabi-tui` framework.

## Table of Contents

- [Migrating from Ratatui](#migrating-from-ratatui)
- [Migrating from Old fusabi-tui](#migrating-from-old-fusabi-tui)
- [Common Patterns](#common-patterns)
- [Import Mappings](#import-mappings)

## Migrating from Ratatui

### Overview

`fusabi-tui-runtime` borrows core types from `ratatui-core` (~2K LOC) but replaces the terminal layer with a flexible renderer abstraction. This allows the same code to run standalone (via crossterm) or as a Scarab plugin (via shared memory).

### Key Differences

| Aspect | Ratatui | fusabi-tui-runtime |
|--------|---------|-------------------|
| Terminal abstraction | `Terminal<Backend>` | `Renderer` trait |
| Backend trait | Iterator-based, synchronous | Buffer-based, async-ready |
| Event handling | External (crossterm, termion) | Built into renderer |
| Hot reload | Not supported | Native support via engine |
| Plugin mode | Not supported | Native Scarab integration |

### Step-by-Step Migration

#### 1. Update Dependencies

**Before (Cargo.toml):**
```toml
[dependencies]
ratatui = "0.30"
crossterm = "0.28"
```

**After (Cargo.toml):**
```toml
[dependencies]
fusabi-tui-core = "0.1"
fusabi-tui-render = { version = "0.1", features = ["crossterm-backend"] }
fusabi-tui-widgets = "0.1"
```

#### 2. Replace Terminal Setup

**Before (ratatui):**
```rust
use ratatui::prelude::*;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Your app code...

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
```

**After (fusabi-tui-runtime):**
```rust
use fusabi_tui_render::prelude::*;
use fusabi_tui_core::{Buffer, Rect};
use std::io;

fn main() -> io::Result<()> {
    let mut renderer = CrosstermRenderer::new(io::stdout())?;
    let size = renderer.size()?;
    let mut buffer = Buffer::new(Rect::new(0, 0, size.width, size.height));

    // Your app code...

    renderer.cleanup()?;
    Ok(())
}
```

#### 3. Update Rendering Code

**Before (ratatui):**
```rust
terminal.draw(|f| {
    let area = f.area();
    let block = Block::default()
        .title("My App")
        .borders(Borders::ALL);
    f.render_widget(block, area);
})?;
```

**After (fusabi-tui-runtime):**
```rust
use fusabi_tui_widgets::{Block, Borders, Widget};

let size = renderer.size()?;
let mut buffer = Buffer::new(Rect::new(0, 0, size.width, size.height));

let block = Block::default()
    .title("My App")
    .borders(Borders::ALL);

block.render(buffer.area, &mut buffer);
renderer.draw(&buffer)?;
renderer.flush()?;
```

#### 4. Replace Event Handling

**Before (ratatui with crossterm):**
```rust
use crossterm::event::{self, Event, KeyCode};

loop {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
}
```

**After (fusabi-tui-runtime):**
```rust
use fusabi_tui_engine::prelude::*;
use std::time::Duration;

loop {
    if let Some(event) = renderer.poll_event(Duration::from_millis(100)) {
        match event {
            Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => break,
            _ => {}
        }
    }
}
```

#### 5. Update Widget Usage

Most widgets work the same way, but the trait signature changes slightly:

**Before (ratatui):**
```rust
impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ...
    }
}
```

**After (fusabi-tui-runtime):**
```rust
use fusabi_tui_widgets::Widget;
use fusabi_tui_core::{Buffer, Rect};

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Same implementation
    }
}
```

### Advanced Features

#### Hot Reload (fusabi-tui-runtime exclusive)

```rust
use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::CrosstermRenderer;
use std::path::PathBuf;

let renderer = CrosstermRenderer::new(io::stdout())?;
let mut engine = DashboardEngine::new(renderer, PathBuf::from("."));

// Enable hot reload with 200ms debounce
engine.enable_hot_reload_with_debounce(200)?;

// Load dashboard script
engine.load(Path::new("dashboard.fsx"))?;

// Main loop with auto-reload
loop {
    // Check for file changes
    if let Some(changes) = engine.poll_changes() {
        if !changes.is_empty() {
            engine.reload()?;
        }
    }

    engine.render()?;
    std::thread::sleep(Duration::from_millis(16));
}
```

#### Scarab Plugin Mode (fusabi-tui-runtime exclusive)

```rust
use fusabi_tui_scarab::prelude::*;

// Connect to Scarab's shared memory
let mut renderer = ScarabRenderer::connect(None)?;

// Rest of the code is identical to crossterm mode
let size = renderer.size()?;
let mut buffer = Buffer::new(size);

// Render to shared memory instead of terminal
block.render(buffer.area, &mut buffer);
renderer.draw(&buffer)?;
renderer.flush()?;
```

## Migrating from Old fusabi-tui

If you're migrating from an older version of `fusabi-tui`, the main changes are organizational:

### Crate Structure Changes

**Before:**
```toml
[dependencies]
fusabi-tui = "0.1"
```

**After:**
```toml
[dependencies]
fusabi-tui-core = "0.1"
fusabi-tui-render = { version = "0.1", features = ["crossterm-backend"] }
fusabi-tui-widgets = "0.1"
fusabi-tui-engine = "0.1"  # Optional, for hot reload
fusabi-tui-scarab = "0.1"  # Optional, for Scarab plugins
```

### Import Changes

**Before:**
```rust
use fusabi_tui::{Buffer, Cell, Rect, Style, Color};
use fusabi_tui::widgets::Block;
use fusabi_tui::renderer::Renderer;
```

**After:**
```rust
use fusabi_tui_core::{Buffer, Cell, Rect, Style, Color};
use fusabi_tui_widgets::Block;
use fusabi_tui_render::prelude::*;
```

## Common Patterns

### Pattern 1: Simple Text Display

**Ratatui:**
```rust
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Block, Borders};

terminal.draw(|f| {
    let text = Paragraph::new("Hello, World!")
        .block(Block::default()
            .title("Title")
            .borders(Borders::ALL));
    f.render_widget(text, f.area());
})?;
```

**fusabi-tui-runtime:**
```rust
use fusabi_tui_core::{Buffer, Rect};
use fusabi_tui_widgets::{Paragraph, Block, Borders, Widget};
use fusabi_tui_render::prelude::*;

let size = renderer.size()?;
let mut buffer = Buffer::new(Rect::new(0, 0, size.width, size.height));

let text = Paragraph::new("Hello, World!")
    .block(Block::default()
        .title("Title")
        .borders(Borders::ALL));

text.render(buffer.area, &mut buffer);
renderer.draw(&buffer)?;
renderer.flush()?;
```

### Pattern 2: Layout Splitting

**Ratatui:**
```rust
use ratatui::prelude::*;

terminal.draw(|f| {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    f.render_widget(header, chunks[0]);
    f.render_widget(body, chunks[1]);
    f.render_widget(footer, chunks[2]);
})?;
```

**fusabi-tui-runtime:**
```rust
use fusabi_tui_core::{Buffer, Layout, Constraint, Direction};
use fusabi_tui_widgets::Widget;

let size = renderer.size()?;
let area = Rect::new(0, 0, size.width, size.height);
let mut buffer = Buffer::new(area);

let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(area);

header.render(chunks[0], &mut buffer);
body.render(chunks[1], &mut buffer);
footer.render(chunks[2], &mut buffer);

renderer.draw(&buffer)?;
renderer.flush()?;
```

### Pattern 3: Stateful Widgets

**Ratatui:**
```rust
use ratatui::prelude::*;
use ratatui::widgets::{List, ListItem, ListState};

let mut state = ListState::default();
state.select(Some(0));

terminal.draw(|f| {
    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
    ];
    let list = List::new(items);
    f.render_stateful_widget(list, f.area(), &mut state);
})?;
```

**fusabi-tui-runtime:**
```rust
use fusabi_tui_core::Buffer;
use fusabi_tui_widgets::{List, ListItem, ListState, StatefulWidget};

let mut state = ListState::default();
state.select(Some(0));

let size = renderer.size()?;
let mut buffer = Buffer::new(Rect::new(0, 0, size.width, size.height));

let items = vec![
    ListItem::new("Item 1"),
    ListItem::new("Item 2"),
];
let list = List::new(items);

list.render(buffer.area, &mut buffer, &mut state);
renderer.draw(&buffer)?;
renderer.flush()?;
```

### Pattern 4: Custom Widgets

**Ratatui:**
```rust
use ratatui::prelude::*;

struct MyWidget {
    title: String,
}

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.x, area.y, &self.title, Style::default());
    }
}
```

**fusabi-tui-runtime:**
```rust
use fusabi_tui_core::{Buffer, Rect, Style};
use fusabi_tui_widgets::Widget;

struct MyWidget {
    title: String,
}

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.x, area.y, &self.title, Style::default());
    }
}
```

## Import Mappings

### Core Types

| Ratatui | fusabi-tui-runtime |
|---------|-------------------|
| `ratatui::buffer::Buffer` | `fusabi_tui_core::buffer::Buffer` |
| `ratatui::buffer::Cell` | `fusabi_tui_core::buffer::Cell` |
| `ratatui::layout::Rect` | `fusabi_tui_core::layout::Rect` |
| `ratatui::layout::Layout` | `fusabi_tui_core::layout::Layout` |
| `ratatui::layout::Constraint` | `fusabi_tui_core::layout::Constraint` |
| `ratatui::layout::Direction` | `fusabi_tui_core::layout::Direction` |
| `ratatui::style::Style` | `fusabi_tui_core::style::Style` |
| `ratatui::style::Color` | `fusabi_tui_core::style::Color` |
| `ratatui::style::Modifier` | `fusabi_tui_core::style::Modifier` |

### Widgets

| Ratatui | fusabi-tui-runtime |
|---------|-------------------|
| `ratatui::widgets::Widget` | `fusabi_tui_widgets::Widget` |
| `ratatui::widgets::StatefulWidget` | `fusabi_tui_widgets::StatefulWidget` |
| `ratatui::widgets::Block` | `fusabi_tui_widgets::Block` |
| `ratatui::widgets::Borders` | `fusabi_tui_widgets::Borders` |
| `ratatui::widgets::BorderType` | `fusabi_tui_widgets::BorderType` |
| `ratatui::widgets::Paragraph` | `fusabi_tui_widgets::Paragraph` |
| `ratatui::widgets::List` | `fusabi_tui_widgets::List` |
| `ratatui::widgets::ListItem` | `fusabi_tui_widgets::ListItem` |
| `ratatui::widgets::ListState` | `fusabi_tui_widgets::ListState` |
| `ratatui::widgets::Table` | `fusabi_tui_widgets::Table` |
| `ratatui::widgets::Row` | `fusabi_tui_widgets::Row` |
| `ratatui::widgets::TableState` | `fusabi_tui_widgets::TableState` |
| `ratatui::widgets::Gauge` | `fusabi_tui_widgets::Gauge` |
| `ratatui::widgets::Sparkline` | `fusabi_tui_widgets::Sparkline` |
| `ratatui::widgets::Tabs` | `fusabi_tui_widgets::Tabs` |
| `ratatui::text::Text` | `fusabi_tui_widgets::Text` |
| `ratatui::text::Line` | `fusabi_tui_widgets::Line` |
| `ratatui::text::Span` | `fusabi_tui_widgets::Span` |

### Terminal and Backend

| Ratatui | fusabi-tui-runtime |
|---------|-------------------|
| `ratatui::Terminal` | No direct equivalent (use `Renderer` directly) |
| `ratatui::backend::Backend` | `fusabi_tui_render::renderer::Renderer` |
| `ratatui::backend::CrosstermBackend` | `fusabi_tui_render::crossterm::CrosstermRenderer` |
| N/A | `fusabi_tui_scarab::renderer::ScarabRenderer` |
| N/A | `fusabi_tui_render::test::TestRenderer` |

### Convenience Imports

For quick setup, use prelude modules:

```rust
// Core types
use fusabi_tui_core::prelude::*;  // Buffer, Cell, Rect, Layout, Style, Color

// Renderer
use fusabi_tui_render::prelude::*;  // Renderer, CrosstermRenderer, Result

// Widgets
use fusabi_tui_widgets::*;  // All widgets

// Engine (for hot reload)
use fusabi_tui_engine::prelude::*;  // DashboardEngine, Event, Action

// Scarab plugin
use fusabi_tui_scarab::prelude::*;  // ScarabRenderer, plugin API
```

## Troubleshooting

### Issue: "Cannot find Terminal in scope"

**Solution:** Replace `Terminal` usage with direct `Renderer` calls as shown in the examples above.

### Issue: "Backend trait not found"

**Solution:** Use `Renderer` trait instead of `Backend`. The API is simpler and more flexible.

### Issue: "draw method signature mismatch"

**Solution:**
- **Ratatui**: `terminal.draw(|f| { ... })?`
- **fusabi-tui-runtime**: `renderer.draw(&buffer)?; renderer.flush()?;`

### Issue: "Events not working"

**Solution:** Use the renderer's built-in event polling or the engine's event system instead of external event libraries.

## Additional Resources

- [API Documentation](https://docs.rs/fusabi-tui-runtime)
- [Examples Directory](./examples/)
- [Architecture Overview](./docs/architecture/OVERVIEW.md)
- [Ratatui Analysis](./docs/architecture/RATATUI_ANALYSIS.md)

## Support

For migration questions:
- Open an issue at https://github.com/fusabi-lang/fusabi-tui-runtime/issues
- Check existing examples in the repository
- Review the documentation in the `docs/` directory
