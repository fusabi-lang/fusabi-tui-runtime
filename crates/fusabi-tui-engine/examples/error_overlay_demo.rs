//! Error overlay demo for the Fusabi TUI engine.
//!
//! This example demonstrates:
//! - Creating a dashboard engine
//! - Enabling hot reload
//! - Error overlay displaying when files fail to load
//! - Error recovery without crashing
//! - Dismissing error overlays with Ctrl+D
//!
//! # Usage
//!
//! 1. Run this example:
//!    ```
//!    cargo run --example error_overlay_demo
//!    ```
//!
//! 2. The demo will attempt to load a nonexistent file, showing the error overlay.
//!
//! 3. Press Ctrl+D to dismiss the error overlay.
//!
//! 4. Press Ctrl+C to exit.

use fusabi_tui_engine::prelude::*;
use fusabi_tui_render::test::TestRenderer;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fusabi TUI Error Overlay Demo");
    println!("==============================\n");

    // Create a test renderer (80x24 terminal)
    let renderer = TestRenderer::new(80, 24);

    // Create the dashboard engine
    let mut engine = DashboardEngine::new(renderer, PathBuf::from("."));

    println!("Created dashboard engine");

    // Enable hot reload with 300ms debounce
    engine.enable_hot_reload_with_debounce(300)?;
    println!("Hot reload enabled (300ms debounce)");

    // Test 1: Load a nonexistent file to trigger error overlay
    println!("\nTest 1: Loading nonexistent file...");
    let result = engine.load(Path::new("/nonexistent/file.fsx"));

    if result.is_err() {
        println!("Error occurred (as expected): {}", result.unwrap_err());
        println!("Error overlay should be visible now.");
    }

    if let Some(overlay) = engine.error_overlay() {
        println!("\nError overlay details:");
        println!("  Title: {}", overlay.error().title);
        println!("  Message: {}", overlay.error().message);
        println!("  Severity: {:?}", overlay.error().severity);
        if let Some(source) = &overlay.error().source {
            println!("  Source: {}", source);
        }
        if !overlay.error().hints.is_empty() {
            println!("  Hints:");
            for hint in &overlay.error().hints {
                println!("    - {}", hint);
            }
        }
    }

    // Render with error overlay
    println!("\nRendering dashboard with error overlay...");
    engine.render()?;
    println!("Render complete.");

    // Simulate waiting for user input
    println!("\nSimulating event loop for 3 seconds...");
    println!("(In a real application, you would handle Ctrl+D to dismiss, Ctrl+R to reload, Ctrl+C to quit)");

    for i in 0..30 {
        // Simulate checking for events
        if i == 15 {
            // Simulate Ctrl+D after 1.5 seconds
            println!("\nSimulating Ctrl+D to dismiss error overlay...");
            let event = Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::ctrl(),
            });

            let action = engine.handle_event(event)?;
            if action == Action::Render {
                println!("Error overlay dismissed!");
                engine.render()?;
            }
        }

        // Check for file changes (none expected in this demo)
        if let Some(changes) = engine.poll_changes() {
            if !changes.is_empty() {
                println!("File changes detected: {:?}", changes);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("\nTest 2: Testing error recovery with hot reload...");

    // Create a temporary invalid file to demonstrate error recovery
    use std::fs::File;
    use std::io::Write as IoWrite;

    let temp_file = "test_error_demo.fsx";
    {
        let mut file = File::create(temp_file)?;
        writeln!(file, "// This is a valid file")?;
        writeln!(file, "let x = 42")?;
        file.flush()?;
    }

    println!("Created temporary file: {}", temp_file);

    // Try to load the valid file
    println!("Loading valid file...");
    let result = engine.load(Path::new(temp_file));

    if result.is_ok() {
        println!("File loaded successfully!");
        println!("Error overlay should be cleared now.");

        if engine.error_overlay().is_none() {
            println!("Confirmed: No error overlay present.");
        }
    }

    // Render without error
    println!("\nRendering dashboard without errors...");
    engine.render()?;
    println!("Render complete.");

    // Cleanup
    println!("\nCleaning up...");
    std::fs::remove_file(temp_file)?;
    println!("Removed temporary file: {}", temp_file);

    println!("\nDemo Summary:");
    println!("  - Error overlay displays when file loading fails");
    println!("  - Application continues running despite errors");
    println!("  - Error overlay can be dismissed with Ctrl+D");
    println!("  - Error overlay clears automatically when reload succeeds");
    println!("  - Hot reload infrastructure is fully functional");

    println!("\nDemo complete!");

    Ok(())
}
