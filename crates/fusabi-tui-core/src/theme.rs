//! Theme system for styling TUI applications.
//!
//! This module provides a comprehensive theming system that allows for consistent
//! styling across all TUI widgets and components. Themes define color palettes,
//! semantic color mappings, and can be loaded from TOML configuration files.
//!
//! # Overview
//!
//! The theme system consists of three main components:
//!
//! - [`ColorPalette`] - Defines the base color palette (background, foreground, accents, etc.)
//! - [`StyleMap`] - Maps semantic names to specific styles for different UI elements
//! - [`Theme`] - Combines palette and styles with metadata into a complete theme
//!
//! # Examples
//!
//! ```rust
//! use fusabi_tui_core::theme::{Theme, ColorPalette, StyleMap};
//! use fusabi_tui_core::style::{Color, Style};
//!
//! // Create a custom theme
//! let palette = ColorPalette {
//!     background: Color::Black,
//!     foreground: Color::White,
//!     primary: Color::Blue,
//!     secondary: Color::Cyan,
//!     accent: Color::Green,
//!     error: Color::Red,
//!     warning: Color::Yellow,
//!     success: Color::Green,
//! };
//!
//! let mut styles = StyleMap::new();
//! styles.insert("title".to_string(), Style::new().fg(Color::Blue));
//!
//! let theme = Theme {
//!     name: "My Theme".to_string(),
//!     colors: palette,
//!     styles,
//! };
//! ```
//!
//! # Built-in Themes
//!
//! The module provides several built-in themes:
//!
//! - [`Theme::dark()`] - A dark theme with subtle colors
//! - [`Theme::light()`] - A light theme suitable for bright environments
//! - [`Theme::slime()`] - The Slime theme from Scarab terminal with green accents

use crate::style::{Color, Style};
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A color palette defining the base colors for a theme.
///
/// This struct contains all the fundamental colors used throughout the UI.
/// Individual widgets can reference these colors to maintain consistency.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColorPalette {
    /// Default background color
    pub background: Color,
    /// Default foreground (text) color
    pub foreground: Color,
    /// Primary accent color (used for interactive elements)
    pub primary: Color,
    /// Secondary accent color (used for highlights)
    pub secondary: Color,
    /// Accent color (used for emphasis)
    pub accent: Color,
    /// Error state color
    pub error: Color,
    /// Warning state color
    pub warning: Color,
    /// Success state color
    pub success: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::dark()
    }
}

impl ColorPalette {
    /// Creates a dark color palette.
    pub fn dark() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            primary: Color::Blue,
            secondary: Color::Cyan,
            accent: Color::Magenta,
            error: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
        }
    }

    /// Creates a light color palette.
    pub fn light() -> Self {
        Self {
            background: Color::White,
            foreground: Color::Black,
            primary: Color::Blue,
            secondary: Color::Cyan,
            accent: Color::Magenta,
            error: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
        }
    }

    /// Creates the Slime theme color palette.
    ///
    /// Based on the Slime theme from Scarab terminal emulator.
    /// Features a dark background with green/cyan accents.
    pub fn slime() -> Self {
        Self {
            background: Color::Rgb(30, 35, 36),        // #1e2324
            foreground: Color::Rgb(224, 224, 224),     // #e0e0e0
            primary: Color::Rgb(168, 223, 90),         // #a8df5a (slime green)
            secondary: Color::Rgb(128, 181, 179),      // #80B5B3 (cyan)
            accent: Color::Rgb(174, 193, 153),         // #AEC199 (green)
            error: Color::Rgb(205, 101, 100),          // #cd6564 (red)
            warning: Color::Rgb(255, 240, 153),        // #fff099 (yellow)
            success: Color::Rgb(174, 193, 153),        // #AEC199 (green)
        }
    }
}

/// A map of semantic style names to actual styles.
///
/// This allows widgets to reference styles by name (e.g., "title", "border", "selected")
/// without hardcoding specific colors. The theme can override these mappings.
pub type StyleMap = HashMap<String, Style>;

/// A complete theme combining colors, styles, and metadata.
///
/// Themes provide a consistent look and feel across the entire application.
/// They can be created programmatically or loaded from configuration files.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Color palette
    pub colors: ColorPalette,
    /// Named style mappings
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "StyleMap::is_empty"))]
    pub styles: StyleMap,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Creates a new theme with the given name and color palette.
    ///
    /// Default styles will be generated based on the color palette.
    pub fn new(name: impl Into<String>, colors: ColorPalette) -> Self {
        let name = name.into();
        let styles = Self::default_styles(&colors);
        Self {
            name,
            colors,
            styles,
        }
    }

    /// Creates the default dark theme.
    pub fn dark() -> Self {
        Self::new("Dark", ColorPalette::dark())
    }

    /// Creates a light theme.
    pub fn light() -> Self {
        Self::new("Light", ColorPalette::light())
    }

    /// Creates the Slime theme.
    ///
    /// Based on the Slime theme from Scarab terminal emulator.
    /// Features a dark background with green/cyan accents.
    pub fn slime() -> Self {
        Self::new("Slime", ColorPalette::slime())
    }

    /// Gets a style by name, falling back to default if not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fusabi_tui_core::theme::Theme;
    ///
    /// let theme = Theme::dark();
    /// let title_style = theme.get_style("title");
    /// let border_style = theme.get_style("border");
    /// ```
    pub fn get_style(&self, name: &str) -> Style {
        self.styles
            .get(name)
            .copied()
            .unwrap_or_else(|| Style::new().fg(self.colors.foreground))
    }

    /// Sets a style by name.
    pub fn set_style(&mut self, name: impl Into<String>, style: Style) {
        self.styles.insert(name.into(), style);
    }

    /// Gets the base style (foreground on background).
    pub fn base_style(&self) -> Style {
        Style::new()
            .fg(self.colors.foreground)
            .bg(self.colors.background)
    }

    /// Gets the primary style.
    pub fn primary_style(&self) -> Style {
        Style::new().fg(self.colors.primary)
    }

    /// Gets the secondary style.
    pub fn secondary_style(&self) -> Style {
        Style::new().fg(self.colors.secondary)
    }

    /// Gets the accent style.
    pub fn accent_style(&self) -> Style {
        Style::new().fg(self.colors.accent)
    }

    /// Gets the error style.
    pub fn error_style(&self) -> Style {
        Style::new().fg(self.colors.error)
    }

    /// Gets the warning style.
    pub fn warning_style(&self) -> Style {
        Style::new().fg(self.colors.warning)
    }

    /// Gets the success style.
    pub fn success_style(&self) -> Style {
        Style::new().fg(self.colors.success)
    }

    /// Loads a theme from a TOML string.
    ///
    /// # Errors
    ///
    /// Returns an error if the TOML is invalid or cannot be deserialized.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fusabi_tui_core::theme::Theme;
    ///
    /// let toml_str = r#"
    ///     name = "My Theme"
    ///
    ///     [colors]
    ///     background = "Black"
    ///     foreground = "White"
    ///     primary = "Blue"
    ///     secondary = "Cyan"
    ///     accent = "Magenta"
    ///     error = "Red"
    ///     warning = "Yellow"
    ///     success = "Green"
    /// "#;
    ///
    /// let theme = Theme::from_toml(toml_str).unwrap();
    /// ```
    #[cfg(feature = "serde")]
    pub fn from_toml(toml_str: &str) -> Result<Self, String> {
        let mut theme: Self = toml::from_str(toml_str)
            .map_err(|e| format!("Failed to parse TOML: {}", e))?;

        // Generate default styles if none were provided
        if theme.styles.is_empty() {
            theme.styles = Self::default_styles(&theme.colors);
        }

        Ok(theme)
    }

    /// Loads a theme from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    #[cfg(feature = "serde")]
    pub fn from_toml_file(path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read theme file: {}", e))?;
        Self::from_toml(&content)
    }

    /// Saves a theme to a TOML string.
    ///
    /// # Errors
    ///
    /// Returns an error if the theme cannot be serialized.
    #[cfg(feature = "serde")]
    pub fn to_toml(&self) -> Result<String, String> {
        toml::to_string_pretty(self).map_err(|e| format!("Failed to serialize theme: {}", e))
    }

    /// Saves a theme to a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    #[cfg(feature = "serde")]
    pub fn to_toml_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), String> {
        let content = self.to_toml()?;
        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write theme file: {}", e))
    }

    /// Generates default styles based on the color palette.
    fn default_styles(colors: &ColorPalette) -> StyleMap {
        let mut styles = HashMap::new();

        // Basic styles
        styles.insert(
            "base".to_string(),
            Style::new().fg(colors.foreground).bg(colors.background),
        );
        styles.insert("title".to_string(), Style::new().fg(colors.primary));
        styles.insert("border".to_string(), Style::new().fg(colors.secondary));
        styles.insert("text".to_string(), Style::new().fg(colors.foreground));

        // Interactive element styles
        styles.insert(
            "selected".to_string(),
            Style::new().fg(colors.background).bg(colors.primary),
        );
        styles.insert(
            "highlight".to_string(),
            Style::new().fg(colors.accent),
        );
        styles.insert(
            "focus".to_string(),
            Style::new().fg(colors.primary),
        );

        // Status styles
        styles.insert("error".to_string(), Style::new().fg(colors.error));
        styles.insert("warning".to_string(), Style::new().fg(colors.warning));
        styles.insert("success".to_string(), Style::new().fg(colors.success));

        // Widget-specific styles
        styles.insert(
            "gauge_bar".to_string(),
            Style::new().fg(colors.primary).bg(colors.background),
        );
        styles.insert(
            "gauge_label".to_string(),
            Style::new().fg(colors.foreground),
        );
        styles.insert(
            "table_header".to_string(),
            Style::new().fg(colors.primary),
        );
        styles.insert(
            "table_selected".to_string(),
            Style::new().fg(colors.background).bg(colors.primary),
        );
        styles.insert(
            "list_selected".to_string(),
            Style::new().fg(colors.background).bg(colors.primary),
        );
        styles.insert(
            "tab_active".to_string(),
            Style::new().fg(colors.background).bg(colors.primary),
        );
        styles.insert(
            "tab_inactive".to_string(),
            Style::new().fg(colors.foreground),
        );

        styles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_palette_dark() {
        let palette = ColorPalette::dark();
        assert_eq!(palette.background, Color::Black);
        assert_eq!(palette.foreground, Color::White);
        assert_eq!(palette.primary, Color::Blue);
    }

    #[test]
    fn test_color_palette_light() {
        let palette = ColorPalette::light();
        assert_eq!(palette.background, Color::White);
        assert_eq!(palette.foreground, Color::Black);
    }

    #[test]
    fn test_color_palette_slime() {
        let palette = ColorPalette::slime();
        assert_eq!(palette.background, Color::Rgb(30, 35, 36));
        assert_eq!(palette.foreground, Color::Rgb(224, 224, 224));
        assert_eq!(palette.primary, Color::Rgb(168, 223, 90));
    }

    #[test]
    fn test_theme_dark() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "Dark");
        assert_eq!(theme.colors.background, Color::Black);
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.name, "Light");
        assert_eq!(theme.colors.background, Color::White);
    }

    #[test]
    fn test_theme_slime() {
        let theme = Theme::slime();
        assert_eq!(theme.name, "Slime");
        assert_eq!(theme.colors.background, Color::Rgb(30, 35, 36));
    }

    #[test]
    fn test_theme_get_style() {
        let theme = Theme::dark();
        let title_style = theme.get_style("title");
        assert_eq!(title_style.fg, Some(Color::Blue));

        // Test fallback for non-existent style
        let unknown_style = theme.get_style("unknown");
        assert_eq!(unknown_style.fg, Some(Color::White));
    }

    #[test]
    fn test_theme_set_style() {
        let mut theme = Theme::dark();
        let custom_style = Style::new().fg(Color::Red);
        theme.set_style("custom", custom_style);

        let retrieved_style = theme.get_style("custom");
        assert_eq!(retrieved_style.fg, Some(Color::Red));
    }

    #[test]
    fn test_theme_base_style() {
        let theme = Theme::dark();
        let base = theme.base_style();
        assert_eq!(base.fg, Some(Color::White));
        assert_eq!(base.bg, Some(Color::Black));
    }

    #[test]
    fn test_theme_helper_styles() {
        let theme = Theme::dark();

        assert_eq!(theme.primary_style().fg, Some(Color::Blue));
        assert_eq!(theme.secondary_style().fg, Some(Color::Cyan));
        assert_eq!(theme.accent_style().fg, Some(Color::Magenta));
        assert_eq!(theme.error_style().fg, Some(Color::Red));
        assert_eq!(theme.warning_style().fg, Some(Color::Yellow));
        assert_eq!(theme.success_style().fg, Some(Color::Green));
    }

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme.name, "Dark");
    }

    #[test]
    fn test_color_palette_default() {
        let palette = ColorPalette::default();
        assert_eq!(palette.background, Color::Black);
        assert_eq!(palette.foreground, Color::White);
    }

    #[test]
    fn test_default_styles_generated() {
        let theme = Theme::dark();

        // Verify that default styles are created
        assert!(theme.styles.contains_key("base"));
        assert!(theme.styles.contains_key("title"));
        assert!(theme.styles.contains_key("border"));
        assert!(theme.styles.contains_key("selected"));
        assert!(theme.styles.contains_key("error"));
        assert!(theme.styles.contains_key("warning"));
        assert!(theme.styles.contains_key("success"));
        assert!(theme.styles.contains_key("gauge_bar"));
        assert!(theme.styles.contains_key("table_header"));
        assert!(theme.styles.contains_key("list_selected"));
        assert!(theme.styles.contains_key("tab_active"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_theme_toml_serialization() {
        let theme = Theme::dark();
        let toml_str = theme.to_toml().unwrap();

        // Verify TOML contains expected fields
        assert!(toml_str.contains("name"));
        assert!(toml_str.contains("[colors]"));
        assert!(toml_str.contains("background"));
        assert!(toml_str.contains("foreground"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_theme_toml_roundtrip() {
        let original = Theme::dark();
        let toml_str = original.to_toml().unwrap();
        let parsed = Theme::from_toml(&toml_str).unwrap();

        assert_eq!(original.name, parsed.name);
        assert_eq!(original.colors, parsed.colors);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_theme_from_toml() {
        let toml_str = r#"
            name = "Test Theme"

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
        assert_eq!(theme.name, "Test Theme");
        assert_eq!(theme.colors.background, Color::Black);
        assert_eq!(theme.colors.foreground, Color::White);
        assert_eq!(theme.colors.primary, Color::Blue);
    }
}
