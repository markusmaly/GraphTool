//! Main application state and ribbon (Home / Design / Tidying / Visualising).

use crate::item::{ChartItem, ItemId, DEFAULT_DSL};
use eframe::egui;
use egui::{Button, RichText, Vec2};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RibbonTab {
    Home,
    Design,
    Tidying,
    Visualising,
}

impl RibbonTab {
    pub fn label(&self) -> &'static str {
        match self {
            RibbonTab::Home => "Home",
            RibbonTab::Design => "Design",
            RibbonTab::Tidying => "Tidying",
            RibbonTab::Visualising => "Visualising",
        }
    }
}

pub struct GraphToolApp {
    pub tab: RibbonTab,
    pub items: Vec<ChartItem>,
    pub next_id: u64,
    pub selected: Option<ItemId>,
    pub zoom: f32,
    pub show_dsl_editor: bool,
    pub show_properties: bool,
    pub dirty_path: Option<PathBuf>,
    pub status: String,
    pub drag: Option<(ItemId, Vec2, Vec2, bool)>, // (id, start pos, item origin, resizing)
}

impl Default for GraphToolApp {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphToolApp {
    pub fn new() -> Self {
        Self {
            tab: RibbonTab::Home,
            items: Vec::new(),
            next_id: 1,
            selected: None,
            zoom: 1.0,
            show_dsl_editor: true,
            show_properties: true,
            dirty_path: None,
            status: "Ready".into(),
            drag: None,
        }
    }

    pub fn add_chart(&mut self, dsl: String, name: String) {
        let mut item = ChartItem::new(ItemId(self.next_id), name, dsl);
        self.next_id += 1;
        let n = self.items.len();
        item.x = 40.0 + (n as f32 % 3.0) * 40.0;
        item.y = 40.0 + (n as f32 % 3.0) * 30.0;
        self.selected = Some(item.id);
        self.status = format!("Added chart `{}`", item.name);
        self.items.push(item);
    }

    pub fn delete_selected(&mut self) {
        if let Some(id) = self.selected {
            self.items.retain(|i| i.id != id);
            self.selected = None;
            self.status = "Deleted chart".into();
        }
    }

    pub fn save_project(&mut self) {
        let path = self.dirty_path.clone().unwrap_or_else(|| PathBuf::from("project.gtp"));
        let json = match serde_json::to_string_pretty(&self.items) {
            Ok(j) => j,
            Err(e) => {
                self.status = format!("Save failed: {e}");
                return;
            }
        };
        match std::fs::write(&path, json) {
            Ok(_) => {
                self.dirty_path = Some(path.clone());
                self.status = format!("Saved {}", path.display());
            }
            Err(e) => self.status = format!("Save failed: {e}"),
        }
    }

    pub fn load_project(&mut self, path: PathBuf) {
        match std::fs::read_to_string(&path) {
            Ok(text) => match serde_json::from_str::<Vec<ChartItem>>(&text) {
                Ok(items) => {
                    self.items = items;
                    self.dirty_path = Some(path.clone());
                    self.next_id = self.items.iter().map(|i| i.id.0).max().unwrap_or(0) + 1;
                    self.selected = None;
                    self.status = format!("Loaded {}", path.display());
                }
                Err(e) => self.status = format!("Load failed: {e}"),
            },
            Err(e) => self.status = format!("Load failed: {e}"),
        }
    }

    pub fn selected_chart(&self) -> Option<&ChartItem> {
        self.items.iter().find(|i| Some(i.id) == self.selected)
    }

    pub fn selected_chart_mut(&mut self) -> Option<&mut ChartItem> {
        self.items.iter_mut().find(|i| Some(i.id) == self.selected)
    }

    /// The ribbon UI (contextual per tab).
    pub fn ribbon(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for t in [RibbonTab::Home, RibbonTab::Design, RibbonTab::Tidying, RibbonTab::Visualising] {
                let sel = self.tab == t;
                let btn = Button::new(RichText::new(t.label()).strong().size(14.0))
                    .selected(sel);
                if ui.add(btn).clicked() {
                    self.tab = t;
                }
            }
            ui.separator();
            match self.tab {
                RibbonTab::Home => {
                    if ui.button("New chart").clicked() {
                        self.add_chart(DEFAULT_DSL.to_string(), format!("Chart {}", self.next_id));
                    }
                    if ui.button("Open…").clicked() {
                        self.load_project(PathBuf::from("project.gtp"));
                    }
                    if ui.button("Save").clicked() {
                        self.save_project();
                    }
                    ui.separator();
                    if ui.button("Delete").clicked() {
                        self.delete_selected();
                    }
                    ui.separator();
                    ui.label("Zoom");
                    if ui.button("−").clicked() {
                        self.zoom = (self.zoom * 0.8).max(0.2);
                    }
                    if ui.button("+").clicked() {
                        self.zoom = (self.zoom * 1.25).min(4.0);
                    }
                    ui.label(format!("{:.0}%", self.zoom * 100.0));
                    ui.separator();
                    ui.toggle_value(&mut self.show_dsl_editor, "DSL editor");
                    ui.toggle_value(&mut self.show_properties, "Properties");
                }
                RibbonTab::Design => {
                    let Some(item) = self.selected_chart_mut() else {
                        ui.weak("Select a chart to design it.");
                        return;
                    };
                    let o = &mut item.theme_overrides;
                    o.enabled = true;
                    ui.label("Palette");
                    color_row(ui, "Bar", &mut o.bar_color);
                    color_row(ui, "Bar highlight", &mut o.bar_highlight_color);
                    color_row(ui, "Milestone", &mut o.milestone_color);
                    color_row(ui, "Labels", &mut o.label_color);
                    color_row(ui, "Grid", &mut o.grid_color);
                    color_row(ui, "Today", &mut o.today_color);
                    color_row(ui, "Dependency", &mut o.dependency_color);
                    ui.separator();
                    ui.add(egui::DragValue::new(&mut o.bar_height).range(6.0..=40.0).prefix("bar h "));
                    ui.add(egui::DragValue::new(&mut o.row_height).range(12.0..=60.0).prefix("row h "));
                }
                RibbonTab::Tidying => {
                    if ui.button("Align left").clicked() {
                        self.align_left();
                    }
                    if ui.button("Align top").clicked() {
                        self.align_top();
                    }
                    if ui.button("Distribute horizontally").clicked() {
                        self.distribute_horizontally();
                    }
                }
                RibbonTab::Visualising => {
                    ui.label("Scale:");
                    let unit = self.selected_chart().map(|c| c.theme_overrides.scale_unit).unwrap_or(0);
                    for (idx, label) in ["Day", "Week", "Month"].iter().enumerate() {
                        if ui.selectable_label(unit == idx as u8, *label).clicked() {
                            if let Some(c) = self.selected_chart_mut() {
                                c.theme_overrides.scale_unit = idx as u8;
                                c.theme_overrides.enabled = true;
                            }
                        }
                    }
                    ui.separator();
                    if let Some(c) = self.selected_chart_mut() {
                        let o = &mut c.theme_overrides;
                        o.enabled = true;
                        ui.checkbox(&mut o.show_today, "Today marker");
                        ui.checkbox(&mut o.weekend_shading, "Weekend shading");
                        ui.separator();
                        ui.label("Bar labels:");
                        for (idx, label) in ["Left", "Right", "Center", "Hidden"].iter().enumerate() {
                            if ui.selectable_label(o.bar_label_pos == idx as u8, *label).clicked() {
                                o.bar_label_pos = idx as u8;
                            }
                        }
                    }
                }
            }
        });
    }

    fn align_left(&mut self) {
        if let Some(min_x) = self.items.iter().map(|i| i.x).reduce(f32::min) {
            for i in &mut self.items {
                i.x = min_x;
            }
            self.status = "Aligned left".into();
        }
    }

    fn align_top(&mut self) {
        if let Some(min_y) = self.items.iter().map(|i| i.y).reduce(f32::min) {
            for i in &mut self.items {
                i.y = min_y;
            }
            self.status = "Aligned top".into();
        }
    }

    fn distribute_horizontally(&mut self) {
        if self.items.len() < 2 {
            return;
        }
        self.items.sort_by_key(|i| i.x.to_bits());
        let n = self.items.len();
        let first_right = self.items[0].x + self.items[0].w;
        let last_left = self.items[n - 1].x;
        let gap = (last_left - first_right).max(0.0) / (n - 1) as f32;
        for i in 1..n {
            self.items[i].x = self.items[i - 1].x + self.items[i - 1].w + gap;
        }
        self.status = "Distributed horizontally".into();
    }
}

fn color_row(ui: &mut egui::Ui, label: &str, color: &mut crate::item::SerColor) {
    let mut arr: [u8; 3] = crate::item::SerColor::into(*color);
    ui.label(label);
    ui.color_edit_button_srgb(&mut arr);
    *color = arr.into();
}

/// The page canvas: horizontal scrolling slide-like area where charts live.
pub fn canvas_page(ui: &mut egui::Ui, app: &mut GraphToolApp) {
    let zoom = app.zoom;
    let page_w = 960.0;
    let page_h = 540.0;

    egui::ScrollArea::horizontal()
        .id_salt("canvas_h")
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, _| {
            egui::ScrollArea::vertical()
                .id_salt("canvas_v")
                .auto_shrink([false, false])
                .show_viewport(ui, |ui, _| {
                    let size = Vec2::new(page_w * zoom, page_h * zoom);
                    let (page_rect, page_resp) = ui.allocate_exact_size(size, egui::Sense::click());
                    ui.painter().rect_filled(page_rect, 2.0, egui::Color32::WHITE);
                    ui.painter().rect_stroke(
                        page_rect,
                        2.0,
                        egui::Stroke::new(1.0, egui::Color32::from_gray(180)),
                        egui::StrokeKind::Outside,
                    );

                    let pointer_on_item = app.items.iter().any(|_| false);
                    for item in &mut app.items {
                        let rect = egui::Rect::from_min_size(
                            page_rect.min + Vec2::new(item.x * zoom, item.y * zoom),
                            Vec2::new(item.w * zoom, item.h * zoom),
                        );
                        let resp = ui.allocate_rect(rect, egui::Sense::click_and_drag());
                        if resp.clicked() {
                            app.selected = Some(item.id);
                            app.status = format!("Selected `{}`", item.name);
                        }
                        let hovered = resp.hovered() || resp.dragged_by(egui::PointerButton::Primary);
                        let is_sel = app.selected == Some(item.id);

                        match item.chart() {
                            Ok(c) => crate::chart_render::draw_chart(ui, rect, &c, zoom),
                            Err(e) => {
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    e.clone(),
                                    egui::FontId::proportional(12.0),
                                    egui::Color32::RED,
                                );
                            }
                        }

                        // move
                        if resp.dragged_by(egui::PointerButton::Primary) {
                            match app.drag {
                                Some((id, _, origin, false)) if id == item.id => {
                                    let delta = resp.drag_delta();
                                    item.x = origin.x + delta.x / zoom;
                                    item.y = origin.y + delta.y / zoom;
                                }
                                _ => {
                                    let pos = resp.interact_pointer_pos().unwrap_or_default();
                                    app.drag = Some((item.id, egui::Vec2::new(pos.x, pos.y), Vec2::new(item.x, item.y), false));
                                }
                            }
                        } else if app.drag.map(|(id, _, _, resizing)| (id, !resizing)) == Some((item.id, true)) {
                            app.drag = None;
                        }

                        // resize handle (bottom-right)
                        let handle = egui::Rect::from_min_size(rect.right_bottom() - Vec2::new(12.0, 12.0), Vec2::splat(12.0));
                        let hresp = ui.allocate_rect(handle, egui::Sense::drag());
                        if hresp.dragged_by(egui::PointerButton::Primary) {
                            match app.drag {
                                Some((id, _, origin, true)) if id == item.id => {
                                    let delta = hresp.drag_delta();
                                    item.w = (origin.x + delta.x / zoom).max(120.0);
                                    item.h = (origin.y + delta.y / zoom).max(100.0);
                                }
                                _ => {
                                    let pos = hresp.interact_pointer_pos().unwrap_or_default();
                                    app.drag = Some((item.id, egui::Vec2::new(pos.x, pos.y), Vec2::new(item.w, item.h), true));
                                }
                            }
                        } else if app.drag.map(|(id, _, _, resizing)| (id, resizing)) == Some((item.id, true)) {
                            app.drag = None;
                        }

                        let frame_color = if is_sel {
                            egui::Color32::from_rgb(0, 120, 215)
                        } else if hovered {
                            egui::Color32::from_gray(160)
                        } else {
                            egui::Color32::from_gray(210)
                        };
                        ui.painter().rect_stroke(rect, 2.0, egui::Stroke::new(if is_sel { 2.0 } else { 1.0 }, frame_color), egui::StrokeKind::Inside);
                    }

                    // click on empty page area de-selects
                    if page_resp.clicked() && !pointer_on_item {
                        let on_any = app.items.iter().any(|i| {
                            let r = egui::Rect::from_min_size(
                                page_rect.min + Vec2::new(i.x * zoom, i.y * zoom),
                                Vec2::new(i.w * zoom, i.h * zoom),
                            );
                            page_resp.interact_pointer_pos().map(|pos| r.contains(pos)).unwrap_or(false)
                        });
                        if !on_any {
                            app.selected = None;
                        }
                    }
                });
        });
}
