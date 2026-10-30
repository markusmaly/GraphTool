//! Phase 1: the GraphTool egui shell.
//!
//! A native window based on egui/eframe 0.37 (crates.io release), showing
//! the complete seven-menu structure. Every item is a stub command from
//! `command::registry()`, enabled according to its workflow prerequisite.
//! Selecting any item performs no action yet - the click is recorded in
//! the session log.
//!
//! Note on the eframe 0.37 `App` trait: the required method is
//! `App::ui(&mut self, ui: &mut egui::Ui, frame)`, optionally preceded by
//! `App::logic(&mut self, ctx, frame)` for non-UI work; the older
//! `App::update(&mut self, ctx, frame)` no longer exists. Panels are
//! shown into a `&mut Ui` (`CentralPanel::show(ui, ..)`), and
//! `TopBottomPanel` has been removed in favour of the `MenuBar` widget,
//! which is placed directly at the top of the root `Ui`.

use egui::{CentralPanel, MenuBar, Ui};

use crate::command::{self, Command, Outcome, Prereq};
use crate::document::Document;

/// The GraphTool application shell: document state plus the session log.
pub struct Shell {
    doc: Document,
    reg: Vec<Command>,
    /// (menu item label and id, outcome text) of this session, most recent last.
    log: Vec<(String, &'static str)>,
}

/// A snapshot of one menu item for rendering: avoids holding borrows of
/// `self` across the egui closure boundaries.
struct ItemSnapshot {
    id: &'static str,
    label: &'static str,
    prereq: Prereq,
    enabled: bool,
}

impl Shell {
    pub fn new() -> Self {
        Self { doc: Document::default(), reg: command::registry(), log: Vec::new() }
    }

    /// Snapshot the items of one menu with their current enabled state.
    fn items(&self, menu: &str) -> Vec<ItemSnapshot> {
        self.reg
            .iter()
            .filter(|c| c.menu == menu)
            .map(|c| ItemSnapshot {
                id: c.id,
                label: c.label,
                prereq: c.prereq,
                enabled: command::is_enabled(c, &self.doc),
            })
            .collect()
    }

    fn menu_bar(&mut self, ui: &mut Ui) {
        MenuBar::new().ui(ui, |ui| {
            ui.label(egui::RichText::new("GraphTool").strong());
            ui.separator();
            for menu in command::MENUS {
                egui::menu::MenuButton::new(menu).ui(ui, |ui| {
                    for item in self.items(menu) {
                        let response = ui
                            .add_enabled(item.enabled, egui::Button::new(item.label))
                            .on_disabled_hover_text(disabled_reason(item.prereq));
                        if response.clicked() {
                            self.record(item.id);
                        }
                    }
                    if menu == "Tidy Data" {
                        ui.separator();
                        ui.label(egui::RichText::new("TO BE DEFINED LATER").weak().small());
                    }
                });
            }
        });
    }

    /// Invoke a stub command by id and record the outcome in the session log.
    /// `self.reg` is read (for the command) while `self.doc` is mutated -
    /// disjoint fields, so the borrow checker permits both.
    fn record(&mut self, id: &str) {
        let Some(cmd) = self.reg.iter().find(|c| c.id == id) else { return };
        let outcome = command::invoke(cmd, &mut self.doc);
        let text = match outcome {
            Outcome::Ok => "ok (changed document)",
            Outcome::NotImplemented => "registered, not yet implemented (Phase 1 stub)",
            Outcome::Disabled => "disabled: prerequisites not met",
        };
        self.log.push((format!("{} [{}]", cmd.label, cmd.id), text));
    }

    fn central(&mut self, ui: &mut Ui) {
        CentralPanel::default().show(ui, |ui| {
            ui.heading("GraphTool - Phase 1: menu structure as stub commands");
            ui.add_space(6.0);
            ui.label("All seven menus and their items are registered; no item has an action yet.");
            ui.label(format!(
                "Commands: {} across {} menus. Handlers are wired in Phase 2+.",
                self.reg.len(),
                command::MENUS.len()
            ));
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(4.0);
            ui.label("Session log");
            egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                if self.log.is_empty() {
                    ui.label(egui::RichText::new(
                        "(select a menu item to record a stub invocation)",
                    ).weak());
                }
                for (label, outcome) in &self.log {
                    ui.label(format!("{label}: {outcome}"));
                }
            });
        });
    }
}

fn disabled_reason(prereq: Prereq) -> &'static str {
    match prereq {
        Prereq::Always => "",
        Prereq::AreaDefined => "disabled until the slide has a defined graphing area",
        Prereq::GraphSelected => "disabled until a graph is inserted and selected",
    }
}

impl eframe::App for Shell {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menu_bar(ui);
        self.central(ui);
    }
}
