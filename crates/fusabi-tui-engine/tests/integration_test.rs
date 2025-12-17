//! Basic integration tests demonstrating ratatui-testlib usage.
//!
//! This file provides simple example tests showing how to use the testing framework.

mod harness;
mod pty_utils;
mod snapshot;
mod input;

use harness::FusabiTuiHarness;
use pty_utils::FusabiExampleBuilder;

#[test]
fn test_harness_creation() {
    // Just verify we can create a harness
    let result = FusabiTuiHarness::new(80, 24);
    assert!(result.is_ok(), "Should be able to create test harness");
}

#[test]
fn test_example_builder() {
    // Verify we can build a command
    let cmd = FusabiExampleBuilder::new("basic_app").build();
    // If we got here, the builder worked
    assert!(true);
}

// Note: Full integration tests that spawn processes are disabled for now
// as they require the examples to be built first. These would typically
// run in CI after building the workspace.
//
// Example of how a full test would look:
//
// #[test]
// #[ignore]  // Ignored by default, run with: cargo test -- --ignored
// fn test_basic_app_full() -> Result<(), Box<dyn std::error::Error>> {
//     let mut harness = FusabiTuiHarness::new(80, 24)?;
//     let cmd = FusabiExampleBuilder::new("basic_app").build();
//     harness.spawn(cmd)?;
//     harness.wait_for_text("Welcome")?;
//     harness.send_char('q')?;
//     Ok(())
// }
