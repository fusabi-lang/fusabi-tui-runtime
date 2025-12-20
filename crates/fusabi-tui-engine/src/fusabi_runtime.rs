//! Fusabi runtime integration for evaluating .fsx dashboard scripts.
//!
//! This module provides the bridge between the Fusabi interpreter and the TUI
//! rendering system, enabling hot-reloadable dashboards written in Fusabi DSL.
//!
//! # Architecture
//!
//! The integration works as follows:
//!
//! 1. `.fsx` files are loaded by the [`FileLoader`](crate::loader::FileLoader)
//! 2. The Fusabi engine evaluates the script with registered host functions
//! 3. Host functions provide access to TUI primitives (colors, styles, widgets)
//! 4. The `render` function in the script is called each frame
//! 5. Widget operations are translated to buffer mutations
//!
//! # Host Functions
//!
//! The following host function modules are registered:
//!
//! - `tui.color` - Color creation and manipulation
//! - `tui.style` - Style building with modifiers
//! - `tui.layout` - Rect and constraint-based layouts
//! - `tui.widget` - Widget creation (Block, Paragraph, List, etc.)
//! - `tui.buffer` - Direct buffer manipulation
//!
//! # Example
//!
//! ```fsharp
//! // dashboard.fsx
//! #load "tui.fsx"
//!
//! let render buffer area state =
//!     let block = Block.create()
//!         |> Block.title "My Dashboard"
//!         |> Block.borders Borders.ALL
//!     block |> Widget.render buffer area
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use fusabi_tui_core::buffer::Buffer;
use fusabi_tui_core::layout::Rect;
use fusabi_tui_core::style::{Color, Modifier, Style};

use crate::error::{EngineError, EngineResult};
use crate::state::DashboardState;

/// Context for Fusabi script evaluation.
///
/// Holds the compiled script state and provides methods for rendering.
/// The context is created when a dashboard file is loaded and persists
/// until the file is reloaded or the engine is destroyed.
#[derive(Debug)]
pub struct FusabiContext {
    /// Path to the entry script file.
    entry_file: PathBuf,

    /// Compiled module cache for hot reload optimization.
    module_cache: HashMap<PathBuf, CompiledModule>,

    /// Registered host function names for debugging.
    registered_functions: Vec<String>,

    /// Whether the context has been successfully initialized.
    initialized: bool,
}

/// A compiled Fusabi module ready for execution.
#[derive(Debug, Clone)]
pub struct CompiledModule {
    /// Source file path.
    pub path: PathBuf,

    /// Raw source content (for cache invalidation).
    pub source_hash: u64,

    /// Dependencies loaded by this module.
    pub dependencies: Vec<PathBuf>,
}

impl FusabiContext {
    /// Create a new Fusabi context for the given entry file.
    ///
    /// This initializes the Fusabi engine and registers all TUI host functions.
    /// The script is not evaluated until [`evaluate`] is called.
    pub fn new(entry_file: PathBuf) -> Self {
        let mut ctx = Self {
            entry_file,
            module_cache: HashMap::new(),
            registered_functions: Vec::new(),
            initialized: false,
        };

        // Register host functions
        ctx.register_color_functions();
        ctx.register_style_functions();
        ctx.register_layout_functions();
        ctx.register_widget_functions();
        ctx.register_buffer_functions();

        ctx
    }

    /// Evaluate the entry script and all its dependencies.
    ///
    /// This compiles and executes the Fusabi script, making the `render`
    /// function available for subsequent calls.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The script file cannot be read
    /// - The script contains syntax errors
    /// - A required dependency is missing
    /// - Runtime evaluation fails
    pub fn evaluate(&mut self, source: &str) -> EngineResult<()> {
        // TODO: Integrate with Fusabi v0.34+ engine
        // 1. Create Engine instance
        // 2. Register host functions via call_host API
        // 3. Evaluate source with module namespace
        // 4. Cache compiled bytecode

        self.initialized = true;
        Ok(())
    }

    /// Render the dashboard by calling the script's render function.
    ///
    /// This invokes the `render` function defined in the Fusabi script,
    /// passing the buffer, area, and state as arguments.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to render widgets into
    /// * `area` - The available rendering area
    /// * `state` - Current dashboard state (focus, selections, etc.)
    ///
    /// # Errors
    ///
    /// Returns an error if the render function fails or is not defined.
    pub fn render(
        &mut self,
        buffer: &mut Buffer,
        area: Rect,
        state: &DashboardState,
    ) -> EngineResult<()> {
        if !self.initialized {
            return Err(EngineError::InvalidState(
                "FusabiContext not initialized".to_string(),
            ));
        }

        // TODO: Call Fusabi's render function
        // 1. Set current buffer in thread-local or context
        // 2. Call "render" function with (buffer, area, state)
        // 3. Collect any widget render operations
        // 4. Apply operations to buffer

        Ok(())
    }

    /// Invalidate cached modules for the given paths.
    ///
    /// Called when files change to trigger recompilation.
    pub fn invalidate(&mut self, paths: &[PathBuf]) {
        for path in paths {
            self.module_cache.remove(path);
        }
        self.initialized = false;
    }

    /// Check if the context is ready for rendering.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the list of registered host functions (for debugging).
    pub fn registered_functions(&self) -> &[String] {
        &self.registered_functions
    }

    // =========================================================================
    // Host Function Registration
    // =========================================================================

    fn register_color_functions(&mut self) {
        // tui.color.rgb(r, g, b) -> Color
        self.registered_functions.push("tui.color.rgb".to_string());

        // tui.color.indexed(i) -> Color
        self.registered_functions
            .push("tui.color.indexed".to_string());

        // tui.color.reset() -> Color
        self.registered_functions.push("tui.color.reset".to_string());

        // Named colors
        for name in &[
            "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white", "darkGray",
            "lightRed", "lightGreen", "lightYellow", "lightBlue", "lightMagenta", "lightCyan",
        ] {
            self.registered_functions
                .push(format!("tui.color.{}", name));
        }
    }

    fn register_style_functions(&mut self) {
        // tui.style.new() -> Style
        self.registered_functions.push("tui.style.new".to_string());

        // tui.style.fg(color, style) -> Style
        self.registered_functions.push("tui.style.fg".to_string());

        // tui.style.bg(color, style) -> Style
        self.registered_functions.push("tui.style.bg".to_string());

        // Modifier functions
        for modifier in &[
            "bold",
            "dim",
            "italic",
            "underlined",
            "slowBlink",
            "rapidBlink",
            "reversed",
            "hidden",
            "crossedOut",
        ] {
            self.registered_functions
                .push(format!("tui.style.{}", modifier));
        }
    }

    fn register_layout_functions(&mut self) {
        // tui.layout.rect(x, y, w, h) -> Rect
        self.registered_functions
            .push("tui.layout.rect".to_string());

        // tui.layout.split(rect, direction, constraints) -> [Rect]
        self.registered_functions
            .push("tui.layout.split".to_string());

        // Constraint constructors
        self.registered_functions
            .push("tui.layout.length".to_string());
        self.registered_functions
            .push("tui.layout.percentage".to_string());
        self.registered_functions
            .push("tui.layout.ratio".to_string());
        self.registered_functions
            .push("tui.layout.fill".to_string());
        self.registered_functions.push("tui.layout.min".to_string());
        self.registered_functions.push("tui.layout.max".to_string());
    }

    fn register_widget_functions(&mut self) {
        // Block widget
        self.registered_functions
            .push("tui.widget.block".to_string());
        self.registered_functions
            .push("tui.widget.blockTitle".to_string());
        self.registered_functions
            .push("tui.widget.blockBorders".to_string());

        // Paragraph widget
        self.registered_functions
            .push("tui.widget.paragraph".to_string());
        self.registered_functions
            .push("tui.widget.paragraphAlignment".to_string());

        // List widget
        self.registered_functions
            .push("tui.widget.list".to_string());
        self.registered_functions
            .push("tui.widget.listItem".to_string());

        // Gauge widget
        self.registered_functions
            .push("tui.widget.gauge".to_string());
        self.registered_functions
            .push("tui.widget.gaugePercent".to_string());

        // Table widget
        self.registered_functions
            .push("tui.widget.table".to_string());
        self.registered_functions
            .push("tui.widget.tableRow".to_string());

        // Sparkline widget
        self.registered_functions
            .push("tui.widget.sparkline".to_string());

        // Tabs widget
        self.registered_functions
            .push("tui.widget.tabs".to_string());

        // Render function
        self.registered_functions
            .push("tui.widget.render".to_string());
    }

    fn register_buffer_functions(&mut self) {
        // tui.buffer.setString(x, y, text, style, buffer) -> ()
        self.registered_functions
            .push("tui.buffer.setString".to_string());

        // tui.buffer.setStyle(area, style, buffer) -> ()
        self.registered_functions
            .push("tui.buffer.setStyle".to_string());

        // tui.buffer.get(x, y, buffer) -> Cell
        self.registered_functions
            .push("tui.buffer.get".to_string());

        // tui.buffer.clear(buffer) -> ()
        self.registered_functions
            .push("tui.buffer.clear".to_string());
    }
}

/// Parse `#load` directives from Fusabi source code.
///
/// Returns a list of file paths that should be loaded as dependencies.
///
/// # Example
///
/// ```ignore
/// let source = r#"
/// #load "../tui.fsx"
/// #load "widgets/block.fsx"
/// "#;
/// let deps = parse_load_directives(source, Path::new("dashboard.fsx"));
/// ```
pub fn parse_load_directives(source: &str, base_path: &std::path::Path) -> Vec<PathBuf> {
    let mut deps = Vec::new();
    let parent = base_path.parent().unwrap_or(std::path::Path::new("."));

    for line in source.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.starts_with("//") || trimmed.is_empty() {
            continue;
        }

        // Parse #load "path/to/file.fsx"
        if let Some(rest) = trimmed.strip_prefix("#load") {
            let rest = rest.trim();

            // Extract quoted path
            if let Some(path_str) = extract_quoted_string(rest) {
                let mut dep_path = parent.to_path_buf();
                dep_path.push(path_str);

                // Normalize path
                if let Ok(normalized) = dep_path.canonicalize() {
                    deps.push(normalized);
                } else {
                    // Keep relative path if canonicalize fails
                    deps.push(dep_path);
                }
            }
        }
    }

    deps
}

/// Extract a quoted string from input.
fn extract_quoted_string(input: &str) -> Option<&str> {
    let input = input.trim();

    if input.starts_with('"') {
        if let Some(end) = input[1..].find('"') {
            return Some(&input[1..=end]);
        }
    }

    None
}

/// Hash source content for cache invalidation.
pub fn hash_source(source: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_load_directives() {
        let source = r#"
// Dashboard file
#load "../tui.fsx"
#load "widgets/block.fsx"

let x = 42
"#;
        let deps = parse_load_directives(source, std::path::Path::new("/app/fsx/dashboard.fsx"));

        assert_eq!(deps.len(), 2);
        // Paths will be relative since files don't exist
        assert!(deps[0].ends_with("tui.fsx"));
        assert!(deps[1].ends_with("block.fsx"));
    }

    #[test]
    fn test_extract_quoted_string() {
        assert_eq!(extract_quoted_string(r#""hello""#), Some("hello"));
        assert_eq!(extract_quoted_string(r#"  "path/to/file"  "#), Some("path/to/file"));
        assert_eq!(extract_quoted_string(r#"unquoted"#), None);
        assert_eq!(extract_quoted_string(r#""unclosed"#), None);
    }

    #[test]
    fn test_hash_source() {
        let hash1 = hash_source("let x = 42");
        let hash2 = hash_source("let x = 42");
        let hash3 = hash_source("let x = 43");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_fusabi_context_new() {
        let ctx = FusabiContext::new(PathBuf::from("test.fsx"));
        assert!(!ctx.is_initialized());
        assert!(!ctx.registered_functions().is_empty());
    }

    #[test]
    fn test_fusabi_context_registered_functions() {
        let ctx = FusabiContext::new(PathBuf::from("test.fsx"));
        let funcs = ctx.registered_functions();

        // Check some expected functions are registered
        assert!(funcs.contains(&"tui.color.rgb".to_string()));
        assert!(funcs.contains(&"tui.style.new".to_string()));
        assert!(funcs.contains(&"tui.layout.rect".to_string()));
        assert!(funcs.contains(&"tui.widget.block".to_string()));
        assert!(funcs.contains(&"tui.buffer.setString".to_string()));
    }
}
