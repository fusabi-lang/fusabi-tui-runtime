//! Hot-reload dashboard example
//!
//! This example demonstrates:
//! - Hot reload with file watching
//! - Dashboard engine usage
//! - Multiple widgets (blocks, gauges, lists)
//! - Event handling
//! - State management

use fusabi_tui_core::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
};
use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::prelude::*;
use fusabi_tui_widgets::{
    block::Block,
    borders::{BorderType, Borders},
    gauge::Gauge,
    list::{List, ListItem, ListState},
    paragraph::Paragraph,
    sparkline::Sparkline,
    widget::{StatefulWidget, Widget},
};
use std::io::stdout;
use std::path::PathBuf;
use std::time::{Duration, Instant};

struct Dashboard {
    cpu_usage: u16,
    memory_usage: u16,
    network_data: Vec<u64>,
    log_items: Vec<String>,
    list_state: ListState,
    start_time: Instant,
    frame_count: u64,
}

impl Dashboard {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            cpu_usage: 0,
            memory_usage: 0,
            network_data: vec![0; 50],
            log_items: vec![
                "Dashboard initialized".to_string(),
                "Watching for file changes...".to_string(),
            ],
            list_state,
            start_time: Instant::now(),
            frame_count: 0,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;

        // Simulate CPU usage
        self.cpu_usage = ((self.frame_count as f64 * 0.5).sin() * 50.0 + 50.0) as u16;

        // Simulate memory usage
        self.memory_usage = ((self.frame_count as f64 * 0.3).cos() * 30.0 + 50.0) as u16;

        // Update network data
        self.network_data.remove(0);
        let new_value = ((self.frame_count as f64 * 0.7).sin() * 5.0 + 5.0) as u64;
        self.network_data.push(new_value);
    }

    fn add_log(&mut self, message: String) {
        self.log_items.push(message);
        if self.log_items.len() > 10 {
            self.log_items.remove(0);
        }
    }

    fn render(&mut self, buffer: &mut Buffer, area: Rect) {
        // Main layout: vertical split
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[
                Constraint::Length(3),    // Title
                Constraint::Fill(1),      // Content
                Constraint::Length(3),    // Footer
            ])
            .split(area);

        // Render title
        self.render_title(buffer, main_chunks[0]);

        // Content layout: horizontal split
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(&[
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_chunks[1]);

        // Left panel
        self.render_left_panel(buffer, content_chunks[0]);

        // Right panel
        self.render_right_panel(buffer, content_chunks[1]);

        // Footer
        self.render_footer(buffer, main_chunks[2]);
    }

    fn render_title(&self, buffer: &mut Buffer, area: Rect) {
        let uptime = self.start_time.elapsed().as_secs();
        let title = format!("Dashboard (Uptime: {}s, Frames: {})", uptime, self.frame_count);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        block.render(area, buffer);
    }

    fn render_left_panel(&mut self, buffer: &mut Buffer, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(6),
                Constraint::Fill(1),
            ])
            .split(area);

        // CPU gauge
        let cpu_gauge = Gauge::default()
            .block(
                Block::default()
                    .title("CPU Usage")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .percent(self.cpu_usage)
            .label(format!("{}%", self.cpu_usage))
            .gauge_style(Style::new().fg(Color::Green).bg(Color::Black));

        cpu_gauge.render(chunks[0], buffer);

        // Memory gauge
        let memory_gauge = Gauge::default()
            .block(
                Block::default()
                    .title("Memory Usage")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .percent(self.memory_usage)
            .label(format!("{}%", self.memory_usage))
            .gauge_style(Style::new().fg(Color::Yellow).bg(Color::Black));

        memory_gauge.render(chunks[1], buffer);

        // Network sparkline
        let sparkline = Sparkline::default()
            .block(
                Block::default()
                    .title("Network Activity")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .data(&self.network_data)
            .style(Style::new().fg(Color::Magenta))
            .max(10);

        sparkline.render(chunks[2], buffer);

        // Info text
        let info = Paragraph::new("Hot reload enabled!\n\nEdit this file and save to see changes.");
        let info_block = Block::default()
            .title("Info")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        info.block(info_block).render(chunks[3], buffer);
    }

    fn render_right_panel(&mut self, buffer: &mut Buffer, area: Rect) {
        // Log list
        let items: Vec<ListItem> = self
            .log_items
            .iter()
            .enumerate()
            .map(|(i, log)| {
                let style = if i % 2 == 0 {
                    Style::new().fg(Color::White)
                } else {
                    Style::new().fg(Color::Gray)
                };
                ListItem::new(log.clone()).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Event Log")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(
                Style::new()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        list.render(area, buffer, &mut self.list_state);
    }

    fn render_footer(&self, buffer: &mut Buffer, area: Rect) {
        let text = "Press 'q' to quit | 'r' to reload | Arrow keys to navigate";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::new().fg(Color::DarkGray));

        paragraph.render(area, buffer);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize renderer
    let renderer = CrosstermRenderer::new(stdout())?;

    // Create dashboard engine
    let mut engine = DashboardEngine::new(renderer, PathBuf::from("."));

    // Enable hot reload with 200ms debounce
    engine.enable_hot_reload_with_debounce(200)?;

    // Create dashboard state
    let mut dashboard = Dashboard::new();
    dashboard.add_log("Dashboard engine started".to_string());

    // Main loop
    let tick_rate = Duration::from_millis(100); // 10 FPS
    let mut last_tick = Instant::now();

    loop {
        // Check for file changes
        if let Some(changes) = engine.poll_changes() {
            if !changes.is_empty() {
                dashboard.add_log(format!("Files changed: {:?}", changes.len()));
                match engine.reload() {
                    Ok(_) => dashboard.add_log("Reloaded successfully".to_string()),
                    Err(e) => dashboard.add_log(format!("Reload error: {}", e)),
                }
            }
        }

        // Get terminal size
        let size = engine.renderer_mut().size()?;
        let area = Rect::new(0, 0, size.width, size.height);
        let mut buffer = Buffer::new(area);

        // Render dashboard
        dashboard.render(&mut buffer, area);

        // Draw to terminal
        engine.renderer_mut().draw(&buffer)?;
        engine.renderer_mut().flush()?;

        // Handle events
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if let Some(event) = engine.renderer_mut().poll_event(timeout) {
            match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => {
                        dashboard.add_log("Quitting...".to_string());
                        break;
                    }
                    KeyCode::Char('r') => {
                        dashboard.add_log("Manual reload requested".to_string());
                        let _ = engine.reload();
                    }
                    KeyCode::Up => {
                        dashboard.list_state.select_previous();
                    }
                    KeyCode::Down => {
                        dashboard.list_state.select_next();
                    }
                    _ => {}
                },
                Event::Resize(_, _) => {
                    dashboard.add_log("Terminal resized".to_string());
                }
                _ => {}
            }
        }

        // Tick update
        if last_tick.elapsed() >= tick_rate {
            dashboard.update();
            last_tick = Instant::now();
        }
    }

    // Cleanup
    engine.renderer_mut().cleanup()?;
    println!("Dashboard shut down gracefully");

    Ok(())
}
