# fusabi-tui-render

Renderer abstraction for the Fusabi TUI framework.

This crate provides a unified renderer interface supporting multiple backends, enabling the same TUI code to run standalone in a terminal or as a Scarab plugin via shared memory.

## Features

- **Renderer Trait**: Backend-agnostic rendering interface
- **Crossterm Backend**: Standalone terminal rendering via crossterm
- **Test Backend**: In-memory rendering for unit tests
- **Event Integration**: Built-in event polling support
- **Async-Ready**: Compatible with tokio and other async runtimes

## Backends

### CrosstermRenderer

Renders to standard terminals using the crossterm library. Supports all major platforms (Linux, macOS, Windows).

```rust
use fusabi_tui_render::prelude::*;
use fusabi_tui_core::{buffer::Buffer, layout::Rect};
use std::io::stdout;

fn main() -> Result<()> {
    let mut renderer = CrosstermRenderer::new(stdout())?;
    
    let size = renderer.size()?;
    let mut buffer = Buffer::new(Rect::new(0, 0, size.width, size.height));
    
    // Draw to buffer...
    
    renderer.draw(&buffer)?;
    renderer.flush()?;
    renderer.cleanup()?;
    Ok(())
}
```

### TestRenderer

In-memory renderer for unit testing TUI applications:

```rust
use fusabi_tui_render::test::TestRenderer;
use fusabi_tui_render::renderer::Renderer;
use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::Style};

let mut renderer = TestRenderer::new(10, 5);
let mut buffer = Buffer::new(Rect::new(0, 0, 10, 5));

buffer.set_string(0, 0, "Test", Style::default());
renderer.draw(&buffer).unwrap();

// Verify the output
assert_eq!(renderer.buffer().get(0, 0).unwrap().symbol(), "T");
```

### ScarabRenderer (in fusabi-tui-scarab)

Renders to Scarab's shared memory for plugin mode. See the fusabi-tui-scarab crate.

## Quick Start

### Basic Rendering

```rust
use fusabi_tui_render::prelude::*;
use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::{Color, Style}};
use std::io::stdout;

fn main() -> Result<()> {
    // Initialize renderer
    let mut renderer = CrosstermRenderer::new(stdout())?;
    
    // Get terminal size
    let size = renderer.size()?;
    let mut buffer = Buffer::new(Rect::new(0, 0, size.width, size.height));
    
    // Draw content
    let style = Style::new().fg(Color::Green);
    buffer.set_string(0, 0, "Hello, Fusabi TUI!", style);
    
    // Render and flush
    renderer.draw(&buffer)?;
    renderer.flush()?;
    
    // Cleanup on exit
    renderer.cleanup()?;
    Ok(())
}
```

### Event Handling

```rust
use fusabi_tui_render::prelude::*;
use std::time::Duration;
use std::io::stdout;

fn main() -> Result<()> {
    let mut renderer = CrosstermRenderer::new(stdout())?;
    
    loop {
        // Poll for events with timeout
        if let Some(event) = renderer.poll_event(Duration::from_millis(100)) {
            match event {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                Event::Resize(width, height) => {
                    // Handle terminal resize
                }
                _ => {}
            }
        }
        
        // Render frame...
    }
    
    renderer.cleanup()?;
    Ok(())
}
```

## Renderer Trait

The core `Renderer` trait provides a simple, flexible interface:

```rust
pub trait Renderer {
    type Error: std::error::Error;
    
    /// Draw a buffer to the renderer
    fn draw(&mut self, buffer: &Buffer) -> Result<(), Self::Error>;
    
    /// Flush pending changes to the display
    fn flush(&mut self) -> Result<(), Self::Error>;
    
    /// Get the current size of the rendering area
    fn size(&self) -> Result<Size, Self::Error>;
    
    /// Poll for input events (optional, returns None if not supported)
    fn poll_event(&mut self, timeout: Duration) -> Option<Event> {
        None
    }
    
    /// Cleanup resources (called on shutdown)
    fn cleanup(&mut self) -> Result<(), Self::Error>;
}
```

## Features

### `crossterm-backend` (default)

Enables the CrosstermRenderer for standalone terminal rendering.

```toml
[dependencies]
fusabi-tui-render = "0.1"
```

### Without Default Features

For minimal dependencies or when using only the test renderer:

```toml
[dependencies]
fusabi-tui-render = { version = "0.1", default-features = false }
```

## Error Handling

All renderers use the `RenderError` type for error handling:

```rust
use fusabi_tui_render::prelude::*;

match renderer.draw(&buffer) {
    Ok(_) => {}
    Err(RenderError::IoError(e)) => {
        eprintln!("IO error: {}", e);
    }
    Err(RenderError::CrosstermError(e)) => {
        eprintln!("Crossterm error: {}", e);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Testing

The test renderer makes it easy to test TUI applications:

```rust
use fusabi_tui_render::test::TestRenderer;
use fusabi_tui_render::renderer::Renderer;
use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::{Color, Style}};

#[test]
fn test_rendering() {
    let mut renderer = TestRenderer::new(20, 10);
    let mut buffer = Buffer::new(Rect::new(0, 0, 20, 10));
    
    // Render a colored box
    let style = Style::new().bg(Color::Blue);
    buffer.set_style(Rect::new(0, 0, 20, 10), style);
    
    renderer.draw(&buffer).unwrap();
    
    // Verify background color
    let cell = renderer.buffer().get(5, 5).unwrap();
    assert_eq!(cell.bg(), Color::Blue);
}
```

## Performance

- **Zero-copy**: Buffers are passed by reference
- **Minimal allocations**: Reuse buffers between frames
- **Efficient diffing**: CrosstermRenderer only sends changed cells
- **Non-blocking**: Event polling is non-blocking with configurable timeout

## Integration with Other Crates

This crate is designed to work seamlessly with:

- **fusabi-tui-core**: Provides Buffer and Style types
- **fusabi-tui-widgets**: Widgets render into buffers
- **fusabi-tui-engine**: Engine manages renderer lifecycle
- **fusabi-tui-scarab**: Scarab shared memory backend

## Examples

See the workspace examples directory for complete applications.

## License

Licensed under either of:

- MIT license or http://opensource.org/licenses/MIT
- Apache License, Version 2.0 or http://www.apache.org/licenses/LICENSE-2.0

at your option.
