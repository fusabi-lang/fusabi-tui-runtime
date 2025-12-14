# fusabi-tui-engine

Hot reload engine and dashboard runtime for Fusabi TUI applications.

This crate provides the core engine for building hot-reloadable TUI applications with the Fusabi framework. It handles file watching, dependency tracking, state management, and event handling.

## Features

- **Hot Reload**: Automatically reload dashboards when files change
- **File Watching**: Efficient file system monitoring with debouncing
- **Dependency Tracking**: Track and reload dependent files
- **Event System**: Rich event handling with keyboard, mouse, and custom events
- **State Management**: Flexible state management for widgets
- **Async Integration**: Built on tokio for async compatibility

## Quick Start

```rust
use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::CrosstermRenderer;
use std::path::PathBuf;
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create renderer
    let renderer = CrosstermRenderer::new(stdout())?;
    
    // Create dashboard engine
    let mut engine = DashboardEngine::new(renderer, PathBuf::from("."));
    
    // Enable hot reload with 200ms debounce
    engine.enable_hot_reload_with_debounce(200)?;
    
    // Load dashboard file
    engine.load(Path::new("dashboard.fsx"))?;
    
    // Main loop
    loop {
        // Check for file changes
        if let Some(changes) = engine.poll_changes() {
            if !changes.is_empty() {
                println!("Reloading dashboard...");
                engine.reload()?;
            }
        }
        
        // Render
        engine.render()?;
        
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
}
```

## Components

### DashboardEngine

The main orchestration engine that manages the entire application lifecycle:

```rust
use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::CrosstermRenderer;
use std::io::stdout;
use std::path::PathBuf;

let renderer = CrosstermRenderer::new(stdout())?;
let mut engine = DashboardEngine::new(renderer, PathBuf::from("."));

// Enable hot reload
engine.enable_hot_reload()?;

// Load dashboard
engine.load(Path::new("dashboard.fsx"))?;

// Access state
let state = engine.state();
println!("Dashboard loaded: {}", state.loaded);

// Render
engine.render()?;
```

### FileWatcher

Watches files for changes with configurable debouncing:

```rust
use fusabi_tui_engine::watcher::FileWatcher;
use std::path::Path;
use std::time::Duration;

// Create watcher with 200ms debounce
let mut watcher = FileWatcher::new(200)?;

// Watch a file
watcher.watch(Path::new("config.toml"))?;

// Check for changes
if let Some(changes) = watcher.poll_changes() {
    for path in changes {
        println!("File changed: {:?}", path);
    }
}
```

### FileLoader

Smart file loading with dependency tracking and caching:

```rust
use fusabi_tui_engine::loader::FileLoader;
use std::path::Path;

let mut loader = FileLoader::new();

// Load a file
let file = loader.load(Path::new("script.fsx"))?;
println!("Content: {}", file.content);
println!("Last modified: {:?}", file.modified);

// Track dependencies
loader.add_dependency(Path::new("script.fsx"), Path::new("lib.fsx"));

// Check if reload needed
if loader.needs_reload(Path::new("script.fsx"))? {
    let file = loader.reload(Path::new("script.fsx"))?;
}
```

## Event System

Comprehensive event handling for user input:

```rust
use fusabi_tui_engine::prelude::*;
use std::time::Duration;

loop {
    // Poll for events
    if let Some(event) = renderer.poll_event(Duration::from_millis(100)) {
        match event {
            Event::Key(KeyEvent { code, modifiers, .. }) => {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        // Handle up arrow
                    }
                    KeyCode::Enter => {
                        // Handle enter
                    }
                    _ => {}
                }
            }
            Event::Mouse(MouseEvent { kind, x, y, .. }) => {
                match kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        println!("Clicked at ({}, {})", x, y);
                    }
                    _ => {}
                }
            }
            Event::Resize(width, height) => {
                println!("Terminal resized to {}x{}", width, height);
            }
            _ => {}
        }
    }
}
```

### Event Types

- **KeyEvent**: Keyboard input with code and modifiers
- **MouseEvent**: Mouse clicks, drags, and scrolling
- **ResizeEvent**: Terminal size changes
- **FocusGained/FocusLost**: Window focus events
- **Paste**: Text paste events

### Action System

Actions represent the result of event handling:

```rust
use fusabi_tui_engine::event::Action;

fn handle_input(key: KeyCode) -> Action {
    match key {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('r') => Action::Reload,
        KeyCode::Up => Action::ScrollUp,
        KeyCode::Down => Action::ScrollDown,
        _ => Action::None,
    }
}

let action = handle_input(KeyCode::Char('q'));
if action.is_quit() {
    // Exit application
}
```

## State Management

### DashboardState

Tracks the overall dashboard state:

```rust
use fusabi_tui_engine::state::DashboardState;

let mut state = DashboardState::default();

state.loaded = true;
state.dirty = true;  // Needs redraw
state.error = None;

// Custom data
state.data.insert("counter".to_string(), 42);
```

### Widget States

State types for stateful widgets:

```rust
use fusabi_tui_engine::state::{ListState, TableState};

// List state
let mut list_state = ListState::default();
list_state.select(Some(0));
list_state.select_next();
list_state.select_previous();

// Table state
let mut table_state = TableState::default();
table_state.select(Some(0));
```

## Hot Reload

The hot reload system automatically detects file changes and reloads your dashboard:

```rust
use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::CrosstermRenderer;
use std::io::stdout;
use std::path::PathBuf;

let renderer = CrosstermRenderer::new(stdout())?;
let mut engine = DashboardEngine::new(renderer, PathBuf::from("."));

// Enable hot reload with custom debounce
engine.enable_hot_reload_with_debounce(200)?;

// Load dashboard
engine.load(Path::new("dashboard.fsx"))?;

// Main loop
loop {
    // Auto-reload on file changes
    if let Some(changes) = engine.poll_changes() {
        if !changes.is_empty() {
            println!("Files changed: {:?}", changes);
            match engine.reload() {
                Ok(_) => println!("Dashboard reloaded successfully"),
                Err(e) => eprintln!("Reload failed: {}", e),
            }
        }
    }
    
    // Render if dirty
    if engine.state().dirty {
        engine.render()?;
    }
    
    std::thread::sleep(Duration::from_millis(16));
}
```

### Development Overlay

The engine provides a development overlay for debugging:

```rust
// Enable development mode
engine.set_dev_mode(true);

// The overlay shows:
// - File path being watched
// - Last reload time
// - Reload count
// - Error messages (if any)
```

## Error Handling

The engine provides detailed error types:

```rust
use fusabi_tui_engine::prelude::*;

match engine.load(Path::new("dashboard.fsx")) {
    Ok(_) => println!("Loaded successfully"),
    Err(EngineError::LoadError(e)) => {
        eprintln!("Failed to load file: {}", e);
    }
    Err(EngineError::WatchError(e)) => {
        eprintln!("File watching error: {}", e);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Performance

- **Debouncing**: Prevents excessive reloads from rapid file changes
- **Incremental loading**: Only reloads changed files and their dependencies
- **Efficient watching**: Uses native file system events (inotify, FSEvents, etc.)
- **State preservation**: Preserves widget state across reloads when possible

## Testing

The engine is designed to be testable:

```rust
use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::test::TestRenderer;
use std::path::PathBuf;

#[test]
fn test_engine() {
    let renderer = TestRenderer::new(80, 24);
    let mut engine = DashboardEngine::new(renderer, PathBuf::from("."));
    
    // Test loading
    assert!(engine.load(Path::new("test_dashboard.fsx")).is_ok());
    
    // Test state
    assert!(engine.state().loaded);
    
    // Test rendering
    assert!(engine.render().is_ok());
}
```

## Integration with Other Crates

This crate is designed to work seamlessly with:

- **fusabi-tui-core**: Provides Buffer, Rect, and Style types
- **fusabi-tui-render**: Manages renderer lifecycle
- **fusabi-tui-widgets**: Provides widget state types
- **fusabi-tui-scarab**: Enables Scarab plugin mode

## Examples

See the workspace examples directory for:

- **basic_app.rs**: Simple standalone TUI application
- **dashboard.rs**: Hot-reload dashboard with file watching
- **stateful.rs**: Stateful widgets with event handling

## Future Enhancements

Planned features:

- **Fusabi VM integration**: Execute .fzb bytecode for plugins
- **Fusabi Frontend integration**: Compile .fsx scripts on-the-fly
- **Plugin system**: Load external Fusabi plugins
- **State serialization**: Persist state across restarts
- **Multi-file projects**: Support for complex dashboard projects

## License

Licensed under either of:

- MIT license or http://opensource.org/licenses/MIT
- Apache License, Version 2.0 or http://www.apache.org/licenses/LICENSE-2.0

at your option.
