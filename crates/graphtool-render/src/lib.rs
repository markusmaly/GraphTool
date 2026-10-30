//! Grammar-driven vector chart renderer for GraphTool.
//!
//! Phase 0 skeleton. Renders a `ChartSpec` into a vector drawing model,
//! built on the low-level primitives of crates/render (paint, text, fonts).
//! The DeckCraft chart model and renderer are NOT used (architecture
//! decision, proposal section 3): all charts are vector graphics.

use graphtool_grammar::ChartSpec;
use graphtool_presets::Preset;

/// A resolved drawing primitive. The concrete primitive set follows
/// crates/render; kept opaque in Phase 0.
#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Rect { x: f64, y: f64, w: f64, h: f64, fill: Option<String>, stroke: Option<String> },
    Line { pts: Vec<(f64, f64)>, stroke: String, width: f64 },
    Text { x: f64, y: f64, text: String, size_pt: f64, fill: String },
}

/// The vector drawing model of a rendered chart: deterministic for a given
/// spec and preset (identical inputs produce identical output).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Drawing {
    pub primitives: Vec<Primitive>,
    pub width_pt: f64,
    pub height_pt: f64,
}

/// Render a chart specification. Phase 0 stub: an empty drawing.
pub fn render(_spec: &ChartSpec, _preset: &Preset, width_pt: f64, height_pt: f64) -> Drawing {
    Drawing { primitives: Vec::new(), width_pt, height_pt }
}
