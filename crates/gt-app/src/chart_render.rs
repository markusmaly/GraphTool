//! Draws a resolved Gantt chart with egui vector primitives (no raster).

use eframe::egui;
use ganttcore::layout::{layout, Layout};
use ganttcore::model::{GanttChart, Theme};

fn to_color(c: ganttcore::model::Color) -> egui::Color32 {
    egui::Color32::from_rgb(c.0, c.1, c.2)
}

/// Draw the chart fitted into `rect` (item box on the page).
pub fn draw_chart(ui: &mut egui::Ui, rect: egui::Rect, chart: &GanttChart, zoom: f32) {
    let lay: Layout = layout(chart);
    let t: &Theme = &chart.theme;

    // Scale layout geometry into the item rect.
    let sx = rect.width() / lay.geom.width as f32;
    let sy = rect.height() / lay.geom.height as f32;
    let s = sx.min(sy);
    let ox = rect.left_top();
    let map = |x: f64, y: f64| -> egui::Pos2 {
        egui::pos2(ox.x + x as f32 * s, ox.y + y as f32 * s)
    };
    let p = ui.painter_at(rect);

    // background
    p.rect_filled(rect, 2.0, to_color(t.background));

    // title
    if let Some(title) = &chart.title {
        p.text(
            map(lay.geom.chart_x, 20.0),
            egui::Align2::LEFT_CENTER,
            title.clone(),
            egui::FontId::proportional((15.0 * s * zoom).max(8.0)),
            to_color(t.label_color),
        );
    }

    // weekend shading
    for (x, y, w, h) in &lay.weekend_rects {
        let a = map(*x, *y);
        let b = map(x + w, y + h);
        p.rect_filled(egui::Rect::from_min_max(a, b), 0.0, egui::Color32::from_rgb(242, 244, 246));
    }

    // grid
    for tick in &lay.ticks {
        let a = map(tick.x, lay.geom.chart_y);
        let b = map(tick.x, lay.geom.chart_y + lay.geom.chart_h);
        p.line_segment([a, b], egui::Stroke::new(1.0, to_color(t.grid_color)));
    }

    // scale labels
    for tick in &lay.ticks {
        let (anchor, y_off, size, strong) = if tick.major {
            (egui::Align2::LEFT_CENTER, -10.0, 12.0, true)
        } else {
            (egui::Align2::CENTER_CENTER, -26.0, 10.0, false)
        };
        let pos = map(tick.x + if tick.major { 3.0 } else { lay.geom.day_w / 2.0 }, lay.geom.chart_y + y_off);
        p.text(
            pos,
            anchor,
            tick.label.clone(),
            egui::FontId::proportional((size * s * zoom).max(7.0)),
            to_color(t.scale_color),
        );
        let _ = strong;
    }

    // row labels
    for task in &chart.tasks {
        let y = lay.geom.chart_y + task.row as f64 * t.row_height + t.row_height / 2.0 + 4.0;
        p.text(
            map(lay.geom.chart_x - 8.0, y),
            egui::Align2::RIGHT_CENTER,
            task.label.clone(),
            egui::FontId::proportional((12.0 * s * zoom).max(8.0)),
            to_color(t.label_color),
        );
    }

    // dependency lines
    for line in &lay.dep_lines {
        let pts: Vec<egui::Pos2> = line.iter().map(|(x, y)| map(*x, *y)).collect();
        if pts.len() >= 2 {
            p.add(egui::Shape::line(pts.clone(), egui::Stroke::new(1.2, to_color(t.dependency_color))));
            // arrowhead
            let last = pts[pts.len() - 1];
            let prev = pts[pts.len() - 2];
            let dir = (last - prev).normalized();
            let perp = egui::Vec2::new(-dir.y, dir.x);
            let a = last - dir * 6.0 + perp * 3.0;
            let b = last - dir * 6.0 - perp * 3.0;
            p.add(egui::Shape::convex_polygon(
                vec![last, a, b],
                to_color(t.dependency_color),
                egui::Stroke::NONE,
            ));
        }
    }

    // bars
    for bar in &lay.bars {
        let a = map(bar.x, bar.y);
        let b = map(bar.x + bar.w, bar.y + bar.h);
        let r = egui::Rect::from_min_max(a, b);
        p.rect_filled(r, t.bar_border_radius as f32 * s, to_color(t.bar_color));
        if bar.done_w > 0.0 {
            let d = map(bar.x + bar.done_w, bar.y + bar.h);
            let rd = egui::Rect::from_min_max(a, d);
            p.rect_filled(rd, t.bar_border_radius as f32 * s, to_color(t.bar_highlight_color));
        }
    }

    // milestones
    for m in &lay.milestones {
        let c = map(m.cx, m.cy);
        let r = m.r as f32 * s;
        p.add(egui::Shape::convex_polygon(
            vec![c + egui::vec2(0.0, -r), c + egui::vec2(r, 0.0), c + egui::vec2(0.0, r), c + egui::vec2(-r, 0.0)],
            to_color(t.milestone_color),
            egui::Stroke::NONE,
        ));
    }

    // today marker
    if let Some(x) = lay.today_x {
        let a = map(x, lay.geom.chart_y);
        let b = map(x, lay.geom.chart_y + lay.geom.chart_h);
        p.line_segment([a, b], egui::Stroke::new(1.5, to_color(t.today_color)));
    }
}
