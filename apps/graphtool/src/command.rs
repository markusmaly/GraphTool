//! Phase 1: the menu structure as stub commands.
//!
//! Every menu item of the confirmed menu structure (canvas, "Menu Structure
//! (visual overview)") is registered here as a named command with its
//! function-registry handler name, its menu path and its workflow
//! prerequisite. Selecting any item performs no action yet: all handlers are
//! empty stubs. Later phases wire the handlers one by one; the menu tree
//! itself is fixed in Phase 1.

use crate::document::Document;

/// Outcome of invoking a command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The command executed and changed the document.
    /// Used from Phase 2 on; Phase 1 stubs return NotImplemented.
    #[allow(dead_code)]
    Ok,
    /// The command is registered but not yet implemented (Phase 1 stub).
    NotImplemented,
    /// The command's prerequisites are not met (e.g. no graph selected).
    Disabled,
}

/// Workflow prerequisite: when a menu item is enabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prereq {
    /// Always available (step-1 and step-2 items).
    Always,
    /// Requires a defined graphing area on the current slide.
    AreaDefined,
    /// Requires a graph to be inserted and selected (workflow steps 4+).
    GraphSelected,
}

/// One menu item / command of the GraphTool menu structure.
pub struct Command {
    /// Stable command id, e.g. "presets.selectActivePreset".
    pub id: &'static str,
    /// Function-registry handler name (registry entries refer back to this).
    pub handler: &'static str,
    /// Menu label as shown in the menu bar.
    pub label: &'static str,
    /// Top-level menu the item belongs to (1-7 of the core workflow).
    pub menu: &'static str,
    pub prereq: Prereq,
    /// Empty stub in Phase 1; filled in by later phases.
    pub run: fn(&mut Document) -> Outcome,
}

fn stub(_: &mut Document) -> Outcome {
    Outcome::NotImplemented
}

/// The seven top-level menus, in core-workflow order.
pub const MENUS: [&str; 7] = [
    "Presets", "Graph Area", "Insert Graph", "Data",
    "Tidy Data", "Format Graph", "Render",
];

/// The complete command registry: one entry per menu item.
pub fn registry() -> Vec<Command> {
    let c = |id, handler, label, menu, prereq| Command {
        id, handler, label, menu, prereq, run: stub,
    };
    use Prereq::*;
    vec![
        // 1. Presets
        c("presets.selectActivePreset", "selectActivePreset()", "Select active preset", "Presets", Always),
        c("presets.openPresetManager", "openPresetManager()", "Preset manager", "Presets", Always),
        c("presets.openPaletteEditor", "openPaletteEditor()", "Colour palettes", "Presets", Always),
        c("presets.openThemeEditor", "openThemeEditor()", "Theme editor", "Presets", Always),
        c("presets.setPresetMasterStorage", "setPresetMasterStorage()", "Store in slide master / deck design", "Presets", Always),
        c("presets.importPreset", "importPreset()", "Import preset...", "Presets", Always),
        c("presets.exportPreset", "exportPreset()", "Export preset...", "Presets", Always),
        c("presets.restoreFactoryPresets", "restoreFactoryPresets()", "Restore factory presets", "Presets", Always),
        // 2. Graph Area
        c("area.defineAutomatic", "defineAreaAutomatic()", "Define full-slide area: automatic", "Graph Area", Always),
        c("area.defineDrawRect", "defineAreaDrawRect()", "Define full-slide area: draw rectangle", "Graph Area", Always),
        c("area.defineCoordinates", "defineAreaCoordinates()", "Define full-slide area: enter coordinates", "Graph Area", Always),
        c("area.applyPlacementPreset", "applyPlacementPreset()", "Place graph: placement presets", "Graph Area", AreaDefined),
        c("area.placeDrawRect", "placeGraphDrawRect()", "Place graph: draw rectangle", "Graph Area", AreaDefined),
        // 3. Insert Graph
        c("graph.insertChart", "insertChart(type)", "Chart-type gallery", "Insert Graph", AreaDefined),
        // 4. Data
        c("data.loadData", "loadData(source)", "Load data", "Data", GraphSelected),
        c("data.enterOrPasteData", "enterOrPasteData()", "Enter data manually / paste data", "Data", GraphSelected),
        c("data.editDataGrid", "editDataGrid()", "Data grid (edit in place)", "Data", GraphSelected),
        c("data.refreshDataLink", "refreshDataLink()", "Data-link refresh", "Data", GraphSelected),
        // 5. Tidy Data - catalogue TO BE DEFINED LATER; menu present as stub
        c("tidy.open", "tidyData()", "Tidy data operations (to be defined)", "Tidy Data", GraphSelected),
        // 6. Format Graph
        c("format.formatGraph", "formatGraph(spec)", "Aesthetics / layers / scales / facets", "Format Graph", GraphSelected),
        c("format.setChartTitles", "setChartTitles()", "Titles", "Format Graph", GraphSelected),
        c("format.applyThemeOverride", "applyThemeOverride()", "Theme overrides", "Format Graph", GraphSelected),
        // 7. Render
        c("render.renderChart", "renderChart()", "Render to slide", "Render", GraphSelected),
        c("render.exportChart", "exportChart(fmt)", "Export chart as SVG / PNG", "Render", GraphSelected),
    ]
}

/// Whether a command may be invoked given the current document state.
pub fn is_enabled(cmd: &Command, doc: &Document) -> bool {
    match cmd.prereq {
        Prereq::Always => true,
        Prereq::AreaDefined => doc.slide_count > 0,
        Prereq::GraphSelected => doc.selected().is_some(),
    }
}

/// Invoke a command, respecting its prerequisite.
pub fn invoke(cmd: &Command, doc: &mut Document) -> Outcome {
    if !is_enabled(cmd, doc) {
        Outcome::Disabled
    } else {
        (cmd.run)(doc)
    }
}

/// The menu tree as text, matching the canvas "Menu Structure (visual overview)".
pub fn menu_tree() -> String {
    let reg = registry();
    let mut s = String::from("GraphTool menu structure (Phase 1 stub)\n");
    for (i, menu) in MENUS.iter().enumerate() {
        let note = if *menu == "Tidy Data" { "  (TO BE DEFINED LATER)" } else { "" };
        s.push_str(&format!("{:>2}. {}{}\n", i + 1, menu, note));
        for cmd in reg.iter().filter(|c| c.menu == *menu) {
            s.push_str(&format!("      - {:<42} [{}]  {}\n",
                cmd.label, cmd.id, cmd.handler));
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_covers_all_seven_menus() {
        let reg = registry();
        assert_eq!(reg.len(), 24);
        for menu in MENUS {
            assert!(reg.iter().any(|c| c.menu == menu),
                "no command registered for menu {menu}");
        }
    }

    #[test]
    fn command_ids_are_unique() {
        let reg = registry();
        let mut ids: Vec<&str> = reg.iter().map(|c| c.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), reg.len());
    }

    #[test]
    fn empty_document_disables_workflow_steps() {
        let reg = registry();
        let mut doc = Document::default();
        // Steps requiring a graph are disabled; Presets remain available.
        for cmd in reg.iter().filter(|c| c.prereq == Prereq::Always) {
            assert_eq!(invoke(cmd, &mut doc), Outcome::NotImplemented);
        }
        for cmd in reg.iter().filter(|c| c.prereq == Prereq::GraphSelected) {
            assert_eq!(invoke(cmd, &mut doc), Outcome::Disabled);
        }
    }

    #[test]
    fn document_with_graph_enables_all_stubs() {
        let reg = registry();
        let mut doc = Document {
            charts: vec![graphtool_grammar::ChartSpec::default()],
            slide_count: 1,
            undo: Vec::new(),
        };
        for cmd in &reg {
            assert_eq!(invoke(cmd, &mut doc), Outcome::NotImplemented,
                "unexpected outcome for {}", cmd.id);
        }
    }
}
