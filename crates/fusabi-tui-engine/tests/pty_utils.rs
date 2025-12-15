//! PTY-based testing utilities for Fusabi TUI applications.
//!
//! This module provides utilities for testing TUI applications that run in
//! pseudo-terminals, including process builders, custom terminal profiles,
//! and performance benchmarking helpers.

use ratatui_testlib::CommandBuilder;
use std::path::PathBuf;

/// Builder for constructing commands to run Fusabi TUI examples.
///
/// # Example
///
/// ```rust,no_run
/// use fusabi_tui_test::FusabiExampleBuilder;
///
/// let cmd = FusabiExampleBuilder::new("basic_app")
///     .with_release()
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct FusabiExampleBuilder {
    example_name: String,
    release: bool,
    features: Vec<String>,
    env_vars: Vec<(String, String)>,
}

impl FusabiExampleBuilder {
    /// Creates a new builder for the specified example.
    pub fn new(example_name: impl Into<String>) -> Self {
        Self {
            example_name: example_name.into(),
            release: false,
            features: Vec::new(),
            env_vars: Vec::new(),
        }
    }

    /// Enables release mode compilation.
    pub fn with_release(mut self) -> Self {
        self.release = true;
        self
    }

    /// Adds a feature flag.
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    /// Adds an environment variable.
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.push((key.into(), value.into()));
        self
    }

    /// Builds the CommandBuilder.
    pub fn build(self) -> CommandBuilder {
        let mut cmd = CommandBuilder::new("cargo");
        cmd.arg("run");

        if self.release {
            cmd.arg("--release");
        }

        if !self.features.is_empty() {
            cmd.arg("--features");
            cmd.arg(self.features.join(","));
        }

        cmd.arg("-p");
        cmd.arg("fusabi-tui-engine");
        cmd.arg("--example");
        cmd.arg(&self.example_name);

        // Set environment variables
        for (key, value) in self.env_vars {
            cmd.env(key, value);
        }

        cmd
    }
}

/// Builder for constructing commands to run custom Fusabi TUI binaries.
#[derive(Debug, Clone)]
pub struct FusabiBinaryBuilder {
    binary_path: PathBuf,
    args: Vec<String>,
    env_vars: Vec<(String, String)>,
}

impl FusabiBinaryBuilder {
    /// Creates a new builder for the specified binary path.
    pub fn new(binary_path: impl Into<PathBuf>) -> Self {
        Self {
            binary_path: binary_path.into(),
            args: Vec::new(),
            env_vars: Vec::new(),
        }
    }

    /// Adds a command-line argument.
    pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Adds an environment variable.
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.push((key.into(), value.into()));
        self
    }

    /// Builds the CommandBuilder.
    pub fn build(self) -> CommandBuilder {
        let mut cmd = CommandBuilder::new(&self.binary_path);

        for arg in self.args {
            cmd.arg(arg);
        }

        for (key, value) in self.env_vars {
            cmd.env(key, value);
        }

        cmd
    }
}

/// Terminal profile presets for testing different terminal capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalPreset {
    /// Standard 80x24 terminal (most compatible)
    Standard,
    /// Large terminal (120x40)
    Large,
    /// Small terminal (40x12) - stress test for layouts
    Small,
    /// Wide terminal (200x24) - test horizontal scrolling
    Wide,
    /// Tall terminal (80x60) - test vertical scrolling
    Tall,
}

impl TerminalPreset {
    /// Returns the (width, height) dimensions for this preset.
    pub fn dimensions(self) -> (u16, u16) {
        match self {
            TerminalPreset::Standard => (80, 24),
            TerminalPreset::Large => (120, 40),
            TerminalPreset::Small => (40, 12),
            TerminalPreset::Wide => (200, 24),
            TerminalPreset::Tall => (80, 60),
        }
    }

    /// Returns a description of this preset.
    pub fn description(self) -> &'static str {
        match self {
            TerminalPreset::Standard => "Standard 80x24 terminal",
            TerminalPreset::Large => "Large 120x40 terminal",
            TerminalPreset::Small => "Small 40x12 terminal (stress test)",
            TerminalPreset::Wide => "Wide 200x24 terminal",
            TerminalPreset::Tall => "Tall 80x60 terminal",
        }
    }
}

/// Helper for collecting performance metrics during tests.
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    /// Number of screen updates observed
    pub update_count: usize,
    /// Total time spent waiting for conditions
    pub total_wait_time_ms: u128,
    /// Peak memory usage (estimated)
    pub peak_memory_bytes: usize,
}

impl PerformanceMetrics {
    /// Creates a new performance metrics tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a screen update.
    pub fn record_update(&mut self) {
        self.update_count += 1;
    }

    /// Records wait time.
    pub fn record_wait(&mut self, duration_ms: u128) {
        self.total_wait_time_ms += duration_ms;
    }

    /// Records memory usage.
    pub fn record_memory(&mut self, bytes: usize) {
        if bytes > self.peak_memory_bytes {
            self.peak_memory_bytes = bytes;
        }
    }

    /// Prints a summary of the metrics.
    pub fn print_summary(&self) {
        println!("=== Performance Metrics ===");
        println!("Screen updates: {}", self.update_count);
        println!("Total wait time: {}ms", self.total_wait_time_ms);
        println!("Peak memory: {} bytes", self.peak_memory_bytes);
    }
}

/// Gets the workspace root directory.
///
/// This is useful for constructing paths to test fixtures or example binaries.
pub fn workspace_root() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");

    PathBuf::from(manifest_dir)
        .parent()
        .expect("Failed to get parent dir")
        .parent()
        .expect("Failed to get workspace root")
        .to_path_buf()
}

/// Gets the path to the examples directory.
pub fn examples_dir() -> PathBuf {
    workspace_root().join("crates/fusabi-tui-engine/examples")
}

/// Gets the path to the target directory.
pub fn target_dir() -> PathBuf {
    workspace_root().join("target")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_builder_basic() {
        let cmd = FusabiExampleBuilder::new("basic_app").build();
        // Just verify it builds without panicking
        assert!(true);
    }

    #[test]
    fn test_example_builder_with_features() {
        let cmd = FusabiExampleBuilder::new("dashboard")
            .with_release()
            .with_feature("hot-reload")
            .build();
        assert!(true);
    }

    #[test]
    fn test_terminal_presets() {
        assert_eq!(TerminalPreset::Standard.dimensions(), (80, 24));
        assert_eq!(TerminalPreset::Large.dimensions(), (120, 40));
        assert_eq!(TerminalPreset::Small.dimensions(), (40, 12));
        assert_eq!(TerminalPreset::Wide.dimensions(), (200, 24));
        assert_eq!(TerminalPreset::Tall.dimensions(), (80, 60));
    }

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();
        metrics.record_update();
        metrics.record_update();
        metrics.record_wait(100);
        metrics.record_memory(1024);

        assert_eq!(metrics.update_count, 2);
        assert_eq!(metrics.total_wait_time_ms, 100);
        assert_eq!(metrics.peak_memory_bytes, 1024);
    }
}
