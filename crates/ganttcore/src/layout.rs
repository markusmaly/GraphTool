//! Layout engine: computes pixel geometry for a resolved chart.

use crate::model::*;

pub const LEFT_GUTTER: f64 = 160.0;
pub const ROW_AREA_TOP: f64 = 56.0;
pub const DAY_WIDTH_DAILY: f64 = 22.0;
pub const WEEK_WIDTH: f64 = 110.0;
pub const MONTH_WIDTH: f64 = 320.0;

#[derive(Debug, Clone)]
pub struct Geom {
    pub width: f64,
    pub height: f64,
    /// Chart area (date axis) origin.
    pub chart_x: f64,
    pub chart_y: f64,
    pub chart_w: f64,
    pub chart_h: f64,
    /// Day-cell width in the current scale.
    pub day_w: f64,
    /// First rendered day (epoch days).
    pub first_day: i64,
    /// Last rendered day (epoch days, inclusive).
    pub last_day: i64,
}

#[derive(Debug, Clone)]
pub struct BarGeom {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    /// Width of the completed portion.
    pub done_w: f64,
}

#[derive(Debug, Clone)]
pub struct MilestoneGeom {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

#[derive(Debug, Clone)]
pub struct ScaleTick {
    pub x: f64,
    pub label: String,
    /// Major tick (e.g. month header) vs minor (day).
    pub major: bool,
    /// Day this tick starts (epoch days).
    pub day: i64,
    /// Days spanned by this tick.
    pub span: i64,
}

pub struct Layout {
    pub geom: Geom,
    pub bars: Vec<BarGeom>,
    pub milestones: Vec<MilestoneGeom>,
    pub ticks: Vec<ScaleTick>,
    /// Weekend shading rectangles (in daily scale).
    pub weekend_rects: Vec<(f64, f64, f64, f64)>,
    pub today_x: Option<f64>,
    /// Dependency polylines (points).
    pub dep_lines: Vec<Vec<(f64, f64)>>,
}

pub fn layout(chart: &GanttChart) -> Layout {
    let theme = &chart.theme;

    // Determine date range from tasks.
    let mut min_day: Option<i64> = None;
    let mut max_day: Option<i64> = None;
    for t in &chart.tasks {
        if let Some(s) = t.start {
            let s = s.to_epoch_days();
            let e = t.end_date().map(|d| d.to_epoch_days()).unwrap_or(s);
            min_day = Some(match min_day { None => s, Some(m) => m.min(s) });
            max_day = Some(match max_day { None => e, Some(m) => m.max(e) });
        }
    }
    let first_day = min_day.unwrap_or(0);
    let last_day = max_day.unwrap_or(first_day);
    // Include today in range if marker enabled.
    let mut first_day = first_day;
    let mut last_day = last_day;
    if theme.show_today {
        if let Some(today) = chart.today {
            let t = today.to_epoch_days();
            first_day = first_day.min(t);
            last_day = last_day.max(t);
        }
    }
    let span_days = (last_day - first_day + 1).max(1);

    let base_day_w = match chart.scale.unit {
        Unit::Day => DAY_WIDTH_DAILY,
        Unit::Week => WEEK_WIDTH / 7.0,
        Unit::Month => MONTH_WIDTH / 30.0,
    };
    // Cap the total chart width so long timelines (e.g. decades of daily
    // data) stay renderable; the day cell shrinks instead.
    const MAX_CHART_W: f64 = 4000.0;
    let day_w = {
        let w = base_day_w * span_days as f64;
        if w > MAX_CHART_W { base_day_w * MAX_CHART_W / w } else { base_day_w }
    };

    let chart_w = day_w * span_days as f64;
    let rows = chart.tasks.len().max(1);
    let chart_h = rows as f64 * theme.row_height;
    let width = LEFT_GUTTER + chart_w + 24.0;
    let height = ROW_AREA_TOP + chart_h + 24.0 + if chart.title.is_some() { 28.0 } else { 0.0 };
    let geom = Geom {
        width,
        height,
        chart_x: LEFT_GUTTER,
        chart_y: ROW_AREA_TOP,
        chart_w,
        chart_h,
        day_w,
        first_day,
        last_day,
    };

    // Bars & milestones.
    let mut bars = Vec::new();
    let mut milestones = Vec::new();
    for t in &chart.tasks {
        let y = geom.chart_y + t.row as f64 * theme.row_height;
        let cy = y + theme.row_height / 2.0;
        if t.is_milestone {
            if let Some(s) = t.start {
                let cx = geom.chart_x + (s.to_epoch_days() - first_day) as f64 * day_w + day_w / 2.0;
                milestones.push(MilestoneGeom { cx, cy, r: theme.bar_height * 0.45 });
            }
        } else if let (Some(s), Some(e)) = (t.start, t.end_date()) {
            let x = geom.chart_x + (s.to_epoch_days() - first_day) as f64 * day_w;
            let w = ((e.to_epoch_days() - s.to_epoch_days() + 1) as f64) * day_w;
            let h = theme.bar_height;
            let done_w = w * (t.percent as f64 / 100.0);
            bars.push(BarGeom { x, y: cy - h / 2.0, w, h, done_w });
        }
    }

    // Scale ticks.
    let mut ticks = Vec::new();
    match chart.scale.unit {
        Unit::Day => {
            // Two-level header: month names over day numbers.
            let mut day = first_day;
            let mut month_start: Option<i64> = None;
            let mut idx = 0;
            while day <= last_day {
                let x = geom.chart_x + idx as f64 * day_w;
                let date = Date::from_epoch_days(day);
                let label = format!("{}", date.day);
                ticks.push(ScaleTick { x, label, major: false, day, span: 1 });
                if date.day == 1 || month_start.is_none() {
                    if let Some(ms) = month_start {
                        ticks.push(ScaleTick {
                            x: geom.chart_x + (ms - first_day) as f64 * day_w,
                            label: month_label(ms),
                            major: true,
                            day: ms,
                            span: day - ms,
                        });
                    }
                    month_start = Some(day);
                }
                if day == last_day {
                    if let Some(ms) = month_start {
                        ticks.push(ScaleTick {
                            x: geom.chart_x + (ms - first_day) as f64 * day_w,
                            label: month_label(ms),
                            major: true,
                            day: ms,
                            span: day - ms + 1,
                        });
                    }
                }
                day += 1;
                idx += 1;
            }
        }
        Unit::Week => {
            // Week header W01.. plus month labels.
            let mut day = first_day;
            while day <= last_day {
                let date = Date::from_epoch_days(day);
                let wk = iso_week_number(date);
                let x = geom.chart_x + (day - first_day) as f64 * day_w;
                ticks.push(ScaleTick { x, label: format!("W{wk:02}"), major: false, day, span: 7 });
                day += 7;
            }
        }
        Unit::Month => {
            let mut day = first_day;
            while day <= last_day {
                let date = Date::from_epoch_days(day);
                let x = geom.chart_x + (day - first_day) as f64 * day_w;
                ticks.push(ScaleTick { x, label: month_label(day), major: true, day, span: 30 });
                // advance to first of next month
                let mut d = date;
                loop {
                    d = d.add_days(1);
                    if d.day == 1 { break; }
                }
                day = d.to_epoch_days();
            }
        }
    }

    // Weekend shading (daily scale only).
    let mut weekend_rects = Vec::new();
    if theme.weekend_shading && chart.scale.unit == Unit::Day {
        let mut day = first_day;
        let mut idx = 0;
        while day <= last_day {
            let date = Date::from_epoch_days(day);
            if date.is_weekend() {
                let x = geom.chart_x + idx as f64 * day_w;
                weekend_rects.push((x, geom.chart_y, day_w, chart_h));
            }
            day += 1;
            idx += 1;
        }
    }

    // Today marker.
    let today_x = if theme.show_today {
        chart.today.map(|t| {
            let d = t.to_epoch_days();
            if d >= first_day && d <= last_day {
                Some(geom.chart_x + (d - first_day) as f64 * day_w + day_w / 2.0)
            } else {
                None
            }
        })
        .flatten()
    } else {
        None
    };

    // Dependency lines: from end of `from` to start of `to`, elbow routing.
    let mut dep_lines = Vec::new();
    let by_name: std::collections::BTreeMap<&str, &Task> =
        chart.tasks.iter().map(|t| (t.name.as_str(), t)).collect();
    for dep in &chart.dependencies {
        let (Some(f), Some(t)) = (by_name.get(dep.from.as_str()), by_name.get(dep.to.as_str())) else { continue };
        let (_fs, Some(fe)) = (f.start, f.end_date()) else { continue };
        let Some(ts) = t.start else { continue };
        let fy = geom.chart_y + f.row as f64 * theme.row_height + theme.row_height / 2.0;
        let ty = geom.chart_y + t.row as f64 * theme.row_height + theme.row_height / 2.0;
        let x1 = geom.chart_x + (fe.to_epoch_days() - first_day + 1) as f64 * day_w;
        let x2 = geom.chart_x + (ts.to_epoch_days() - first_day) as f64 * day_w;
        if t.row > f.row {
            // down-and-over
            dep_lines.push(vec![(x1, fy), (x1, ty - theme.bar_height / 2.0 - 3.0), (x2 - 6.0, ty - theme.bar_height / 2.0 - 3.0), (x2 - 6.0, ty), (x2, ty)]);
        } else {
            dep_lines.push(vec![(x1, fy), (x1, geom.chart_y + chart_h + 6.0), (x2 - 6.0, geom.chart_y + chart_h + 6.0), (x2 - 6.0, ty), (x2, ty)]);
        }
    }

    Layout { geom, bars, milestones, ticks, weekend_rects, today_x, dep_lines }
}

fn month_label(day: i64) -> String {
    let d = Date::from_epoch_days(day);
    let names = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    format!("{} {}", names[(d.month - 1) as usize], d.year)
}

fn iso_week_number(date: Date) -> u32 {
    // Simplified ISO week calculation.
    let ord = date.to_epoch_days() - Date::new(date.year, 1, 1).to_epoch_days();
    let wd_mon0 = date.weekday() - 1;
    ((ord + 10 - wd_mon0 as i64) / 7).max(1) as u32
}
