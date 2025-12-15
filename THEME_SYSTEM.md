# Theme System

The fusabi-tui-runtime now includes a comprehensive theme system that allows for consistent styling across all TUI widgets and components.

## Overview

The theme system consists of three main components:

- **ColorPalette** - Defines the base color palette (background, foreground, primary, secondary, accent, error, warning, success)
- **StyleMap** - Maps semantic names to specific styles for different UI elements
- **Theme** - Combines palette and styles with metadata into a complete theme

## Features

- **Built-in Themes**: Dark, Light, and Slime (matching Scarab terminal)
- **Custom Themes**: Create your own themes programmatically or via TOML files
- **TOML Support**: Load and save themes from/to TOML configuration files (requires `serde` feature)
- **Widget Integration**: Widgets can apply theme styles with the `.themed()` method
- **Runtime Switching**: Change themes at runtime without rebuilding

## Usage

### Using Built-in Themes

```rust
use fusabi_tui_core::theme::Theme;

// Create a dark theme
let dark = Theme::dark();

// Create a light theme
let light = Theme::light();

// Create the Slime theme (from Scarab terminal)
let slime = Theme::slime();
```

### Creating Custom Themes

```rust
use fusabi_tui_core::theme::{Theme, ColorPalette};
use fusabi_tui_core::style::Color;

let palette = ColorPalette {
    background: Color::Black,
    foreground: Color::White,
    primary: Color::Blue,
    secondary: Color::Cyan,
    accent: Color::Green,
    error: Color::Red,
    warning: Color::Yellow,
    success: Color::Green,
};

let theme = Theme::new("My Theme", palette);
```

### Applying Themes to Widgets

```rust
use fusabi_tui_core::theme::Theme;
use fusabi_tui_widgets::block::Block;
use fusabi_tui_widgets::borders::Borders;

let theme = Theme::dark();

let block = Block::default()
    .title("Themed Panel")
    .borders(Borders::ALL)
    .themed(&theme);  // Apply theme styles
```

### Getting Specific Styles

```rust
let theme = Theme::dark();

// Get predefined styles
let title_style = theme.get_style("title");
let border_style = theme.get_style("border");
let error_style = theme.error_style();
let success_style = theme.success_style();

// Get base style
let base = theme.base_style();
```

### Loading Themes from TOML

Enable the `serde` feature in your `Cargo.toml`:

```toml
[dependencies]
fusabi-tui-core = { version = "0.1", features = ["serde"] }
```

Then load a theme:

```rust
use fusabi_tui_core::theme::Theme;

// From a TOML string
let toml_str = r#"
    name = "My Theme"

    [colors]
    background = "Black"
    foreground = "White"
    primary = "Blue"
    secondary = "Cyan"
    accent = "Magenta"
    error = "Red"
    warning = "Yellow"
    success = "Green"
"#;

let theme = Theme::from_toml(toml_str).unwrap();

// From a TOML file
let theme = Theme::from_toml_file("theme.toml").unwrap();
```

### Saving Themes to TOML

```rust
let theme = Theme::slime();

// Save to string
let toml_str = theme.to_toml().unwrap();
println!("{}", toml_str);

// Save to file
theme.to_toml_file("slime.toml").unwrap();
```

## Built-in Theme Details

### Dark Theme

A dark theme with subtle colors suitable for low-light environments:

- Background: Black
- Foreground: White
- Primary: Blue
- Secondary: Cyan
- Accent: Magenta

### Light Theme

A light theme suitable for bright environments:

- Background: White
- Foreground: Black
- Primary: Blue
- Secondary: Cyan
- Accent: Magenta

### Slime Theme

The Slime theme from Scarab terminal emulator with green/cyan accents:

- Background: `#1e2324` (dark gray-green)
- Foreground: `#e0e0e0` (light gray)
- Primary: `#a8df5a` (slime green)
- Secondary: `#80B5B3` (cyan)
- Accent: `#AEC199` (muted green)
- Error: `#cd6564` (red)
- Warning: `#fff099` (yellow)
- Success: `#AEC199` (green)

## Available Style Names

The following style names are available via `theme.get_style()`:

### Basic Styles
- `base` - Base style with foreground on background
- `title` - Title text style
- `border` - Border style
- `text` - Regular text style

### Interactive Element Styles
- `selected` - Selected item style
- `highlight` - Highlighted text style
- `focus` - Focused element style

### Status Styles
- `error` - Error message style
- `warning` - Warning message style
- `success` - Success message style

### Widget-Specific Styles
- `gauge_bar` - Gauge/progress bar style
- `gauge_label` - Gauge label style
- `table_header` - Table header style
- `table_selected` - Selected table row style
- `list_selected` - Selected list item style
- `tab_active` - Active tab style
- `tab_inactive` - Inactive tab style

## Examples

Run the theme demo:

```bash
cargo run -p fusabi-tui-widgets --example theme_demo
```

With serde support:

```bash
cargo run -p fusabi-tui-widgets --features fusabi-tui-core/serde --example theme_demo
```

## Color Format

Colors can be specified in several formats:

- **Named colors**: `"Black"`, `"Red"`, `"Green"`, `"Yellow"`, `"Blue"`, `"Magenta"`, `"Cyan"`, `"White"`, `"DarkGray"`, `"LightRed"`, etc.
- **RGB**: `{ Rgb = [255, 128, 0] }`
- **Indexed**: `{ Indexed = 42 }`
- **Reset**: `"Reset"`

When using TOML, use the string format for named colors.

## Architecture

The theme system is implemented in `fusabi-tui-core` and is available to all widgets in `fusabi-tui-widgets`. The `serde` feature is optional and adds TOML serialization/deserialization support.

```
fusabi-tui-core
└── theme module
    ├── ColorPalette
    ├── StyleMap (HashMap<String, Style>)
    └── Theme
        ├── Built-in themes (dark, light, slime)
        ├── TOML loading/saving (with serde feature)
        └── Style helpers
```

## Future Enhancements

Potential future improvements to the theme system:

1. Hot-reloading themes from files
2. Theme inheritance (extend existing themes)
3. Per-widget theme overrides
4. Theme validation
5. Theme preview/generation tools
6. Additional built-in themes
7. Theme color interpolation for animations
