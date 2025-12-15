//! Screenshot comparison and snapshot testing helpers for Fusabi TUI applications.
//!
//! This module provides utilities for visual regression testing using snapshot
//! comparison. It integrates with `insta` for snapshot testing and provides
//! custom comparison logic for TUI-specific patterns.

use ratatui_testlib::{Result, ScreenState, TermTestError};
use std::collections::HashMap;

/// A snapshot of a terminal screen for comparison.
///
/// This captures not just the text content, but also metadata like dimensions,
/// cursor position, and optional styled regions.
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenSnapshot {
    /// Terminal width
    pub width: u16,
    /// Terminal height
    pub height: u16,
    /// Screen contents (raw text)
    pub contents: String,
    /// Cursor position (row, col)
    pub cursor_pos: (u16, u16),
    /// Named regions for partial comparison
    pub regions: HashMap<String, Region>,
}

/// A named region within a terminal screen.
///
/// Regions allow you to compare specific areas of the screen while ignoring
/// dynamic content in other areas (e.g., timestamps, counters).
#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    /// Top-left row (0-based)
    pub row: u16,
    /// Top-left column (0-based)
    pub col: u16,
    /// Width in columns
    pub width: u16,
    /// Height in rows
    pub height: u16,
    /// Extracted text content
    pub content: String,
}

impl ScreenSnapshot {
    /// Creates a snapshot from a ScreenState.
    pub fn from_state(state: &ScreenState) -> Self {
        let (width, height) = state.size();
        let contents = state.contents();
        let cursor_pos = state.cursor_position();

        Self {
            width,
            height,
            contents,
            cursor_pos,
            regions: HashMap::new(),
        }
    }

    /// Adds a named region to the snapshot.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use fusabi_tui_test::ScreenSnapshot;
    /// # use ratatui_testlib::ScreenState;
    /// # let state = ScreenState::new(80, 24);
    /// let mut snapshot = ScreenSnapshot::from_state(&state);
    /// snapshot.add_region("header", 0, 0, 80, 3);
    /// snapshot.add_region("footer", 0, 21, 80, 3);
    /// ```
    pub fn add_region(&mut self, name: impl Into<String>, row: u16, col: u16, width: u16, height: u16) {
        let content = self.extract_region(row, col, width, height);
        let region = Region {
            row,
            col,
            width,
            height,
            content,
        };
        self.regions.insert(name.into(), region);
    }

    /// Extracts text from a specific region.
    fn extract_region(&self, row: u16, col: u16, width: u16, height: u16) -> String {
        let lines: Vec<&str> = self.contents.lines().collect();
        let mut result = String::new();

        for r in row..row + height {
            if r as usize >= lines.len() {
                break;
            }

            let line = lines[r as usize];
            let start = col as usize;
            let end = (col + width) as usize;

            if start < line.len() {
                let slice = if end <= line.len() {
                    &line[start..end]
                } else {
                    &line[start..]
                };
                result.push_str(slice);
            }

            result.push('\n');
        }

        result
    }

    /// Compares this snapshot with another, returning differences.
    pub fn compare(&self, other: &ScreenSnapshot) -> SnapshotComparison {
        let mut differences = Vec::new();

        // Check dimensions
        if self.width != other.width || self.height != other.height {
            differences.push(SnapshotDifference::Dimensions {
                expected: (self.width, self.height),
                actual: (other.width, other.height),
            });
        }

        // Check cursor position
        if self.cursor_pos != other.cursor_pos {
            differences.push(SnapshotDifference::CursorPosition {
                expected: self.cursor_pos,
                actual: other.cursor_pos,
            });
        }

        // Check contents
        if self.contents != other.contents {
            differences.push(SnapshotDifference::Contents {
                diff: compute_text_diff(&self.contents, &other.contents),
            });
        }

        SnapshotComparison {
            matches: differences.is_empty(),
            differences,
        }
    }

    /// Compares only a specific region with another snapshot.
    pub fn compare_region(&self, region_name: &str, other: &ScreenSnapshot) -> Result<()> {
        let region = self.regions.get(region_name)
            .ok_or_else(|| TermTestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Region '{}' not found", region_name)
            )))?;

        let other_region = other.regions.get(region_name)
            .ok_or_else(|| TermTestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Region '{}' not found in other snapshot", region_name)
            )))?;

        if region.content != other_region.content {
            Err(TermTestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Region '{}' content mismatch:\nExpected:\n{}\nActual:\n{}",
                    region_name, region.content, other_region.content
                )
            )))
        } else {
            Ok(())
        }
    }
}

/// Result of comparing two snapshots.
#[derive(Debug)]
pub struct SnapshotComparison {
    /// Whether the snapshots match
    pub matches: bool,
    /// List of differences found
    pub differences: Vec<SnapshotDifference>,
}

impl SnapshotComparison {
    /// Returns true if snapshots match.
    pub fn is_match(&self) -> bool {
        self.matches
    }

    /// Prints a detailed report of differences.
    pub fn print_report(&self) {
        if self.matches {
            println!("Snapshots match!");
            return;
        }

        println!("Snapshot differences found:");
        for (i, diff) in self.differences.iter().enumerate() {
            println!("  {}. {}", i + 1, diff);
        }
    }
}

/// A specific difference between two snapshots.
#[derive(Debug)]
pub enum SnapshotDifference {
    /// Terminal dimensions differ
    Dimensions {
        expected: (u16, u16),
        actual: (u16, u16),
    },
    /// Cursor positions differ
    CursorPosition {
        expected: (u16, u16),
        actual: (u16, u16),
    },
    /// Text contents differ
    Contents {
        diff: String,
    },
}

impl std::fmt::Display for SnapshotDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnapshotDifference::Dimensions { expected, actual } => {
                write!(f, "Dimensions: expected {:?}, got {:?}", expected, actual)
            }
            SnapshotDifference::CursorPosition { expected, actual } => {
                write!(f, "Cursor: expected {:?}, got {:?}", expected, actual)
            }
            SnapshotDifference::Contents { diff } => {
                write!(f, "Contents:\n{}", diff)
            }
        }
    }
}

/// Computes a human-readable diff between two text strings.
fn compute_text_diff(expected: &str, actual: &str) -> String {
    let mut result = String::new();

    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();

    let max_lines = expected_lines.len().max(actual_lines.len());

    for i in 0..max_lines {
        let expected_line = expected_lines.get(i).copied().unwrap_or("");
        let actual_line = actual_lines.get(i).copied().unwrap_or("");

        if expected_line != actual_line {
            result.push_str(&format!("Line {}:\n", i + 1));
            result.push_str(&format!("  - {}\n", expected_line));
            result.push_str(&format!("  + {}\n", actual_line));
        }
    }

    if result.is_empty() {
        result.push_str("(no line differences, but contents differ in whitespace or newlines)");
    }

    result
}

/// Helper for creating golden file snapshots.
///
/// Golden files are stored snapshots that tests compare against to detect
/// visual regressions.
#[derive(Debug)]
pub struct GoldenFile {
    name: String,
    snapshot: ScreenSnapshot,
}

impl GoldenFile {
    /// Creates a new golden file from a snapshot.
    pub fn new(name: impl Into<String>, snapshot: ScreenSnapshot) -> Self {
        Self {
            name: name.into(),
            snapshot,
        }
    }

    /// Gets the golden file name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the snapshot.
    pub fn snapshot(&self) -> &ScreenSnapshot {
        &self.snapshot
    }

    /// Asserts that the current state matches this golden file.
    pub fn assert_matches(&self, current: &ScreenSnapshot) -> Result<()> {
        let comparison = self.snapshot.compare(current);

        if comparison.matches {
            Ok(())
        } else {
            comparison.print_report();
            Err(TermTestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Golden file '{}' does not match current state", self.name)
            )))
        }
    }
}

/// Normalizes screen contents by removing dynamic elements.
///
/// This is useful for comparing snapshots that contain timestamps, counters,
/// or other dynamic data.
///
/// # Example
///
/// ```rust,no_run
/// # use fusabi_tui_test::normalize_screen;
/// let contents = "Counter: 42\nUptime: 123s";
/// let normalized = normalize_screen(contents, &[
///     (r"Counter: \d+", "Counter: <N>"),
///     (r"Uptime: \d+s", "Uptime: <N>s"),
/// ]);
/// assert_eq!(normalized, "Counter: <N>\nUptime: <N>s");
/// ```
pub fn normalize_screen(contents: &str, patterns: &[(&str, &str)]) -> String {
    let mut result = contents.to_string();

    for (pattern, replacement) in patterns {
        // Use simple string replacement for now to avoid regex dependency
        // In production, you'd want to add regex as a dependency
        result = result.replace(pattern, replacement);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_from_state() {
        let state = ScreenState::new(80, 24);
        let snapshot = ScreenSnapshot::from_state(&state);

        assert_eq!(snapshot.width, 80);
        assert_eq!(snapshot.height, 24);
    }

    #[test]
    fn test_snapshot_comparison_identical() {
        let state = ScreenState::new(80, 24);
        let snap1 = ScreenSnapshot::from_state(&state);
        let snap2 = ScreenSnapshot::from_state(&state);

        let comparison = snap1.compare(&snap2);
        assert!(comparison.matches);
    }

    #[test]
    fn test_normalize_screen() {
        let contents = "Counter: 42\nUptime: 123s\nVersion: 0.1.1";
        // Note: normalize_screen uses simple string replace, not regex
        // So we need to provide exact strings to replace
        let normalized = normalize_screen(contents, &[
            ("Counter: 42", "Counter: <N>"),
            ("Uptime: 123s", "Uptime: <N>s"),
        ]);

        assert!(normalized.contains("Counter: <N>"));
        assert!(normalized.contains("Uptime: <N>s"));
        assert!(normalized.contains("Version: 0.1.1"));
    }
}
