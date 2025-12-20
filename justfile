# Fusabi TUI Runtime - Development Tasks
# Run `just --list` to see all available recipes

# Default recipe - show help
default:
    @just --list

# ============ Build & Test ============

# Build all crates
build:
    cargo build --workspace

# Build in release mode
build-release:
    cargo build --workspace --release

# Run all tests
test:
    cargo test --workspace

# Run tests with output
test-verbose:
    cargo test --workspace -- --nocapture

# Run a specific test
test-one NAME:
    cargo test --workspace {{NAME}} -- --nocapture

# ============ Examples ============

# Run basic TUI app example
example-basic:
    cargo run --example basic_app -p fusabi-tui-engine

# Run dashboard example
example-dashboard:
    cargo run --example dashboard -p fusabi-tui-engine

# Run hot reload demo
example-hot-reload:
    cargo run --example hot_reload_demo -p fusabi-tui-engine

# Run error overlay demo
example-error-overlay:
    cargo run --example error_overlay_demo -p fusabi-tui-engine

# Run block widget demo
example-block:
    cargo run --example block_demo -p fusabi-tui-widgets

# Run theme demo
example-theme:
    cargo run --example theme_demo -p fusabi-tui-widgets

# Run Scarab plugin example
example-scarab:
    cargo run --example scarab_plugin -p fusabi-tui-scarab

# List all available examples
examples:
    @echo "Available examples:"
    @echo "  just example-basic        - Basic TUI app"
    @echo "  just example-dashboard    - Dashboard with widgets"
    @echo "  just example-hot-reload   - Hot reload demo"
    @echo "  just example-error-overlay - Error overlay demo"
    @echo "  just example-block        - Block widget demo"
    @echo "  just example-theme        - Theme system demo"
    @echo "  just example-scarab       - Scarab shared memory"

# ============ Code Quality ============

# Format all code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints
lint:
    cargo clippy --workspace --all-targets

# Run clippy with warnings as errors
lint-strict:
    cargo clippy --workspace --all-targets -- -D warnings

# Check for security vulnerabilities
audit:
    cargo audit

# ============ Documentation ============

# Build documentation
doc:
    cargo doc --workspace --no-deps

# Build and open documentation
doc-open:
    cargo doc --workspace --no-deps --open

# ============ Development ============

# Full check (format, lint, test)
check: fmt-check lint test
    @echo "All checks passed!"

# Pre-commit check
pre-commit: fmt lint test
    @echo "Ready to commit!"

# Clean build artifacts
clean:
    cargo clean

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch for changes and run a specific example
watch-example NAME:
    cargo watch -x "run --example {{NAME}} -p fusabi-tui-engine"

# ============ Release ============

# Check if ready for release
release-check: fmt-check lint-strict test doc
    @echo "Release checks passed!"

# Dry-run publish to crates.io
publish-dry:
    cargo publish --dry-run -p fusabi-tui-core
    cargo publish --dry-run -p fusabi-tui-render
    cargo publish --dry-run -p fusabi-tui-widgets
    cargo publish --dry-run -p fusabi-tui-engine
    cargo publish --dry-run -p fusabi-tui-scarab
