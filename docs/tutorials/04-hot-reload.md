# Tutorial 4: Hot Reload

Build dashboards that update live when you edit files.

## What You'll Learn

- Setting up the DashboardEngine
- Enabling hot reload with file watching
- Handling reload events in your main loop
- Error overlay for development
- Best practices for hot-reloadable dashboards

## Prerequisites

- Completed [Tutorial 3: Tables and Data](03-tables-data.md)
- Understanding of file paths in Rust

## Overview

Hot reload allows you to:
- Edit dashboard configuration while the app runs
- See changes immediately without restarting
- Debug errors with an in-app overlay
- Track file dependencies automatically

## Step 1: Project Setup

```bash
cargo new hot-reload-demo
cd hot-reload-demo
```

Edit `Cargo.toml`:

```toml
[package]
name = "hot-reload-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
fusabi-tui-core = "0.2"
fusabi-tui-widgets = "0.2"
fusabi-tui-render = { version = "0.2", features = ["crossterm-backend"] }
fusabi-tui-engine = "0.2"
crossterm = "0.28"
```

## Step 2: Create a Dashboard File

Create `dashboard.fsx`:

```fsharp
// Dashboard configuration
// Edit this file and watch it reload!

#load "widgets.fsx"

let title = "Hot Reload Demo"
let items = ["Item A"; "Item B"; "Item C"]
let color = "Cyan"
```

Create `widgets.fsx`:

```fsharp
// Widget helpers - a dependency of dashboard.fsx
let formatItem name = sprintf "• %s" name
```

## Step 3: Create the Application

Create `src/main.rs`:

```rust
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fusabi_tui_core::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
};
use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::prelude::*;
use fusabi_tui_widgets::{
    block::Block,
    borders::Borders,
    paragraph::Paragraph,
    widget::Widget,
};
use std::io::stdout;
use std::path::{Path, PathBuf};
use std::time::Duration;

struct App {
    engine: DashboardEngine<CrosstermRenderer<std::io::Stdout>>,
    reload_count: u32,
    last_error: Option<String>,
}

impl App {
    fn new(renderer: CrosstermRenderer<std::io::Stdout>) -> EngineResult<Self> {
        let root_path = PathBuf::from(".");
        let mut engine = DashboardEngine::new(renderer, root_path);

        // Enable hot reload with 200ms debounce
        // This prevents rapid reloads when editors do multiple saves
        engine.enable_hot_reload_with_debounce(200)?;

        Ok(Self {
            engine,
            reload_count: 0,
            last_error: None,
        })
    }

    fn load_dashboard(&mut self, path: &Path) -> EngineResult<()> {
        match self.engine.load(path) {
            Ok(()) => {
                self.last_error = None;
                Ok(())
            }
            Err(e) => {
                // Show error in the UI but don't crash
                self.last_error = Some(e.to_string());
                self.engine.show_error(&e);
                Ok(())
            }
        }
    }

    fn check_for_changes(&mut self) {
        // Poll for file changes
        if let Some(changes) = self.engine.poll_changes() {
            if !changes.is_empty() {
                // File(s) changed - trigger reload
                self.reload_count += 1;

                match self.engine.reload() {
                    Ok(()) => {
                        self.last_error = None;
                        self.engine.dismiss_error();
                    }
                    Err(e) => {
                        self.last_error = Some(e.to_string());
                        self.engine.show_error(&e);
                    }
                }
            }
        }
    }
}
```

## Step 4: The Render Loop

Add the main function:

```rust
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let renderer = CrosstermRenderer::new(stdout())?;
    let mut app = App::new(renderer)?;

    // Set up custom render callback
    app.engine.set_render_callback(|buffer, area, state| {
        render_dashboard(buffer, area, state);
    });

    // Load initial dashboard
    app.load_dashboard(Path::new("dashboard.fsx"))?;

    // Main loop
    loop {
        // Check for file changes (hot reload)
        app.check_for_changes();

        // Render if needed
        if app.engine.state().dirty || app.reload_count > 0 {
            app.engine.render()?;
        }

        // Handle keyboard events
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('r') => {
                            // Manual reload
                            if let Err(e) = app.engine.reload() {
                                app.engine.show_error(&e);
                            }
                        }
                        KeyCode::Enter => {
                            // Dismiss error overlay
                            app.engine.dismiss_error();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Cleanup
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn render_dashboard(buffer: &mut Buffer, area: Rect, state: &DashboardState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(&[
            Constraint::Length(3),  // Header
            Constraint::Fill(1),    // Content
            Constraint::Length(3),  // Status bar
        ])
        .split(area);

    // Header
    let header = Paragraph::new(" Hot Reload Demo")
        .style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).style(Style::new().fg(Color::Cyan)));
    header.render(chunks[0], buffer);

    // Content - shows state info
    let content_text = format!(
        "Dashboard loaded!\n\n\
         Watching for file changes...\n\
         Edit dashboard.fsx and save to see updates.\n\n\
         State: dirty={}, reload_count={}",
        state.dirty,
        0 // In real app, track this
    );

    let content = Paragraph::new(content_text)
        .style(Style::new().fg(Color::White))
        .block(
            Block::default()
                .title(" Dashboard Content ")
                .borders(Borders::ALL)
        );
    content.render(chunks[1], buffer);

    // Status bar
    let status = Paragraph::new(" [r] Reload  [Enter] Dismiss Error  [q] Quit")
        .style(Style::new().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).style(Style::new().fg(Color::DarkGray)));
    status.render(chunks[2], buffer);
}
```

## Step 5: Run It

```bash
cargo run
```

Now try:
1. Open `dashboard.fsx` in your editor
2. Change the title or add items
3. Save the file
4. Watch the TUI update automatically!

## Understanding the Code

### DashboardEngine

The engine manages the hot reload lifecycle:

```rust
// Create engine with root path for resolving relative paths
let engine = DashboardEngine::new(renderer, PathBuf::from("."));

// Enable file watching with debounce
engine.enable_hot_reload_with_debounce(200)?;

// Load initial dashboard
engine.load(Path::new("dashboard.fsx"))?;
```

### File Watching

The engine uses `notify` for efficient file system monitoring:

```rust
// Poll for changes (non-blocking)
if let Some(changes) = engine.poll_changes() {
    if !changes.is_empty() {
        engine.reload()?;
    }
}
```

The debounce time (200ms) prevents rapid reloads when editors save multiple times.

### Error Handling

Errors are displayed as overlays instead of crashing:

```rust
match engine.reload() {
    Ok(()) => engine.dismiss_error(),
    Err(e) => engine.show_error(&e),
}
```

Users can dismiss errors with Enter and continue working.

### Render Callback

The render callback integrates with your UI:

```rust
engine.set_render_callback(|buffer, area, state| {
    // Your rendering logic here
    // This is called by engine.render()
});
```

### State Management

The engine tracks dirty state:

```rust
if engine.state().dirty {
    engine.render()?;
}
```

State becomes dirty when:
- Files are loaded or reloaded
- Errors are shown or dismissed
- You manually call `state.mark_dirty()`

## Dependency Tracking

The engine automatically tracks `#load` directives:

```fsharp
// dashboard.fsx
#load "widgets.fsx"    // This dependency is tracked
#load "../shared.fsx"  // Relative paths work too
```

When any dependency changes, the main file reloads.

## Best Practices

### 1. Debounce Appropriately

```rust
// Short debounce for quick iteration
engine.enable_hot_reload_with_debounce(100)?;

// Longer debounce for complex dashboards
engine.enable_hot_reload_with_debounce(500)?;
```

### 2. Handle Errors Gracefully

```rust
fn safe_reload(&mut self) {
    match self.engine.reload() {
        Ok(()) => {
            self.engine.dismiss_error();
            log::info!("Dashboard reloaded");
        }
        Err(e) => {
            self.engine.show_error(&e);
            log::warn!("Reload failed: {}", e);
            // Don't crash - keep the old state
        }
    }
}
```

### 3. Preserve User State

```rust
// Store UI state separately from dashboard config
struct App {
    engine: DashboardEngine<R>,
    user_state: UserState,  // Survives reloads
}

struct UserState {
    selected_tab: usize,
    scroll_position: usize,
    filters: Vec<String>,
}
```

### 4. Show Reload Feedback

```rust
fn render_status_bar(&self, buffer: &mut Buffer, area: Rect) {
    let status = if self.engine.has_error() {
        "⚠ Error - Press Enter to dismiss"
    } else if self.just_reloaded {
        "✓ Reloaded"
    } else {
        "Watching for changes..."
    };

    Paragraph::new(status).render(area, buffer);
}
```

## Advanced: Custom File Loader

For custom file types or preprocessing:

```rust
use fusabi_tui_engine::loader::{FileLoader, LoadedFile};

// The FileLoader handles caching and dependency tracking
let loader = FileLoader::new();

// Load with dependencies
let file = loader.load(Path::new("dashboard.fsx"))?;
println!("Dependencies: {:?}", file.dependencies);
```

## Exercises

### 1. Add Reload Counter

Display how many times the dashboard has reloaded:

```rust
struct App {
    reload_count: u32,
}

// In check_for_changes:
if !changes.is_empty() {
    self.reload_count += 1;
}
```

### 2. Add File Change Notifications

Show which files changed:

```rust
if let Some(changes) = engine.poll_changes() {
    for path in &changes {
        println!("Changed: {}", path.display());
    }
}
```

### 3. Add Auto-Dismiss Errors

Dismiss error overlay after successful reload:

```rust
if engine.reload().is_ok() {
    engine.dismiss_error();
    // Maybe also show "Reload successful" message
}
```

## Troubleshooting

### Changes Not Detected

1. Check debounce timing (try longer delay)
2. Verify file permissions
3. Some editors use atomic saves - this should work, but check

### Terminal Stuck After Crash

Run `reset` or `stty sane` to restore terminal.

### Too Many Reloads

Increase debounce time or filter out non-essential file types.

## Next Steps

- **[Fusabi Integration Guide](../guides/fusabi-integration.md)** - Full scripting support
- **[Error Overlay API](../api-reference.md#error-overlay)** - Customize error display
- **[Widget Gallery](../../crates/fusabi-tui-widgets/README.md)** - All available widgets

## Complete Code

See the complete working example at:
`crates/fusabi-tui-engine/examples/dashboard.rs`
