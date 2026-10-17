//! gt-app library: GUI application built on egui.

pub mod app;
pub mod chart_render;
pub mod item;

#[cfg(target_arch = "wasm32")]
pub mod web;

pub use app::{canvas_page, GraphToolApp, RibbonTab};

use eframe::egui;

impl eframe::App for GraphToolApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("ribbon").show(ui, |ui| {
            self.ribbon(ui);
        });
        egui::Panel::bottom("status").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(self.status.clone());
            });
        });
        egui::Panel::right("side").show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.show_properties {
                    properties_panel(ui, self);
                }
                if self.show_dsl_editor {
                    dsl_editor(ui, self);
                }
            });
        });
        egui::CentralPanel::default().show(ui, |ui| {
            app::canvas_page(ui, self);
        });
    }
}

fn properties_panel(ui: &mut egui::Ui, app: &mut GraphToolApp) {
    egui::CollapsingHeader::new("Properties")
        .default_open(true)
        .show(ui, |ui| {
            let Some(item) = app.selected_chart_mut() else {
                ui.weak("No chart selected.");
                return;
            };
            egui::Grid::new("props").num_columns(2).show(ui, |ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut item.name);
                ui.end_row();
                ui.label("Position");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut item.x).speed(1.0).prefix("x "));
                    ui.add(egui::DragValue::new(&mut item.y).speed(1.0).prefix("y "));
                });
                ui.end_row();
                ui.label("Size");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut item.w).speed(1.0).range(120.0..=2000.0).prefix("w "));
                    ui.add(egui::DragValue::new(&mut item.h).speed(1.0).range(100.0..=1500.0).prefix("h "));
                });
                ui.end_row();
            });
        });
}

fn dsl_editor(ui: &mut egui::Ui, app: &mut GraphToolApp) {
    egui::CollapsingHeader::new("Gantt DSL")
        .default_open(true)
        .show(ui, |ui| {
            let Some(item) = app.selected_chart_mut() else {
                ui.weak("No chart selected.");
                return;
            };
            ui.label("Edit the PlantUML-like DSL; the chart re-renders live.");
            let mut buf = item.dsl.clone();
            let edit = egui::TextEdit::multiline(&mut buf)
                .desired_rows(12)
                .code_editor();
            if ui.add(edit).changed() {
                item.dsl = buf;
            }
        });
}
