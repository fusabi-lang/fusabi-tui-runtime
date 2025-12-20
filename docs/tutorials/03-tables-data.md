# Tutorial 3: Tables and Data

Display structured data in tables with sorting, selection, and dynamic updates.

## What You'll Learn

- Creating tables with headers and rows
- Column width constraints
- Row selection and highlighting
- Dynamic data updates
- Styling individual cells and rows

## Prerequisites

- Completed [Tutorial 2: Interactive UI](02-interactive-ui.md)
- Understanding of Rust structs

## Step 1: Project Setup

```bash
cargo new table-demo
cd table-demo
```

Edit `Cargo.toml`:

```toml
[package]
name = "table-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
fusabi-tui-core = "0.2"
fusabi-tui-widgets = "0.2"
fusabi-tui-render = { version = "0.2", features = ["crossterm-backend"] }
crossterm = "0.28"
```

## Step 2: Define Data Structures

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
    style::{Color, Style, Modifier},
};
use fusabi_tui_render::prelude::*;
use fusabi_tui_widgets::{
    block::Block,
    borders::Borders,
    paragraph::Paragraph,
    table::{Row, Table, TableCell, TableState},
    widget::{Widget, StatefulWidget},
};
use std::io::stdout;
use std::time::Duration;

/// A process entry for our task manager demo
#[derive(Clone)]
struct Process {
    pid: u32,
    name: String,
    cpu: f32,
    memory: u64,
    status: ProcessStatus,
}

#[derive(Clone, Copy, PartialEq)]
enum ProcessStatus {
    Running,
    Sleeping,
    Stopped,
}

impl ProcessStatus {
    fn as_str(&self) -> &'static str {
        match self {
            ProcessStatus::Running => "Running",
            ProcessStatus::Sleeping => "Sleeping",
            ProcessStatus::Stopped => "Stopped",
        }
    }

    fn color(&self) -> Color {
        match self {
            ProcessStatus::Running => Color::Green,
            ProcessStatus::Sleeping => Color::Yellow,
            ProcessStatus::Stopped => Color::Red,
        }
    }
}

/// Application state
struct App {
    processes: Vec<Process>,
    table_state: TableState,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        // Sample data
        let processes = vec![
            Process { pid: 1, name: "systemd".to_string(), cpu: 0.1, memory: 12_400, status: ProcessStatus::Running },
            Process { pid: 42, name: "bash".to_string(), cpu: 0.0, memory: 5_200, status: ProcessStatus::Sleeping },
            Process { pid: 100, name: "nginx".to_string(), cpu: 2.5, memory: 45_000, status: ProcessStatus::Running },
            Process { pid: 101, name: "postgres".to_string(), cpu: 5.2, memory: 128_000, status: ProcessStatus::Running },
            Process { pid: 200, name: "redis".to_string(), cpu: 0.8, memory: 22_000, status: ProcessStatus::Running },
            Process { pid: 305, name: "node".to_string(), cpu: 12.3, memory: 256_000, status: ProcessStatus::Running },
            Process { pid: 410, name: "python".to_string(), cpu: 3.1, memory: 89_000, status: ProcessStatus::Sleeping },
            Process { pid: 500, name: "vim".to_string(), cpu: 0.0, memory: 8_500, status: ProcessStatus::Stopped },
        ];

        let mut table_state = TableState::new();
        table_state.select(Some(0));

        Self {
            processes,
            table_state,
            should_quit: false,
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') => self.next_row(),
            KeyCode::Up | KeyCode::Char('k') => self.previous_row(),
            KeyCode::Home => self.table_state.select(Some(0)),
            KeyCode::End => self.table_state.select(Some(self.processes.len().saturating_sub(1))),
            _ => {}
        }
    }

    fn next_row(&mut self) {
        let len = self.processes.len();
        if len == 0 { return; }

        let next = match self.table_state.selected() {
            Some(i) => (i + 1) % len,
            None => 0,
        };
        self.table_state.select(Some(next));
    }

    fn previous_row(&mut self) {
        let len = self.processes.len();
        if len == 0 { return; }

        let prev = match self.table_state.selected() {
            Some(0) => len - 1,
            Some(i) => i - 1,
            None => 0,
        };
        self.table_state.select(Some(prev));
    }

    fn selected_process(&self) -> Option<&Process> {
        self.table_state.selected().and_then(|i| self.processes.get(i))
    }
}
```

## Step 3: Build the Table

Add the rendering functions:

```rust
fn render_ui(app: &mut App, buffer: &mut Buffer, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(&[
            Constraint::Length(3),   // Title
            Constraint::Fill(1),     // Table
            Constraint::Length(5),   // Details
        ])
        .split(area);

    render_title(buffer, chunks[0]);
    render_table(app, buffer, chunks[1]);
    render_details(app, buffer, chunks[2]);
}

fn render_title(buffer: &mut Buffer, area: Rect) {
    let title = Paragraph::new(" Process Monitor")
        .style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).style(Style::new().fg(Color::Cyan)));

    title.render(area, buffer);
}

fn render_table(app: &mut App, buffer: &mut Buffer, area: Rect) {
    // Create header row
    let header = Row::new(vec![
        TableCell::new("PID"),
        TableCell::new("Name"),
        TableCell::new("CPU %"),
        TableCell::new("Memory"),
        TableCell::new("Status"),
    ]).style(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    // Create data rows
    let rows: Vec<Row> = app.processes.iter().map(|p| {
        Row::new(vec![
            TableCell::new(p.pid.to_string()),
            TableCell::new(&p.name),
            TableCell::new(format!("{:.1}", p.cpu)),
            TableCell::new(format_memory(p.memory)),
            TableCell::new(p.status.as_str()).style(Style::new().fg(p.status.color())),
        ])
    }).collect();

    // Create table widget
    let table = Table::new(rows)
        .header(header)
        .widths(&[
            Constraint::Length(8),   // PID
            Constraint::Length(15),  // Name
            Constraint::Length(8),   // CPU
            Constraint::Length(12),  // Memory
            Constraint::Length(10),  // Status
        ])
        .column_spacing(2)
        .style(Style::new().fg(Color::White))
        .highlight_style(
            Style::new()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        );

    // Render with block
    let block = Block::default()
        .title(" Processes ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::White));

    let inner = block.inner(area);
    block.render(area, buffer);

    StatefulWidget::render(&table, inner, buffer, &mut app.table_state);
}

fn render_details(app: &App, buffer: &mut Buffer, area: Rect) {
    let detail_text = if let Some(process) = app.selected_process() {
        format!(
            "Selected: {} (PID {})\n\
             CPU: {:.1}% | Memory: {} | Status: {}",
            process.name,
            process.pid,
            process.cpu,
            format_memory(process.memory),
            process.status.as_str()
        )
    } else {
        "No process selected".to_string()
    };

    let details = Paragraph::new(detail_text)
        .style(Style::new().fg(Color::White))
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::White))
        );

    details.render(area, buffer);
}

fn format_memory(bytes: u64) -> String {
    if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}
```

## Step 4: Main Loop

```rust
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut renderer = CrosstermRenderer::new(stdout())?;
    renderer.show_cursor(false)?;

    let mut app = App::new();

    while !app.should_quit {
        let size = renderer.size()?;
        let mut buffer = Buffer::new(size);

        render_ui(&mut app, &mut buffer, size);

        renderer.draw(&buffer)?;
        renderer.flush()?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                }
            }
        }
    }

    renderer.show_cursor(true)?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
```

## Step 5: Run It

```bash
cargo run
```

You'll see a process table with:
- Column headers in yellow
- Colored status indicators
- Selection highlighting
- Details panel for selected process

## Understanding the Code

### TableCell Styling

Individual cells can have their own styles:

```rust
TableCell::new(p.status.as_str())
    .style(Style::new().fg(p.status.color()))
```

This allows conditional formatting based on data values.

### Column Widths

Control column sizing with constraints:

```rust
.widths(&[
    Constraint::Length(8),    // Fixed width
    Constraint::Percentage(30), // Percentage of table width
    Constraint::Fill(1),      // Fill remaining space
])
```

### Header Row

The header is styled separately:

```rust
let header = Row::new(vec!["Col1", "Col2"])
    .style(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD));

Table::new(rows).header(header)
```

### Row Height and Spacing

For multi-line cells or spacing:

```rust
Row::new(cells)
    .height(2)        // Row spans 2 lines
    .bottom_margin(1) // Extra space after row
```

## Exercises

### 1. Add Sorting

Sort processes by different columns:

```rust
enum SortColumn { Pid, Name, Cpu, Memory }
enum SortOrder { Ascending, Descending }

impl App {
    fn sort_by(&mut self, column: SortColumn) {
        match column {
            SortColumn::Cpu => {
                self.processes.sort_by(|a, b|
                    b.cpu.partial_cmp(&a.cpu).unwrap()
                );
            }
            // ... other columns
        }
    }
}
```

Add keybindings like `1`-`4` to sort by column.

### 2. Add Filtering

Filter processes by status or name:

```rust
impl App {
    fn filter_by_status(&mut self, status: ProcessStatus) {
        self.visible_processes = self.processes
            .iter()
            .filter(|p| p.status == status)
            .cloned()
            .collect();
    }
}
```

### 3. Add CPU Bars

Show CPU as a visual bar:

```rust
fn cpu_bar(cpu: f32, width: u16) -> String {
    let filled = ((cpu / 100.0) * width as f32) as usize;
    let empty = width as usize - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
```

### 4. Live Updates

Simulate live data updates:

```rust
use std::time::Instant;

struct App {
    last_update: Instant,
    // ...
}

impl App {
    fn update(&mut self) {
        if self.last_update.elapsed() > Duration::from_secs(1) {
            // Update CPU values with random variance
            for process in &mut self.processes {
                process.cpu = (process.cpu + rand::random::<f32>() - 0.5)
                    .clamp(0.0, 100.0);
            }
            self.last_update = Instant::now();
        }
    }
}
```

## Advanced Patterns

### Conditional Row Styling

Style entire rows based on conditions:

```rust
let rows: Vec<Row> = app.processes.iter().map(|p| {
    let base_style = if p.cpu > 50.0 {
        Style::new().fg(Color::Red)  // High CPU warning
    } else {
        Style::default()
    };

    Row::new(vec![...]).style(base_style)
}).collect();
```

### Scrollable Tables

TableState automatically tracks scroll offset:

```rust
// Get scroll info for a scrollbar
let offset = app.table_state.offset();
let visible = area.height as usize;
let total = app.processes.len();
```

### Column Alignment

Right-align numeric columns:

```rust
fn right_pad(s: &str, width: usize) -> String {
    format!("{:>width$}", s, width = width)
}

TableCell::new(right_pad(&format!("{:.1}", cpu), 6))
```

## Next Steps

- **[Tutorial 4: Hot Reload](04-hot-reload.md)** - Live dashboard updates
- **[Widget Gallery](../../crates/fusabi-tui-widgets/README.md)** - Gauges, sparklines, charts
- **[API Reference](../api-reference.md)** - Complete API documentation

## Complete Code

See the complete working example at:
`crates/fusabi-tui-engine/examples/dashboard.rs`
