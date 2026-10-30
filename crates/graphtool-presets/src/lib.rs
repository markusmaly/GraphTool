//! Style presets for GraphTool: colour palettes, ggplot2 themes, base fonts,
//! import/export and factory presets.
//!
//! Phase 0 skeleton: the data model and factory theme list. The full theme
//! editor and import/export are later phases (phase group A).
//!
//! NOTE: serde derives were temporarily removed for the Phase 0 sandbox
//! build (no C toolchain available there); they are restored as of this
//! revision, per the confirmed architecture (serialisable, versioned
//! ChartSpec embedded in the PPTX as a custom XML part).

use serde::{Deserialize, Serialize};


/// A style preset: palette plus complete theme, restyling a chart entirely.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub palette: Palette,
    pub theme: ThemeRef,
    /// Base font: family and base size, with derived relative sizes
    /// (ggplot2 base_size/base_family manner). None: slide-master design.
    pub base_font: Option<BaseFont>,
    /// Whether this preset is stored in the slide master / deck design.
    pub master_stored: bool,
}

/// A colour palette.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Palette {
    /// ColorBrewer name, corporate name, or "custom".
    pub id: String,
    /// Ordered series of colours (hex or named).
    pub colors: Vec<String>,
    pub kind: PaletteKind,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq, Eq)]
pub enum PaletteKind {
    #[default]
    Qualitative,
    Sequential,
    Diverging,
}

/// Reference to a ggplot2-style theme.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeRef {
    /// Built-in: theme_grey (default), theme_bw, theme_minimal, ...
    /// Extension: hrbrthemes, ggthemes, cowplot, ggpubr; or a custom id.
    pub id: String,
    /// Element-level overrides on top of the named theme: TO BE DEFINED LATER.
    pub overrides: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct BaseFont {
    pub family: String,
    /// Base font size in points; derived sizes are relative to this.
    pub size_pt: f64,
}

/// Factory theme list, mirroring ggplot2's built-in themes.
pub const FACTORY_THEMES: [&str; 8] = [
    "theme_grey", "theme_bw", "theme_minimal", "theme_classic",
    "theme_light", "theme_dark", "theme_void", "theme_linedraw",
];

/// Restore a factory preset to its default state. Phase 0 stub.
pub fn restore_factory(_id: &str) -> Option<Preset> {
    None
}
