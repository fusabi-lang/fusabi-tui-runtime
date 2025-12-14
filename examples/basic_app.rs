//! Basic standalone TUI application example
//!
//! This example demonstrates:
//! - Creating a simple TUI application
//! - Using the crossterm renderer
//! - Rendering widgets (Block, Paragraph)
//! - Handling keyboard events
//! - Clean shutdown

use fusabi_tui_core::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
};
use fusabi_tui_render::prelude::*;
use fusabi_tui_widgets::{
    block::Block,
    borders::{BorderType, Borders},
    paragraph::{Alignment, Paragraph},
    text::{Line, Span, Text},
    widget::Widget,
};
use std::io::{self, stdout};
use std::time::{Duration, Instant};

struct App {
    counter: u32,
    start_time: Instant,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            counter: 0,
            start_time: Instant::now(),
            should_quit: false,
        }
    }

    fn on_tick(&mut self) {
        self.counter = self.counter.wrapping_add(1);
    }

    fn on_key(&mut self, code: char) {
        match code {
            'q' => self.should_quit = true,
            _ => {}
        }
    }

    fn render(&self, buffer: &mut Buffer, area: Rect) {
        // Split the terminal into sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[
                Constraint::Length(3),    // Header
                Constraint::Fill(1),      // Body
                Constraint::Length(3),    // Footer
            ])
            .split(area);

        // Render header
        self.render_header(buffer, chunks[0]);

        // Render body
        self.render_body(buffer, chunks[1]);

        // Render footer
        self.render_footer(buffer, chunks[2]);
    }

    fn render_header(&self, buffer: &mut Buffer, area: Rect) {
        let title = "Fusabi TUI - Basic Example";
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::new().fg(Color::Cyan));

        block.render(area, buffer);
    }

    fn render_body(&self, buffer: &mut Buffer, area: Rect) {
        let elapsed = self.start_time.elapsed();
        let seconds = elapsed.as_secs();

        let text = Text::from(vec![
            Line::from(vec![
                Span::styled("Welcome to ", Style::default()),
                Span::styled(
                    "fusabi-tui-runtime",
                    Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::styled("!", Style::default()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Counter: "),
                Span::styled(
                    format!("{}", self.counter),
                    Style::new().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![
                Span::raw("Uptime: "),
                Span::styled(
                    format!("{}s", seconds),
                    Style::new().fg(Color::Magenta),
                ),
            ]),
            Line::from(""),
            Line::from("This is a simple standalone TUI application."),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("'q'", Style::new().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to quit"),
            ]),
        ]);

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title("Status")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center);

        paragraph.render(area, buffer);
    }

    fn render_footer(&self, buffer: &mut Buffer, area: Rect) {
        let text = "fusabi-tui-runtime v0.1.0 | Standalone Mode";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::new().fg(Color::DarkGray));

        paragraph.render(area, buffer);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal
    let mut renderer = CrosstermRenderer::new(stdout())?;
    let mut app = App::new();

    // Main loop
    let tick_rate = Duration::from_millis(250); // 4 FPS
    let mut last_tick = Instant::now();

    loop {
        // Get terminal size
        let size = renderer.size()?;
        let area = Rect::new(0, 0, size.width, size.height);
        let mut buffer = Buffer::new(area);

        // Render UI
        app.render(&mut buffer, area);

        // Draw to terminal
        renderer.draw(&buffer)?;
        renderer.flush()?;

        // Handle events
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if let Some(event) = renderer.poll_event(timeout) {
            match event {
                Event::Key(key_event) => {
                    if let KeyCode::Char(c) = key_event.code {
                        app.on_key(c);
                    }
                }
                Event::Resize(_, _) => {
                    // Terminal was resized, will re-render on next iteration
                }
                _ => {}
            }
        }

        // Tick
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        // Check quit
        if app.should_quit {
            break;
        }
    }

    // Cleanup
    renderer.cleanup()?;
    println!("Thanks for using fusabi-tui-runtime!");

    Ok(())
}
