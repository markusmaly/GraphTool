//! ganttcore — parser, resolver, layout and SVG renderer for
//! thinkcell-style Gantt charts driven by a PlantUML-like DSL.

pub mod datasets;
pub mod examples;
pub mod layout;
pub mod lexer;
pub mod model;
pub mod parser;
pub mod resolve;
pub mod svg;

pub use model::{BarLabelPos, Color, Date, Dependency, GanttChart, Scale, Task, Theme, Unit};

/// Parse + resolve + render in one call.
pub fn render_source_to_svg(src: &str) -> Result<String, String> {
    let spec = parser::parse(src).map_err(|e| e.to_string())?;
    let mut chart = resolve::resolve(&spec).map_err(|e| e.to_string())?;
    // Default today marker: actual current date if enabled and not set.
    if chart.theme.show_today && chart.today.is_none() {
        chart.today = Some(today_utc());
    }
    Ok(svg::render_to_svg(&chart))
}

/// Current UTC date (no chrono dependency).
pub fn today_utc() -> Date {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as i64;
    Date::from_epoch_days(secs / 86_400)
}
