//! GraphTool document model, per the confirmed Phase 0 decision:
//! one spec per chart; document = slides + chart specs + data refs;
//! undo = stack of previous spec states per command.

use graphtool_grammar::ChartSpec;

/// The GraphTool document: what a session edits, and what is saved to a deck.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    /// Chart specifications, in slide order. One spec per chart.
    pub charts: Vec<ChartSpec>,
    /// Slide count (Phase 1 stub: slides are addressed by index).
    pub slide_count: usize,
    /// Undo: stack of previous chart-spec states, most recent last.
    pub undo: Vec<ChartSpec>,
}

impl Document {
    /// The graph selection: which chart subsequent commands act on.
    /// None: no graph selected (later workflow steps are disabled).
    pub fn selected(&self) -> Option<&ChartSpec> {
        self.charts.first()
    }

    /// Replace a chart spec, pushing the previous state onto the undo stack.
    /// Used from Phase 2 on; Phase 1 registers stubs only.
    #[allow(dead_code)]
    pub fn set_chart(&mut self, index: usize, spec: ChartSpec) -> bool {
        if let Some(old) = self.charts.get_mut(index) {
            self.undo.push(old.clone());
            *old = spec;
            true
        } else {
            false
        }
    }

    /// Undo the most recent chart-spec change (Phase 1: first chart only).
    #[allow(dead_code)]
    pub fn undo(&mut self) -> bool {
        match self.undo.pop() {
            Some(prev) => {
                if let Some(first) = self.charts.first_mut() {
                    *first = prev;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}
