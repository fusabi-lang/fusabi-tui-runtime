# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-14

### Added

#### fusabi-tui-core (v0.1.0)
- Initial release of core TUI primitives
- `Cell` type for individual character cells with styling
- `Buffer` type for 2D grid of cells
- `Rect` type for rectangular areas
- `Layout` type with Cassowary constraint solver
- `Constraint` enum for flexible layout constraints
- `Style` type for text styling with colors and modifiers
- `Color` enum supporting 16 basic colors, 256-color palette, and RGB
- `Modifier` bitflags for text attributes (bold, italic, underlined, etc.)
- `symbols` module with Unicode characters for borders and UI elements
- Comprehensive documentation and examples

#### fusabi-tui-render (v0.1.0)
- Initial release of renderer abstraction layer
- `Renderer` trait for backend-agnostic rendering
- `CrosstermRenderer` for standalone terminal rendering (feature: crossterm-backend)
- `TestRenderer` for in-memory testing
- Built-in event polling support
- Error handling with `RenderError` type
- Thread-safe design (Send + Sync)
- Comprehensive documentation and examples

#### fusabi-tui-widgets (v0.1.0)
- Initial release of widget library
- `Widget` trait for stateless widgets
- `StatefulWidget` trait for widgets with state
- `Block` widget for bordered containers
- `Paragraph` widget for multi-line text with wrapping
- `List` widget for scrollable lists with selection
- `Table` widget for tabular data display
- `Gauge` widget for progress bars
- `Sparkline` widget for inline mini-charts
- `Tabs` widget for tab navigation
- `Text`, `Line`, and `Span` types for rich text formatting
- `Borders` and `BorderType` for border customization
- Comprehensive documentation and examples

#### fusabi-tui-engine (v0.1.0)
- Initial release of hot reload engine
- `DashboardEngine` for application orchestration
- `FileWatcher` for file system monitoring with debouncing
- `FileLoader` for smart file loading with caching
- `Event` system for keyboard, mouse, and custom events
- `Action` system for event handling results
- `DashboardState` for state management
- `ListState` and `TableState` for widget state
- Hot reload support with dependency tracking
- Development overlay for debugging
- Async tokio integration
- Comprehensive documentation and examples

#### fusabi-tui-scarab (v0.1.0)
- Initial release of Scarab shared memory backend
- `ScarabRenderer` for zero-copy rendering via shared memory
- Lock-free synchronization using sequence numbers
- Type-safe conversions between Fusabi and Scarab types
- Plugin API for interactive TUI applications
- Full `Renderer` trait implementation
- Compatible with Scarab's split-process architecture
- `#[repr(C)]` memory layout guarantees
- Comprehensive documentation and examples

#### Documentation
- Workspace README.md with project overview
- MIGRATION.md with guides for migrating from ratatui and old fusabi-tui
- Individual README.md files for each crate
- Architecture documentation in docs/ directory
- Ratatui analysis document
- Design documents for Fusabi enhancements and hot reload
- Package management design document
- Ecosystem overview document

#### Infrastructure
- Cargo workspace configuration
- MIT OR Apache-2.0 dual licensing
- Consistent versioning across all crates
- Test infrastructure with unit tests
- CI/CD ready structure

### Changed
- N/A (initial release)

### Deprecated
- N/A (initial release)

### Removed
- N/A (initial release)

### Fixed
- N/A (initial release)

### Security
- N/A (initial release)

---

## Release Notes

### v0.1.0 - Initial Release

This is the first release of fusabi-tui-runtime, a unified TUI framework for Fusabi applications. The framework provides:

**Core Features:**
- Dual-mode runtime: standalone (crossterm) and plugin (Scarab shared memory)
- Hot reload support for rapid development iteration
- Comprehensive widget library with 10+ widgets
- Flexible constraint-based layout system
- Rich text styling with colors and modifiers
- Type-safe, zero-copy rendering

**Architecture:**
The framework is split into 5 crates for modularity:
- `fusabi-tui-core`: Core primitives (~2K LOC from ratatui-core)
- `fusabi-tui-render`: Renderer abstraction with multiple backends
- `fusabi-tui-widgets`: Widget library with composable components
- `fusabi-tui-engine`: Hot reload engine with file watching
- `fusabi-tui-scarab`: Scarab shared memory integration

**Migration:**
Users migrating from ratatui or old fusabi-tui should consult MIGRATION.md for detailed guidance on updating imports, replacing Terminal with Renderer, and leveraging new features like hot reload.

**Known Limitations:**
- Fusabi VM integration not yet implemented
- Fusabi Frontend compilation not yet implemented
- Package management (fpm) not yet available
- Limited example applications

**Future Plans:**
- Integration with Fusabi VM for .fzb bytecode execution
- Integration with Fusabi Frontend for .fsx compilation
- Package registry for sharing widgets and themes
- Additional widgets and layout features
- Performance optimizations

---

[Unreleased]: https://github.com/fusabi-lang/fusabi-tui-runtime/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/fusabi-lang/fusabi-tui-runtime/releases/tag/v0.1.0
