//! Terminal UI layer built on atomic design principles.
//!
//! Atoms are the smallest styled primitives, composites combine atoms into
//! reusable widgets, and views assemble composites into full command outputs.

/// Smallest styled display primitives (badge, icon, label, etc.).
pub mod atoms;
/// Named RGB color constants for the brand palette.
pub mod colors;
/// Mid-level widgets composed from atoms.
pub mod composites;
/// Row and column layout containers with terminal-aware overflow.
pub mod layout;
/// Markdown-to-styled-terminal renderer.
pub mod markdown;
/// Semantic style tokens resolved from the color palette.
pub mod theme;
/// Terminal measurement and ANSI-stripping helpers.
pub mod utils;
/// Top-level output renderers, one per CLI command group.
pub mod views;

/// Enable ANSI color output only when stdout is a TTY with color support.
pub fn init() {
  yansi::whenever(yansi::Condition::TTY_AND_COLOR);
}
