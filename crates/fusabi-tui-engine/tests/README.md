# Fusabi TUI Testing Framework

Integration testing framework for Fusabi TUI applications using `ratatui-testlib`.

## Overview

This testing framework provides comprehensive PTY-based integration testing for Fusabi TUI applications. It wraps `ratatui-testlib` with Fusabi-specific utilities and provides:

- **Test Harness Adapter** - Specialized harness for Fusabi TUI apps
- **PTY Testing Utilities** - Command builders and terminal profiles
- **Screenshot Comparison** - Snapshot testing and visual regression detection
- **Input Simulation** - High-level input sequences and common patterns

## Quick Start

### Basic Test Example

```rust
use fusabi_tui_test::{FusabiTuiHarness, FusabiExampleBuilder};

#[test]
fn test_my_app() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test harness
    let mut harness = FusabiTuiHarness::new(80, 24)?;

    // Build command to run example
    let cmd = FusabiExampleBuilder::new("basic_app").build();

    // Spawn the application
    harness.spawn(cmd)?;

    // Wait for UI to render
    harness.wait_for_title("Fusabi TUI")?;

    // Verify content
    harness.assert_contains("Welcome")?;

    // Simulate user input
    harness.send_char('q')?;
    harness.wait_for_exit()?;

    Ok(())
}
```

## Modules

### `harness` - Test Harness Adapter

The `FusabiTuiHarness` wraps `ratatui_testlib::TuiTestHarness` with Fusabi-specific helpers:

```rust
use fusabi_tui_test::FusabiTuiHarness;

let mut harness = FusabiTuiHarness::new(80, 24)?;

// Spawn app
harness.spawn(cmd)?;

// Wait for specific UI elements
harness.wait_for_title("My App")?;
harness.wait_for_counter(5)?;  // Fusabi-specific
harness.wait_for_uptime(1)?;   // Fusabi-specific

// Assertions
harness.assert_contains("text")?;
harness.assert_not_contains("error")?;
harness.assert_cursor_at(10, 5)?;
harness.assert_block_title("Header")?;
```

### `pty_utils` - PTY Testing Utilities

Utilities for building commands and testing with different terminal profiles:

```rust
use fusabi_tui_test::{FusabiExampleBuilder, TerminalPreset};

// Build command for an example
let cmd = FusabiExampleBuilder::new("dashboard")
    .with_release()
    .with_feature("hot-reload")
    .with_env("RUST_LOG", "debug")
    .build();

// Test with different terminal sizes
let (width, height) = TerminalPreset::Large.dimensions();
let mut harness = FusabiTuiHarness::new(width, height)?;
```

**Available Terminal Presets:**
- `Standard` - 80x24 (most compatible)
- `Large` - 120x40
- `Small` - 40x12 (stress test)
- `Wide` - 200x24
- `Tall` - 80x60

### `snapshot` - Screenshot Comparison

Snapshot testing and visual regression detection:

```rust
use fusabi_tui_test::{ScreenSnapshot, normalize_screen};

// Capture a snapshot
let snapshot = ScreenSnapshot::from_state(harness.state());

// Normalize dynamic content for comparison
let normalized = normalize_screen(
    &snapshot.contents,
    &[
        (r"Counter: \d+", "Counter: <N>"),
        (r"Uptime: \d+s", "Uptime: <N>s"),
    ],
);

// Compare snapshots
let comparison = snapshot1.compare(&snapshot2);
if !comparison.is_match() {
    comparison.print_report();
}

// Test specific regions
let mut snapshot = ScreenSnapshot::from_state(harness.state());
snapshot.add_region("header", 0, 0, 80, 3);
snapshot.add_region("footer", 0, 21, 80, 3);
snapshot.compare_region("header", &other_snapshot)?;
```

### `input` - Input Simulation

High-level input simulation with common patterns:

```rust
use fusabi_tui_test::{CommonInputs, InputBuilder, InputSequence};

// Use common input sequences
harness.send_char('q')?;  // Simple quit

// Or use pre-built sequences
let sequence = CommonInputs::navigate_down(3);
let sequence = CommonInputs::select_item(2);
let sequence = CommonInputs::search("query");

// Build custom sequences
let sequence = InputBuilder::new()
    .down(3)
    .enter()
    .wait("Result".to_string())
    .type_text("hello")
    .pause(Duration::from_millis(100))
    .build();

// Execute sequence
for step in sequence.steps() {
    match step {
        InputStep::Key(key) => harness.send_key(*key)?,
        InputStep::Text(text) => harness.send_text(text)?,
        InputStep::Delay(duration) => std::thread::sleep(*duration),
        InputStep::WaitForText(text) => harness.wait_for_text(text)?,
    }
}
```

## Running Tests

### Run all tests

```bash
cd crates/fusabi-tui-engine
cargo test
```

### Run specific test file

```bash
cargo test --test basic_app_test
cargo test --test dashboard_test
```

### Run specific test

```bash
cargo test test_basic_app_startup
```

### Run with output

```bash
cargo test -- --nocapture
```

### Run in release mode (faster)

```bash
cargo test --release
```

## Writing Tests

### Test Structure

```rust
mod mod;  // Import test helpers

use fusabi_tui_test::*;

#[test]
fn test_something() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create harness
    let mut harness = FusabiTuiHarness::new(80, 24)?;

    // 2. Build command
    let cmd = FusabiExampleBuilder::new("my_app").build();

    // 3. Spawn app
    harness.spawn(cmd)?;

    // 4. Wait for initial render
    harness.wait_for_text("Ready")?;

    // 5. Interact and verify
    harness.send_char('x')?;
    harness.assert_contains("Expected")?;

    // 6. Clean shutdown
    harness.send_char('q')?;
    harness.wait_for_exit()?;

    Ok(())
}
```

### Best Practices

1. **Always wait for initial render** before sending input
2. **Use specific wait conditions** instead of fixed sleeps
3. **Test different terminal sizes** for responsive layouts
4. **Normalize dynamic content** in snapshots (timestamps, counters)
5. **Clean up properly** by quitting the app at the end
6. **Use meaningful test names** that describe what is being tested

### Common Patterns

**Testing counter updates:**
```rust
harness.wait_for_counter(5)?;  // Wait for at least 5 increments
```

**Testing uptime:**
```rust
harness.wait_for_uptime(2)?;  // Wait for at least 2 seconds
```

**Testing layout:**
```rust
let snapshot = ScreenSnapshot::from_state(harness.state());
assert_eq!(snapshot.width, 80);
assert_eq!(snapshot.height, 24);
```

**Testing navigation:**
```rust
harness.send_key(KeyCode::Down)?;
std::thread::sleep(Duration::from_millis(100));
harness.update_state()?;
```

**Testing with multiple terminal sizes:**
```rust
for preset in [TerminalPreset::Standard, TerminalPreset::Large] {
    let (w, h) = preset.dimensions();
    let mut harness = FusabiTuiHarness::new(w, h)?;
    // ... test logic ...
}
```

## CI Integration

### GitHub Actions

Add this to your `.github/workflows/test.yml`:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
      - name: Run tests
        run: cargo test --workspace
      - name: Run integration tests
        run: cargo test -p fusabi-tui-engine
```

### Docker

Tests work in Docker environments (headless):

```dockerfile
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo test --workspace
```

## Performance Testing

Track performance metrics during tests:

```rust
use fusabi_tui_test::PerformanceMetrics;

let mut metrics = PerformanceMetrics::new();

for _ in 0..10 {
    harness.update_state()?;
    metrics.record_update();
}

metrics.print_summary();
```

## Troubleshooting

### Test hangs or times out

- Increase timeout: `FusabiTuiHarness::with_timeout(80, 24, Duration::from_secs(30))?`
- Check that the app actually outputs the expected text
- Add debug output: `println!("{}", harness.screen_contents());`

### Text not found on screen

- Update screen state: `harness.update_state()?`
- Wait longer: `std::thread::sleep(Duration::from_millis(500))`
- Check exact string: screen contents might have extra whitespace

### Process doesn't exit

- Ensure you're sending the correct quit key
- Check app's event handler
- Use `is_running()` to debug: `assert!(!harness.is_running())`

### Terminal size issues

- Verify with: `let (w, h) = harness.state().size();`
- Apps might have minimum size requirements
- Test with different `TerminalPreset` values

## Example Tests

See the example tests in this directory:

- `basic_app_test.rs` - Simple app testing
- `dashboard_test.rs` - Complex multi-widget app testing

## API Reference

Full API documentation:

```bash
cargo doc --open -p fusabi-tui-engine
```

## Further Reading

- [ratatui-testlib Documentation](https://github.com/raibid-labs/ratatui-testlib)
- [Fusabi TUI Runtime Guide](../README.md)
- [PTY Testing Best Practices](https://github.com/raibid-labs/ratatui-testlib/blob/main/docs/ARCHITECTURE.md)
