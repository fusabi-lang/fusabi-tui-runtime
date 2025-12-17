# Testing Guide for Fusabi TUI Runtime

This guide covers testing strategies and CI integration for Fusabi TUI applications using the ratatui-testlib integration.

## Table of Contents

- [Overview](#overview)
- [Test Types](#test-types)
- [Writing Tests](#writing-tests)
- [CI Integration](#ci-integration)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Overview

The Fusabi TUI runtime provides comprehensive testing support through integration with `ratatui-testlib`. This enables:

- **PTY-based integration testing** - Test full applications in a real terminal environment
- **Input simulation** - Keyboard and mouse event replay
- **Screen verification** - Text matching, cursor position, layout checks
- **Snapshot testing** - Visual regression detection
- **CI/CD ready** - Headless execution in Docker and GitHub Actions

## Test Types

### Unit Tests

Test individual components in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_render() {
        let mut buffer = Buffer::new(Rect::new(0, 0, 10, 5));
        let widget = MyWidget::new();
        widget.render(&mut buffer, buffer.area);
        // Assertions on buffer contents
    }
}
```

### Integration Tests

Test full applications with PTY:

```rust
use fusabi_tui_test::{FusabiTuiHarness, FusabiExampleBuilder};

#[test]
fn test_full_app() -> Result<(), Box<dyn std::error::Error>> {
    let mut harness = FusabiTuiHarness::new(80, 24)?;
    let cmd = FusabiExampleBuilder::new("my_app").build();
    harness.spawn(cmd)?;
    harness.wait_for_text("Ready")?;
    harness.send_char('q')?;
    harness.wait_for_exit()?;
    Ok(())
}
```

### Snapshot Tests

Capture and compare terminal output:

```rust
use fusabi_tui_test::ScreenSnapshot;

#[test]
fn test_ui_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let mut harness = FusabiTuiHarness::new(80, 24)?;
    // ... spawn and wait for render ...

    let snapshot = ScreenSnapshot::from_state(harness.state());
    let normalized = fusabi_tui_test::normalize_screen(
        &snapshot.contents,
        &[(r"Counter: \d+", "Counter: <N>")],
    );

    // With insta crate:
    // insta::assert_snapshot!("my_ui", normalized);

    Ok(())
}
```

## Writing Tests

### Basic Test Structure

```rust
// tests/my_test.rs
mod mod;  // Import test helpers

use fusabi_tui_test::*;
use std::time::Duration;

#[test]
fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup
    let mut harness = FusabiTuiHarness::new(80, 24)?;
    let cmd = FusabiExampleBuilder::new("example").build();

    // 2. Execute
    harness.spawn(cmd)?;
    harness.wait_for_text("Ready")?;

    // 3. Interact
    harness.send_key(KeyCode::Down)?;
    harness.send_key(KeyCode::Enter)?;

    // 4. Verify
    harness.assert_contains("Expected Output")?;

    // 5. Cleanup
    harness.send_char('q')?;
    harness.wait_for_exit()?;

    Ok(())
}
```

### Testing Navigation

```rust
#[test]
fn test_list_navigation() -> Result<(), Box<dyn std::error::Error>> {
    let mut harness = FusabiTuiHarness::new(80, 24)?;
    let cmd = FusabiExampleBuilder::new("list_app").build();
    harness.spawn(cmd)?;

    harness.wait_for_text("Item 0")?;

    // Navigate down
    let sequence = CommonInputs::navigate_down(3);
    for step in sequence.steps() {
        if let InputStep::Key(key) = step {
            harness.send_key(*key)?;
            std::thread::sleep(Duration::from_millis(50));
            harness.update_state()?;
        }
    }

    // Verify selection
    harness.assert_contains("Item 3")?;

    harness.send_char('q')?;
    harness.wait_for_exit()?;
    Ok(())
}
```

### Testing Dynamic Content

```rust
#[test]
fn test_counter_increments() -> Result<(), Box<dyn std::error::Error>> {
    let mut harness = FusabiTuiHarness::new(80, 24)?;
    let cmd = FusabiExampleBuilder::new("counter_app").build();
    harness.spawn(cmd)?;

    // Wait for initial render
    harness.wait_for_text("Counter:")?;

    // Wait for counter to increment using Fusabi-specific helper
    harness.wait_for_counter(5)?;

    // Verify it's not at initial value
    harness.assert_not_contains("Counter: 0")?;

    harness.send_char('q')?;
    harness.wait_for_exit()?;
    Ok(())
}
```

### Testing Multiple Terminal Sizes

```rust
#[test]
fn test_responsive_layout() -> Result<(), Box<dyn std::error::Error>> {
    let presets = vec![
        TerminalPreset::Standard,
        TerminalPreset::Large,
        TerminalPreset::Wide,
    ];

    for preset in presets {
        let (width, height) = preset.dimensions();
        let mut harness = FusabiTuiHarness::new(width, height)?;
        let cmd = FusabiExampleBuilder::new("responsive_app").build();

        harness.spawn(cmd)?;
        harness.wait_for_text("App")?;

        // Verify layout adapts to size
        let (actual_w, actual_h) = harness.state().size();
        assert_eq!(actual_w, width);
        assert_eq!(actual_h, height);

        harness.send_char('q')?;
        harness.wait_for_exit()?;
    }

    Ok(())
}
```

## CI Integration

### GitHub Actions

Create `.github/workflows/test.yml`:

```yaml
name: Tests

on:
  push:
    branches: [main]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Run unit tests
        run: cargo test --lib --workspace

      - name: Run integration tests
        run: cargo test -p fusabi-tui-engine

      - name: Run doc tests
        run: cargo test --doc --workspace
```

### Advanced CI with Coverage

```yaml
name: Tests with Coverage

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
          components: llvm-tools-preview

      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov

      - name: Run tests with coverage
        run: cargo llvm-cov --workspace --lcov --output-path lcov.info

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info
          fail_ci_if_error: true
```

### Docker Integration

`Dockerfile.test`:

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build dependencies
RUN cargo build --release --workspace

# Run tests
RUN cargo test --workspace --release

FROM scratch
# Tests run during build; this is just a marker image
```

Build and test:

```bash
docker build -f Dockerfile.test -t fusabi-tui-test .
```

### GitLab CI

`.gitlab-ci.yml`:

```yaml
image: rust:latest

stages:
  - test
  - integration

variables:
  CARGO_HOME: $CI_PROJECT_DIR/.cargo

cache:
  paths:
    - .cargo/
    - target/

test:unit:
  stage: test
  script:
    - cargo test --lib --workspace

test:integration:
  stage: integration
  script:
    - cargo test -p fusabi-tui-engine
  artifacts:
    reports:
      junit: target/test-results.xml
```

## Best Practices

### 1. Always Wait for Initial Render

```rust
// Good
harness.spawn(cmd)?;
harness.wait_for_text("Ready")?;
harness.send_char('x')?;

// Bad
harness.spawn(cmd)?;
harness.send_char('x')?;  // Might send before app is ready
```

### 2. Use Specific Wait Conditions

```rust
// Good
harness.wait_for_text("Loading complete")?;

// Bad
std::thread::sleep(Duration::from_secs(2));  // Unreliable
```

### 3. Normalize Dynamic Content in Snapshots

```rust
let normalized = normalize_screen(
    &snapshot.contents,
    &[
        (r"\d{4}-\d{2}-\d{2}", "YYYY-MM-DD"),
        (r"Counter: \d+", "Counter: <N>"),
        (r"\d+\.\d+ MB", "<N> MB"),
    ],
);
```

### 4. Test Edge Cases

```rust
// Test minimum size
let mut harness = FusabiTuiHarness::new(40, 12)?;

// Test with invalid input
harness.send_text("\x00\x01\x02")?;
harness.assert_contains("Expected")?;
```

### 5. Clean Up Resources

```rust
// Always quit properly
harness.send_char('q')?;
harness.wait_for_exit()?;

// Or use RAII pattern
struct TestGuard {
    harness: FusabiTuiHarness,
}

impl Drop for TestGuard {
    fn drop(&mut self) {
        let _ = self.harness.send_char('q');
        let _ = self.harness.wait_for_exit();
    }
}
```

## Troubleshooting

### Test Times Out

**Problem**: Test hangs waiting for text that never appears.

**Solutions**:
1. Increase timeout: `FusabiTuiHarness::with_timeout(80, 24, Duration::from_secs(30))?`
2. Check exact text: `println!("{}", harness.screen_contents());`
3. Update screen state: `harness.update_state()?`

### Process Won't Exit

**Problem**: `wait_for_exit()` times out.

**Solutions**:
1. Verify quit key: Check app's event handler
2. Send signal: `harness.send_key(KeyCode::Esc)?`
3. Check if running: `assert!(!harness.is_running())`

### Flaky Tests

**Problem**: Tests pass sometimes but fail randomly.

**Solutions**:
1. Use wait conditions instead of sleeps
2. Add retry logic for transient issues
3. Increase poll interval: `.with_poll_interval(Duration::from_millis(100))`

### CI Failures (Works Locally)

**Problem**: Tests fail in CI but pass locally.

**Solutions**:
1. Check terminal environment in CI
2. Add debug output: `cargo test -- --nocapture`
3. Use headless features if available
4. Increase timeouts for slower CI runners

### Memory Issues

**Problem**: Tests consume too much memory.

**Solutions**:
1. Run tests sequentially: `cargo test -- --test-threads=1`
2. Reduce buffer size: `.with_buffer_size(4096)`
3. Clean up between tests

## Performance Benchmarking

Track test performance:

```rust
use fusabi_tui_test::PerformanceMetrics;

#[test]
fn benchmark_render_performance() -> Result<(), Box<dyn std::error::Error>> {
    let mut harness = FusabiTuiHarness::new(80, 24)?;
    let cmd = FusabiExampleBuilder::new("app").build();
    harness.spawn(cmd)?;

    let mut metrics = PerformanceMetrics::new();
    let start = std::time::Instant::now();

    for _ in 0..100 {
        harness.update_state()?;
        metrics.record_update();
    }

    let elapsed = start.elapsed();
    println!("100 updates in {:?}", elapsed);
    println!("Average: {:?}/update", elapsed / 100);

    assert!(elapsed.as_millis() < 5000, "Too slow");

    harness.send_char('q')?;
    harness.wait_for_exit()?;
    Ok(())
}
```

## Further Resources

- [Test Module README](../crates/fusabi-tui-engine/tests/README.md)
- [ratatui-testlib Documentation](https://github.com/raibid-labs/ratatui-testlib)
- [Example Tests](../crates/fusabi-tui-engine/tests/)

## Contributing

When adding new test utilities:

1. Add to appropriate module in `tests/`
2. Update module exports in `tests/mod.rs`
3. Add documentation with examples
4. Add unit tests for the helper itself
5. Update this guide with new patterns
