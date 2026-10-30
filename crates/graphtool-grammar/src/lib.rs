//! Grammar-of-graphics chart specification model for GraphTool.
//!
//! Phase 0 skeleton implementing the confirmed ChartSpec sketch
//! (architecture proposal, section 4). This is the single source of truth:
//! every chart is reproducible from its stored specification, and every
//! edit (wizard, menu, script) is an edit to a `ChartSpec`.
//!
//! NOTE: serde derives were temporarily removed for the Phase 0 sandbox
//! build (no C toolchain available there); they are restored as of this
//! revision, per the confirmed architecture (serialisable, versioned
//! ChartSpec embedded in the PPTX as a custom XML part).

use serde::{Deserialize, Serialize};


/// The stored specification of one chart.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct ChartSpec {
    /// Specification format version; serialised with every stored chart so
    /// that future format revisions can be detected and migrated.
    pub version: u32,
    pub data: DataRef,
    pub tidy: Vec<graphtool_tidy::TidyOp>,
    pub preset: PresetRef,
    pub layers: Vec<LayerSpec>,
    pub scales: Vec<ScaleSpec>,
    pub coord: Coord,
    pub facets: Option<FacetSpec>,
    pub labels: Labels,
    pub annotations: Vec<Annotation>,
}

/// Reference to the data source of a chart.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct DataRef {
    /// Kind of source: bundled sample dataset, imported file, clipboard, link.
    pub kind: DataKind,
    /// Source locator (dataset id, file path, link target).
    pub locator: Option<String>,
    /// Whether an external link is refreshed automatically.
    pub linked: bool,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq, Eq)]
pub enum DataKind {
    #[default]
    Sample,
    File,
    Clipboard,
    Manual,
}

/// Reference to the active style preset (palettes, theme, base font).
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct PresetRef {
    pub id: Option<String>,
    /// None: take the design-derived default (slide master settings).
    pub master_stored: Option<bool>,
}

/// One layer of the chart, in drawing order.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct LayerSpec {
    pub geom: Geom,
    pub stat: Stat,
    pub position: Position,
    /// Data override: a layer may use a different tidy recipe result.
    pub data: Option<DataRef>,
    /// aes mappings: column name (or None) per aesthetic.
    pub aes: Aes,
    /// Layer parameters (line width, point size, alpha, ...): TO BE DEFINED LATER.
    pub params: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq, Eq)]
pub enum Geom {
    #[default]
    Point,
    Line,
    Bar,
    Area,
    Smooth,
    Text,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq, Eq)]
pub enum Stat {
    #[default]
    Identity,
    Count,
    Smooth,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq, Eq)]
pub enum Position {
    #[default]
    Identity,
    Stack,
    Dodge,
    Fill,
}

/// Aesthetic mappings: column names per aesthetic.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Aes {
    pub x: Option<String>,
    pub y: Option<String>,
    pub colour: Option<String>,
    pub fill: Option<String>,
    pub size: Option<String>,
    pub linetype: Option<String>,
    pub alpha: Option<String>,
    pub group: Option<String>,
}

/// A scale specification (transform, palette, number format): TO BE DEFINED LATER.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ScaleSpec {
    pub aes: String,
    pub transform: Option<String>,
    pub palette: Option<String>,
    pub format: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq, Eq)]
pub enum Coord {
    #[default]
    Cartesian,
    Flip,
    Polar,
    Equal,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct FacetSpec {
    pub kind: FacetKind,
    /// Faceting variable(s).
    pub vars: Vec<String>,
    /// Whether scales are free per panel.
    pub free_scales: bool,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
pub enum FacetKind {
    Wrap,
    Grid,
}

/// Titles and captions.
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Labels {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub caption: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
}

/// A text or line annotation placed in data or plot coordinates: TO BE DEFINED LATER.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Annotation {
    pub kind: String,
    pub params: Option<String>,
}
