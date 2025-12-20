# Fusabi TUI Tutorials

Step-by-step guides for building TUI applications with fusabi-tui-runtime.

## Tutorial Series

| Tutorial | Description | Time |
|----------|-------------|------|
| [01. Hello World](01-hello-world.md) | Your first TUI app - rendering text, handling input | 10 min |
| [02. Interactive UI](02-interactive-ui.md) | State management, lists with selection, layouts | 15 min |
| [03. Tables and Data](03-tables-data.md) | Display structured data, column styling, dynamic updates | 20 min |
| [04. Hot Reload](04-hot-reload.md) | Live dashboard updates, file watching, error handling | 15 min |

## Prerequisites

- **Rust 1.75+** installed
- Basic Rust knowledge (ownership, structs, enums)
- A terminal emulator (iTerm2, Windows Terminal, etc.)

## Quick Start

If you're new to fusabi-tui-runtime, start with [Tutorial 01: Hello World](01-hello-world.md).

```bash
# Create a new project
cargo new my-tui-app
cd my-tui-app

# Add dependencies
cat >> Cargo.toml << 'EOF'
fusabi-tui-core = "0.2"
fusabi-tui-widgets = "0.2"
fusabi-tui-render = { version = "0.2", features = ["crossterm-backend"] }
crossterm = "0.28"
EOF

# Follow Tutorial 01...
```

## Learning Path

### Beginner Track
1. **Hello World** - Understand the render loop
2. **Interactive UI** - Add user interaction

### Intermediate Track
3. **Tables and Data** - Complex data display
4. **Hot Reload** - Live development workflow

### Advanced Topics (Coming Soon)
- Custom widgets
- Shared memory rendering (Scarab)
- Fusabi scripting integration
- Performance optimization

## Code Examples

Complete code for each tutorial is available in:
```
crates/fusabi-tui-engine/examples/
```

Run examples with:
```bash
cargo run --example basic_app -p fusabi-tui-engine
cargo run --example dashboard -p fusabi-tui-engine
```

## Getting Help

- **API Reference**: [docs/api-reference.md](../api-reference.md)
- **Troubleshooting**: [docs/TROUBLESHOOTING.md](../TROUBLESHOOTING.md)
- **Widget Gallery**: [crates/fusabi-tui-widgets/README.md](../../crates/fusabi-tui-widgets/README.md)
- **Issues**: [GitHub Issues](https://github.com/fusabi-lang/fusabi-tui-runtime/issues)

## Feedback

Found an issue with a tutorial? Please open an issue with:
- Which tutorial
- The specific step
- What went wrong
- Your Rust version and OS
