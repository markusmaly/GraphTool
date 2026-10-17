//! Headless UI smoke test: runs the full app UI through an egui context
//! without a window.

use egui::{Context, RawInput};

#[test]
fn app_ui_renders_without_panic() {
    let ctx = Context::default();
    for _ in 0..3 {
        let mut out = ctx.run_ui(RawInput::default(), |ui| {
            let mut app = gt_app::GraphToolApp::new();
            app.add_chart(gt_app::item::DEFAULT_DSL.to_string(), "Chart 1".into());
            app.add_chart(gt_app::item::DEFAULT_DSL.to_string(), "Chart 2".into());
            app.selected = Some(app.items[0].id);

            egui::Panel::top("ribbon").show(ui, |ui| {
                app.ribbon(ui);
            });
            egui::Panel::bottom("status").show(ui, |ui| {
                ui.label(app.status.clone());
            });
            egui::Panel::right("side").show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Some(item) = app.selected_chart_mut() {
                        ui.text_edit_singleline(&mut item.name);
                        let mut buf = item.dsl.clone();
                        let edit = egui::TextEdit::multiline(&mut buf).desired_rows(6).code_editor();
                        if ui.add(edit).changed() {
                            item.dsl = buf;
                        }
                    }
                });
            });
            egui::CentralPanel::default().show(ui, |ui| {
                gt_app::canvas_page(ui, &mut app);
            });
        });
        out.textures_delta.clear();
    }
}
