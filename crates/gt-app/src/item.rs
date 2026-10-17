//! Chart items placed on the page, plus project (de)serialization.

use ganttcore::model::{BarLabelPos, Color, GanttChart, Theme, Unit};
use ganttcore::{parser, resolve};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ItemId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartItem {
    #[serde(skip)]
    pub id: ItemId,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub dsl: String,
    #[serde(default)]
    pub theme_overrides: ThemeOverrides,
    #[serde(skip)]
    pub last_dsl: String,
    #[serde(skip)]
    pub cached: Option<Result<GanttChart, String>>,
}

impl ChartItem {
    pub fn new(id: ItemId, name: String, dsl: String) -> Self {
        Self {
            id,
            name,
            x: 40.0,
            y: 40.0,
            w: 560.0,
            h: 320.0,
            dsl,
            theme_overrides: ThemeOverrides::default(),
            last_dsl: String::new(),
            cached: None,
        }
    }

    /// Returns the resolved chart, recomputing only when the DSL changed.
    pub fn chart(&mut self) -> Result<GanttChart, String> {
        if self.dsl != self.last_dsl || self.cached.is_none() {
            let result = parser::parse(&self.dsl)
                .map_err(|e| e.to_string())
                .and_then(|spec| resolve::resolve(&spec).map_err(|e| e.to_string()));
            self.cached = Some(result);
            self.last_dsl = self.dsl.clone();
        }
        match self.cached.as_mut().unwrap() {
            Ok(c) => {
                self.theme_overrides.apply(c);
                Ok(c.clone())
            }
            Err(e) => Err(e.clone()),
        }
    }

}

/// Serializable theme overrides applied after parsing the DSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeOverrides {
    pub enabled: bool,
    pub bar_color: SerColor,
    pub bar_highlight_color: SerColor,
    pub milestone_color: SerColor,
    pub label_color: SerColor,
    pub axis_color: SerColor,
    pub grid_color: SerColor,
    pub today_color: SerColor,
    pub dependency_color: SerColor,
    pub background: SerColor,
    pub bar_height: f32,
    pub row_height: f32,
    pub weekend_shading: bool,
    pub show_today: bool,
    pub bar_label_pos: u8, // 0 left, 1 right, 2 center, 3 hidden
    pub scale_unit: u8,    // 0 day, 1 week, 2 month
}

impl Default for ThemeOverrides {
    fn default() -> Self {
        let t = Theme::default();
        Self::from_theme(&t)
    }
}

impl ThemeOverrides {
    pub fn from_theme(t: &Theme) -> Self {
        Self {
            enabled: false,
            bar_color: t.bar_color.into(),
            bar_highlight_color: t.bar_highlight_color.into(),
            milestone_color: t.milestone_color.into(),
            label_color: t.label_color.into(),
            axis_color: t.axis_color.into(),
            grid_color: t.grid_color.into(),
            today_color: t.today_color.into(),
            dependency_color: t.dependency_color.into(),
            background: t.background.into(),
            bar_height: t.bar_height as f32,
            row_height: t.row_height as f32,
            weekend_shading: t.weekend_shading,
            show_today: t.show_today,
            bar_label_pos: 0,
            scale_unit: 0,
        }
    }

    pub fn apply(&self, chart: &mut GanttChart) {
        if !self.enabled {
            return;
        }
        let t = &mut chart.theme;
        t.bar_color = self.bar_color.into();
        t.bar_highlight_color = self.bar_highlight_color.into();
        t.milestone_color = self.milestone_color.into();
        t.label_color = self.label_color.into();
        t.axis_color = self.axis_color.into();
        t.grid_color = self.grid_color.into();
        t.today_color = self.today_color.into();
        t.dependency_color = self.dependency_color.into();
        t.background = self.background.into();
        t.bar_height = self.bar_height.max(4.0) as f64;
        t.row_height = self.row_height.max(8.0) as f64;
        t.weekend_shading = self.weekend_shading;
        t.show_today = self.show_today;
        t.bar_label_pos = match self.bar_label_pos {
            1 => BarLabelPos::Right,
            2 => BarLabelPos::Center,
            3 => BarLabelPos::Hidden,
            _ => BarLabelPos::Left,
        };
        chart.scale.unit = match self.scale_unit {
            1 => Unit::Week,
            2 => Unit::Month,
            _ => Unit::Day,
        };
    }
}

/// Serde-friendly RGB wrapper.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SerColor(pub u8, pub u8, pub u8);

impl From<Color> for SerColor {
    fn from(c: Color) -> Self {
        SerColor(c.0, c.1, c.2)
    }
}

impl From<SerColor> for Color {
    fn from(c: SerColor) -> Self {
        Color(c.0, c.1, c.2)
    }
}

impl From<SerColor> for [u8; 3] {
    fn from(c: SerColor) -> Self {
        [c.0, c.1, c.2]
    }
}

impl From<[u8; 3]> for SerColor {
    fn from(c: [u8; 3]) -> Self {
        SerColor(c[0], c[1], c[2])
    }
}

/// Default DSL used when a new chart is inserted from the ribbon.
pub const DEFAULT_DSL: &str = r#"@startgantt
projectscale daily
project starts 2025-01-06
title New chart

[T1] Prototype design starts 2025-01-06 and lasts 8 days
[T1] is 60% complete
[T2] Prototype build starts at [T1]'s end and lasts 10 days
[M1] Demo happens at [T2]'s end
@endgantt
"#;
