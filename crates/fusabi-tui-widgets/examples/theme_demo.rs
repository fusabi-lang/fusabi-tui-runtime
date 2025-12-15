//! Demonstration of the theme system with runtime switching.
//!
//! This example shows how to use themes with widgets and switch between
//! different themes at runtime.

use fusabi_tui_core::{
    buffer::Buffer,
    layout::Rect,
    theme::{ColorPalette, Theme},
    style::Color,
};
use fusabi_tui_widgets::{
    block::Block,
    borders::{BorderType, Borders},
    widget::Widget,
};

fn main() {
    println!("=== Theme System Demo ===\n");

    // Create buffer for rendering
    let area = Rect::new(0, 0, 60, 20);
    let mut buffer = Buffer::new(area);

    // Demo 1: Dark theme
    println!("Dark Theme:");
    print_theme_demo(&Theme::dark(), &mut buffer);

    // Demo 2: Light theme
    println!("\nLight Theme:");
    print_theme_demo(&Theme::light(), &mut buffer);

    // Demo 3: Slime theme
    println!("\nSlime Theme:");
    print_theme_demo(&Theme::slime(), &mut buffer);

    // Demo 4: Custom theme
    println!("\nCustom Theme:");
    let custom_palette = ColorPalette {
        background: Color::Rgb(40, 40, 40),
        foreground: Color::Rgb(220, 220, 220),
        primary: Color::Rgb(255, 100, 100),
        secondary: Color::Rgb(100, 255, 100),
        accent: Color::Rgb(100, 100, 255),
        error: Color::Red,
        warning: Color::Yellow,
        success: Color::Green,
    };
    let custom_theme = Theme::new("Custom", custom_palette);
    print_theme_demo(&custom_theme, &mut buffer);

    // Demo 5: Theme serialization
    println!("\n=== Theme Serialization ===");
    #[cfg(feature = "fusabi-tui-core/serde")]
    {
        let theme = Theme::slime();
        match theme.to_toml() {
            Ok(toml) => {
                println!("Slime theme as TOML:\n{}", toml);
            }
            Err(e) => println!("Error serializing theme: {}", e),
        }
    }

    #[cfg(not(feature = "fusabi-tui-core/serde"))]
    {
        println!("(serde feature not enabled)");
    }
}

fn print_theme_demo(theme: &Theme, buffer: &mut Buffer) {
    println!("  Theme: {}", theme.name);
    println!("  Colors:");
    println!("    Background: {:?}", theme.colors.background);
    println!("    Foreground: {:?}", theme.colors.foreground);
    println!("    Primary:    {:?}", theme.colors.primary);
    println!("    Secondary:  {:?}", theme.colors.secondary);
    println!("    Accent:     {:?}", theme.colors.accent);
    println!("    Error:      {:?}", theme.colors.error);
    println!("    Warning:    {:?}", theme.colors.warning);
    println!("    Success:    {:?}", theme.colors.success);

    // Render a themed block
    let area = Rect::new(0, 0, 50, 10);
    buffer.clear();

    let block = Block::default()
        .title(format!("{} Theme Demo", theme.name))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .themed(theme);

    block.render(area, buffer);

    println!("\n  Themed Block Created:");
    println!("    Title style:  {:?}", theme.get_style("title"));
    println!("    Border style: {:?}", theme.get_style("border"));
    println!("    Base style:   {:?}", theme.base_style());
}
