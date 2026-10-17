//! SVG writer: renders a resolved chart (with layout) to an SVG string.

use crate::layout::{layout, Layout};
use crate::model::*;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub fn render_to_svg(chart: &GanttChart) -> String {
    let lay: Layout = layout(chart);
    let t = &chart.theme;
    let g = &lay.geom;
    let mut s = String::with_capacity(16 * 1024);
    let title_h = if chart.title.is_some() { 28.0 } else { 0.0 };

    s.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.1}\" height=\"{:.1}\" viewBox=\"0 0 {:.1} {:.1}\" font-family=\"{}\">\n",
        g.width, g.height, g.width, g.height, esc(&t.font_family)
    ));
    s.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\"/>\n",
        g.width, g.height, t.background.hex()
    ));

    if let Some(title) = &chart.title {
        s.push_str(&format!(
            "  <text x=\"{:.1}\" y=\"20\" font-size=\"15\" font-weight=\"bold\" fill=\"{}\">{}</text>\n",
            g.chart_x, t.label_color.hex(), esc(title)
        ));
    }

    // Weekend shading.
    for (x, y, w, h) in &lay.weekend_rects {
        s.push_str(&format!(
            "  <rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#F2F4F6\"/>\n",
            x, y, w, h
        ));
    }

    // Grid: vertical lines at ticks.
    for tick in &lay.ticks {
        s.push_str(&format!(
            "  <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\"/>\n",
            tick.x, g.chart_y, tick.x, g.chart_y + g.chart_h, t.grid_color.hex()
        ));
    }

    // Scale labels.
    for tick in &lay.ticks {
        if tick.major {
            s.push_str(&format!(
                "  <text x=\"{:.2}\" y=\"{:.1}\" font-size=\"12\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
                tick.x + 3.0, g.chart_y - 10.0, t.scale_color.hex(), esc(&tick.label)
            ));
        } else {
            s.push_str(&format!(
                "  <text x=\"{:.2}\" y=\"{:.1}\" font-size=\"10\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n",
                tick.x + t_day_center(tick, &lay), g.chart_y - 26.0 + title_h, t.scale_color.hex(), esc(&tick.label)
            ));
        }
    }

    // Row labels.
    for task in &chart.tasks {
        let y = g.chart_y + task.row as f64 * t.row_height + t.row_height / 2.0 + 4.0;
        s.push_str(&format!(
            "  <text x=\"{:.1}\" y=\"{:.1}\" font-size=\"12\" fill=\"{}\" text-anchor=\"end\">{}</text>\n",
            g.chart_x - 8.0, y, t.label_color.hex(), esc(&task.label)
        ));
    }

    // Dependency lines.
    for line in &lay.dep_lines {
        let pts: Vec<String> = line.iter().map(|(x, y)| format!("{:.2},{:.2}", x, y)).collect();
        s.push_str(&format!(
            "  <polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.2\"/>\n",
            pts.join(" "), t.dependency_color.hex()
        ));
        if let Some((lx, ly)) = line.last() {
            s.push_str(&format!(
                "  <path d=\"M {:.2} {:.2} l -6 -3 l 0 6 z\" fill=\"{}\"/>\n",
                lx, ly, t.dependency_color.hex()
            ));
        }
    }

    // Bars.
    for (task, bar) in chart.tasks.iter().zip(&lay.bars) {
        s.push_str(&format!(
            "  <rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" rx=\"{:.2}\" fill=\"{}\"{}>/>\n",
            bar.x,
            bar.y,
            bar.w,
            bar.h,
            t.bar_border_radius,
            task.color.unwrap_or(t.bar_color).hex(),
            ""
        ));
        if bar.done_w > 0.0 {
            s.push_str(&format!(
                "  <rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" rx=\"{:.2}\" fill=\"{}\"/>\n",
                bar.x, bar.y, bar.done_w, bar.h, t.bar_border_radius,
                t.bar_highlight_color.hex()
            ));
        }
    }

    // Milestones.
    for (task, m) in chart.tasks.iter().zip(&lay.milestones) {
        let c = task.color.unwrap_or(t.milestone_color);
        s.push_str(&format!(
            "  <path d=\"M {:.2} {:.2} l {:.2} {:.2} l {:.2} {:.2} l {:.2} {:.2} z\" fill=\"{}\"/>\n",
            m.cx, m.cy - m.r,
            m.r, m.r,
            -m.r, m.r,
            m.r, -m.r,
            c.hex()
        ));
    }

    s.push_str("</svg>\n");
    s
}

fn t_day_center(_tick: &crate::layout::ScaleTick, lay: &Layout) -> f64 {
    lay.geom.day_w / 2.0
}
