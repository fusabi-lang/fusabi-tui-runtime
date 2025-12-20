# Tutorial 1: Hello World

Build your first TUI application with fusabi-tui-runtime.

## What You'll Learn

- Setting up a new project
- Creating a basic renderer
- Rendering text to the terminal
- Handling keyboard input
- Proper terminal cleanup

## Prerequisites

- Rust 1.75 or later
- Basic familiarity with Rust

## Step 1: Create a New Project

```bash
cargo new hello-tui
cd hello-tui
```

## Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "hello-tui"
version = "0.1.0"
edition = "2021"

[dependencies]
fusabi-tui-core = "0.2"
fusabi-tui-widgets = "0.2"
fusabi-tui-render = { version = "0.2", features = ["crossterm-backend"] }
crossterm = "0.28"
```

## Step 3: Basic Application Structure

Replace `src/main.rs` with:

```rust
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fusabi_tui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};
use fusabi_tui_render::prelude::*;
use fusabi_tui_widgets::{
    block::Block,
    borders::Borders,
    paragraph::Paragraph,
    widget::Widget,
};
use std::io::stdout;
use std::time::Duration;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Step 1: Enter raw mode and alternate screen
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    // Step 2: Create renderer
    let mut renderer = CrosstermRenderer::new(stdout())?;
    renderer.show_cursor(false)?;

    // Step 3: Main loop
    loop {
        // Get terminal size
        let size = renderer.size()?;
        let mut buffer = Buffer::new(size);

        // Render UI
        render_ui(&mut buffer, size);

        // Draw to screen
        renderer.draw(&buffer)?;
        renderer.flush()?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // Step 4: Cleanup
    renderer.show_cursor(true)?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    println!("Goodbye!");
    Ok(())
}

fn render_ui(buffer: &mut Buffer, area: Rect) {
    // Create a block with a title
    let block = Block::default()
        .title(" Hello Fusabi TUI! ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));

    // Create paragraph with instructions
    let text = "Welcome to your first TUI application!\n\n\
                Press 'q' or ESC to quit.";

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::new().fg(Color::White));

    // Render the widget
    paragraph.render(area, buffer);
}
```

## Step 4: Run It

```bash
cargo run
```

You should see a bordered box with "Hello Fusabi TUI!" as the title. Press 'q' or ESC to exit.

## Understanding the Code

### Terminal Modes

```rust
enable_raw_mode()?;  // Disable line buffering, echo
stdout().execute(EnterAlternateScreen)?;  // Use alternate buffer
```

Raw mode lets us receive individual keypresses. Alternate screen preserves your terminal history.

### The Render Loop

```rust
loop {
    let size = renderer.size()?;      // Get current terminal size
    let mut buffer = Buffer::new(size); // Create empty buffer
    render_ui(&mut buffer, size);       // Fill buffer with widgets
    renderer.draw(&buffer)?;            // Send to terminal
    renderer.flush()?;                  // Ensure it's displayed

    // Handle events...
}
```

### Widgets

Widgets are composable UI elements:

```rust
let block = Block::default()
    .title(" Title ")        // Add a title
    .borders(Borders::ALL)   // Add borders on all sides
    .style(Style::new().fg(Color::Cyan));  // Style the border

let paragraph = Paragraph::new("Text content")
    .block(block);           // Put paragraph inside the block

paragraph.render(area, buffer);  // Render to buffer
```

### Cleanup

Always restore terminal state:

```rust
renderer.show_cursor(true)?;
stdout().execute(LeaveAlternateScreen)?;
disable_raw_mode()?;
```

## Common Issues

### Terminal stuck in raw mode

If your app crashes, your terminal may be stuck. Run `reset` or `stty sane` to fix it.

### Nothing appears

- Check that the area has non-zero width and height
- Ensure you're calling `renderer.flush()?`

### Colors look wrong

Some terminals have limited color support. Try `Color::Red` or other basic colors.

## Next Steps

- **[Tutorial 2: Interactive UI](02-interactive-ui.md)** - Add state and handle more inputs
- **[Widget Gallery](../../crates/fusabi-tui-widgets/README.md)** - Explore available widgets
- **[Layout System](../guides/layout-system.md)** - Learn constraint-based layouts

## Complete Code

The complete code for this tutorial is available at:
`crates/fusabi-tui-engine/examples/basic_app.rs`
