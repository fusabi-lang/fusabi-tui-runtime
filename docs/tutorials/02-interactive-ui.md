# Tutorial 2: Interactive UI

Build a navigable list application with keyboard controls and state management.

## What You'll Learn

- Managing application state
- Using layouts to divide the screen
- Creating interactive lists with selection
- Handling keyboard navigation
- Using StatefulWidget for stateful components

## Prerequisites

- Completed [Tutorial 1: Hello World](01-hello-world.md)
- Basic understanding of Rust structs and enums

## Step 1: Project Setup

Create a new project or continue from Tutorial 1:

```bash
cargo new interactive-tui
cd interactive-tui
```

Edit `Cargo.toml`:

```toml
[package]
name = "interactive-tui"
version = "0.1.0"
edition = "2021"

[dependencies]
fusabi-tui-core = "0.2"
fusabi-tui-widgets = "0.2"
fusabi-tui-render = { version = "0.2", features = ["crossterm-backend"] }
crossterm = "0.28"
```

## Step 2: Define Application State

Create `src/main.rs` with the application state structure:

```rust
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fusabi_tui_core::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
};
use fusabi_tui_render::prelude::*;
use fusabi_tui_widgets::{
    block::Block,
    borders::Borders,
    list::{List, ListItem, ListState},
    paragraph::Paragraph,
    widget::{Widget, StatefulWidget},
};
use std::io::stdout;
use std::time::Duration;

/// Application state
struct App {
    /// List of items to display
    items: Vec<String>,
    /// State for the list widget (tracks selection)
    list_state: ListState,
    /// Whether the app should quit
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let items = vec![
            "Item 1: First entry".to_string(),
            "Item 2: Second entry".to_string(),
            "Item 3: Third entry".to_string(),
            "Item 4: Fourth entry".to_string(),
            "Item 5: Fifth entry".to_string(),
            "Item 6: Sixth entry".to_string(),
            "Item 7: Seventh entry".to_string(),
            "Item 8: Eighth entry".to_string(),
        ];

        let mut list_state = ListState::new();
        list_state.select(Some(0)); // Select first item by default

        Self {
            items,
            list_state,
            should_quit: false,
        }
    }

    /// Handle keyboard input
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.list_state.select_next(self.items.len());
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.list_state.select_previous(self.items.len());
            }
            KeyCode::Home => {
                self.list_state.select_first();
            }
            KeyCode::End => {
                self.list_state.select_last(self.items.len());
            }
            KeyCode::Enter => {
                // Could trigger an action on the selected item
                if let Some(selected) = self.list_state.selected() {
                    // In a real app, you'd do something with the selection
                    let _ = selected;
                }
            }
            _ => {}
        }
    }
}
```

## Step 3: Create the Layout

Add the rendering function that divides the screen into sections:

```rust
fn render_ui(app: &mut App, buffer: &mut Buffer, area: Rect) {
    // Create a vertical layout with header, content, and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(&[
            Constraint::Length(3),  // Header
            Constraint::Fill(1),    // Main content (fills remaining)
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Render each section
    render_header(buffer, chunks[0]);
    render_list(app, buffer, chunks[1]);
    render_footer(buffer, chunks[2]);
}

fn render_header(buffer: &mut Buffer, area: Rect) {
    let header = Paragraph::new(" Interactive List Demo")
        .style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::Cyan))
        );

    header.render(area, buffer);
}

fn render_list(app: &mut App, buffer: &mut Buffer, area: Rect) {
    // Convert strings to ListItems
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|s| ListItem::new(s.as_str()))
        .collect();

    // Create the list widget
    let list = List::new(items)
        .block(
            Block::default()
                .title(" Select an Item ")
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::White))
        )
        .highlight_style(
            Style::new()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    // Use StatefulWidget::render for stateful widgets
    StatefulWidget::render(&list, area, buffer, &mut app.list_state);
}

fn render_footer(buffer: &mut Buffer, area: Rect) {
    let footer = Paragraph::new(" [j/k or arrows] Navigate  [Enter] Select  [q] Quit")
        .style(Style::new().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::DarkGray))
        );

    footer.render(area, buffer);
}
```

## Step 4: Main Loop

Add the main function with the event loop:

```rust
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut renderer = CrosstermRenderer::new(stdout())?;
    renderer.show_cursor(false)?;

    // Create application state
    let mut app = App::new();

    // Main loop
    while !app.should_quit {
        // Get terminal size and create buffer
        let size = renderer.size()?;
        let mut buffer = Buffer::new(size);

        // Render UI
        render_ui(&mut app, &mut buffer, size);

        // Draw to screen
        renderer.draw(&buffer)?;
        renderer.flush()?;

        // Handle events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                }
            }
        }
    }

    // Cleanup
    renderer.show_cursor(true)?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    println!("Goodbye!");
    Ok(())
}
```

## Step 5: Run It

```bash
cargo run
```

You should see:
- A header with the title
- A list of items with one highlighted
- A footer with keyboard shortcuts

Try navigating with `j`/`k` or arrow keys. Press `q` to quit.

## Understanding the Code

### Application State

```rust
struct App {
    items: Vec<String>,
    list_state: ListState,  // Tracks selection
    should_quit: bool,
}
```

Keeping state in a struct makes it easy to:
- Pass to render functions
- Modify in response to events
- Extend with new fields

### ListState

```rust
let mut list_state = ListState::new();
list_state.select(Some(0));  // Select first item
```

`ListState` tracks:
- Currently selected index (`selected()`)
- Scroll offset for long lists (`offset()`)

Navigation methods:
- `select_next(len)` - Move to next item (wraps around)
- `select_previous(len)` - Move to previous item (wraps around)
- `select_first()` - Jump to first item
- `select_last(len)` - Jump to last item

### StatefulWidget

```rust
// Regular Widget (stateless)
paragraph.render(area, buffer);

// StatefulWidget (needs state)
StatefulWidget::render(&list, area, buffer, &mut app.list_state);
```

Use `StatefulWidget::render` when a widget needs to track state between renders.

### Layout System

```rust
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(&[
        Constraint::Length(3),  // Fixed 3 rows
        Constraint::Fill(1),    // Fill remaining space
        Constraint::Length(3),  // Fixed 3 rows
    ])
    .split(area);
```

Constraint types:
- `Length(n)` - Exactly n cells
- `Percentage(p)` - p% of available space
- `Min(n)` - At least n cells
- `Max(n)` - At most n cells
- `Fill(weight)` - Fill remaining space (proportional by weight)
- `Ratio(num, denom)` - Fraction of available space

## Exercises

### 1. Add Item Details

Show details of the selected item in a side panel:

```rust
let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(&[
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(content_area);

// Left: List
render_list(app, buffer, chunks[0]);

// Right: Details
if let Some(idx) = app.list_state.selected() {
    let detail = Paragraph::new(format!("Selected: {}", app.items[idx]));
    detail.render(chunks[1], buffer);
}
```

### 2. Add More Items

Make a list long enough to scroll:

```rust
let items: Vec<String> = (1..=50)
    .map(|i| format!("Item {}: Description", i))
    .collect();
```

The list will automatically scroll to keep the selection visible.

### 3. Add Delete Functionality

Allow deleting items with the Delete key:

```rust
KeyCode::Delete | KeyCode::Char('d') => {
    if let Some(selected) = self.list_state.selected() {
        if !self.items.is_empty() {
            self.items.remove(selected);
            // Adjust selection if needed
            if selected >= self.items.len() && selected > 0 {
                self.list_state.select(Some(selected - 1));
            }
        }
    }
}
```

## Common Patterns

### Centering Content

```rust
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(&[
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(&[
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
```

### Tab Navigation

For multi-tab interfaces, track the active tab:

```rust
struct App {
    active_tab: usize,
    // ...
}

KeyCode::Tab => {
    self.active_tab = (self.active_tab + 1) % TAB_COUNT;
}
KeyCode::BackTab => {
    self.active_tab = self.active_tab.checked_sub(1).unwrap_or(TAB_COUNT - 1);
}
```

## Next Steps

- **[Tutorial 3: Tables and Data](03-tables-data.md)** - Display tabular data
- **[Layout Guide](../guides/layout-system.md)** - Advanced layout techniques
- **[Widget Gallery](../../crates/fusabi-tui-widgets/README.md)** - All available widgets

## Complete Code

The complete code for this tutorial is in a single file above. For a more structured approach, see:
`crates/fusabi-tui-engine/examples/dashboard.rs`
