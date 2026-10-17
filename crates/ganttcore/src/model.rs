//! Data model for Gantt charts.

/// A calendar date (proleptic Gregorian), day-resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Days since 1970-01-01 (may be negative).
    pub fn to_epoch_days(&self) -> i64 {
        let (y, m) = if self.month <= 2 {
            (self.year - 1, self.month + 12)
        } else {
            (self.year, self.month)
        };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = (y - era * 400) as i64;
        let mp = (m as i64 + 9) % 12;
        let doy = (153 * mp + 2) / 5 + self.day as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era as i64 * 146097 + doe - 719468
    }

    pub fn from_epoch_days(days: i64) -> Self {
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        Self::new(if m <= 2 { y + 1 } else { y } as i32, m as u32, d as u32)
    }

    pub fn add_days(&self, days: i64) -> Self {
        Self::from_epoch_days(self.to_epoch_days() + days)
    }

    /// ISO weekday: 1=Monday .. 7=Sunday.
    pub fn weekday(&self) -> u32 {
        let d = self.to_epoch_days();
        // 1970-01-01 was a Thursday (weekday 4).
        let w = (d + 3).rem_euclid(7);
        (w + 1) as u32
    }

    pub fn is_weekend(&self) -> bool {
        matches!(self.weekday(), 6 | 7)
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// Color as #RRGGBB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const BLACK: Color = Color(0, 0, 0);
    pub const WHITE: Color = Color(255, 255, 255);

    pub fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

/// Named colors matching a corporate palette (CSS-like subset).
pub fn color_by_name(name: &str) -> Option<Color> {
    let c = match name.to_ascii_lowercase().as_str() {
        "black" => Color(0, 0, 0),
        "white" => Color(255, 255, 255),
        "gray" | "grey" => Color(128, 128, 128),
        "silver" => Color(192, 192, 192),
        "red" => Color(239, 62, 54),
        "crimson" => Color(220, 20, 60),
        "orange" => Color(255, 128, 0),
        "amber" => Color(255, 191, 0),
        "yellow" => Color(253, 218, 36),
        "green" => Color(34, 139, 34),
        "forestgreen" => Color(34, 139, 34),
        "olive" => Color(107, 142, 35),
        "teal" => Color(0, 128, 128),
        "cyan" => Color(0, 174, 239),
        "azure" => Color(0, 127, 255),
        "dodgerblue" => Color(30, 144, 255),
        "blue" => Color(0, 82, 155),
        "midnightblue" => Color(25, 25, 112),
        "navy" => Color(0, 0, 128),
        "purple" => Color(128, 0, 128),
        "violet" => Color(238, 130, 238),
        "brown" => Color(165, 42, 42),
        "whitesmoke" => Color(245, 245, 245),
        "lightgray" | "lightgrey" => Color(211, 211, 211),
        _ => return None,
    };
    Some(c)
}

/// Unit for durations and scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Day,
    Week,
    Month,
}

impl Unit {
    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "daily" | "day" | "days" => Some(Unit::Day),
            "weekly" | "week" | "weeks" => Some(Unit::Week),
            "monthly" | "month" | "months" => Some(Unit::Month),
            _ => None,
        }
    }
}

/// Where a task's bar label sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarLabelPos {
    Left,
    Right,
    Center,
    Hidden,
}

/// A task or milestone in the chart.
#[derive(Debug, Clone)]
pub struct Task {
    /// Raw name as written between brackets, e.g. `T1`.
    pub name: String,
    /// Display label; defaults to name.
    pub label: String,
    /// Start date (inclusive). `None` until resolved.
    pub start: Option<Date>,
    /// Duration in calendar days; 0 for pure milestones.
    pub duration_days: u64,
    /// Completion 0..=100.
    pub percent: u8,
    /// True if this entry renders as a milestone diamond.
    pub is_milestone: bool,
    /// Color override.
    pub color: Option<Color>,
    /// Bar label position override.
    pub label_pos: Option<BarLabelPos>,
    /// Renderer-assigned row.
    pub row: usize,
    /// Explicit end date if given ("ends 2025-01-15").
    pub end: Option<Date>,
}

impl Task {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            label: name.clone(),
            name,
            start: None,
            duration_days: 0,
            percent: 0,
            is_milestone: false,
            color: None,
            label_pos: None,
            row: 0,
            end: None,
        }
    }

    /// End date (inclusive) of the bar.
    pub fn end_date(&self) -> Option<Date> {
        match (self.start, self.end) {
            (Some(s), Some(e)) => Some(if e >= s { e } else { s }),
            (Some(s), None) => Some(if self.duration_days == 0 {
                s
            } else {
                s.add_days(self.duration_days as i64 - 1)
            }),
            (None, Some(e)) => Some(e),
            (None, None) => None,
        }
    }
}

/// Dependency arrow between two tasks.
#[derive(Debug, Clone)]
pub struct Dependency {
    pub from: String,
    pub to: String,
}

/// Visual theme ("skinparam" settings + corporate identity).
#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub bar_color: Color,
    pub bar_highlight_color: Color,
    pub bar_border_radius: f64,
    pub milestone_color: Color,
    pub label_color: Color,
    pub axis_color: Color,
    pub scale_color: Color,
    pub grid_color: Color,
    pub today_color: Color,
    pub dependency_color: Color,
    pub font_family: String,
    pub bar_height: f64,
    pub row_height: f64,
    pub bar_label_pos: BarLabelPos,
    pub show_today: bool,
    pub weekend_shading: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::WHITE,
            bar_color: Color(0, 82, 155),
            bar_highlight_color: Color(0, 122, 207),
            bar_border_radius: 2.0,
            milestone_color: Color(220, 20, 60),
            label_color: Color(33, 37, 41),
            axis_color: Color(120, 120, 120),
            scale_color: Color(90, 90, 90),
            grid_color: Color(233, 236, 239),
            today_color: Color(220, 20, 60),
            dependency_color: Color(110, 110, 110),
            font_family: "Arial, Helvetica, sans-serif".into(),
            bar_height: 18.0,
            row_height: 28.0,
            bar_label_pos: BarLabelPos::Left,
            show_today: false,
            weekend_shading: true,
        }
    }
}

/// Calendar scale configuration.
#[derive(Debug, Clone)]
pub struct Scale {
    pub unit: Unit,
    /// Project start date.
    pub start: Option<Date>,
    /// Number of units to render; None = auto from data.
    pub length: Option<u64>,
}

impl Default for Scale {
    fn default() -> Self {
        Self { unit: Unit::Day, start: None, length: None }
    }
}

/// Parsed, resolved chart ready for layout.
#[derive(Debug, Clone)]
pub struct GanttChart {
    pub scale: Scale,
    pub theme: Theme,
    pub tasks: Vec<Task>,
    pub dependencies: Vec<Dependency>,
    pub today: Option<Date>,
    /// Title shown above the chart.
    pub title: Option<String>,
}

/// Parser's intermediate representation before dates are resolved.
#[derive(Debug, Clone)]
pub struct Spec {
    pub directives: Vec<Directive>,
    pub theme: Theme,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Directive {
    /// `projectscale daily`
    ProjectScale(Unit),
    /// `project starts 2025-01-06`
    ProjectStarts(Date),
    /// `today` marker
    Today,
    /// `[Task] ...` line, raw spec for later resolution.
    Task(TaskSpec),
    /// `[T1] is 40% complete`
    PercentComplete { target: String, percent: u8 },
    /// `[T2] starts at [T1]'s end and lasts 10 days`
    Dependency { from: String, to: String },
    /// `title Some Title`
    Title(String),
}

/// Raw task line, resolved later (dates may reference other tasks).
#[derive(Debug, Clone)]
pub struct TaskSpec {
    pub name: String,
    pub label: Option<String>,
    pub start_spec: StartSpec,
    pub duration: Option<u64>,
    pub duration_unit: Unit,
    pub end_spec: Option<EndSpec>,
    pub percent: Option<u8>,
    pub is_milestone: bool,
    pub color: Option<Color>,
    pub label_pos: Option<BarLabelPos>,
}

#[derive(Debug, Clone)]
pub enum StartSpec {
    Fixed(Date),
    /// start at another task's end (or start), plus offset days
    Ref { name: String, offset_days: i64, at_end: bool },
    Unspecified,
}

#[derive(Debug, Clone)]
pub enum EndSpec {
    Fixed(Date),
    Ref { name: String, offset_days: i64 },
}
