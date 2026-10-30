//! Slide-area negotiation and placement presets for GraphTool.
//!
//! Phase 0 skeleton. Computes the full-slide graphing area by deducting the
//! title, slide number and footer bounding boxes and the master-derived left
//! and right edge margins, then places charts via the placement presets
//! (full, 1/2, 1/3, 2/3, 1/4, 1/6).
//!
//! NOTE: serde derives were temporarily removed for the Phase 0 sandbox
//! build (no C toolchain available there); they are restored as of this
//! revision, per the confirmed architecture (serialisable, versioned
//! ChartSpec embedded in the PPTX as a custom XML part).

use serde::{Deserialize, Serialize};


/// A rectangle in points, top-left origin.
#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// Bounding boxes to deduct before any size calculation.
#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Deductions {
    pub title: Option<Rect>,
    pub slide_number: Option<Rect>,
    pub footer: Option<Rect>,
    /// Master-derived left/right edge margins.
    pub margin_left: f64,
    pub margin_right: f64,
}

/// Compute the full-slide graphing area: the slide rectangle minus all
/// deductions. Later phases detect the boxes from the layout/master
/// automatically (this Phase 0 function is pure and unit-testable).
pub fn full_area(slide: Rect, d: &Deductions) -> Rect {
    let x = slide.x + d.margin_left;
    let mut w = slide.w - d.margin_left - d.margin_right;
    let mut y = slide.y;
    let mut h = slide.h;
    for r in [d.title, d.slide_number, d.footer].into_iter().flatten() {
        // A deduction spans the full remaining width; expand vertically.
        let top = r.y - slide.y;
        let bottom = (r.y + r.h) - slide.y;
        if top < h * 0.5 {
            y += bottom;
            h -= bottom;
        } else {
            h -= bottom - top;
        }
    }
    w = w.max(0.0);
    h = h.max(0.0);
    Rect { x, y, w, h }
}

/// The placement presets: fraction and position within the full area.
#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
pub enum Placement {
    Full,
    HalfLeft, HalfRight,
    ThirdLeft, ThirdMiddle, ThirdRight,
    TwoThirdsLeft, TwoThirdsRight,
    QuarterTopLeft, QuarterTopRight, QuarterBottomLeft, QuarterBottomRight,
    SixthTopLeft, SixthTopMiddle, SixthTopRight,
    SixthBottomLeft, SixthBottomMiddle, SixthBottomRight,
}

/// Compute the graphing rectangle for a placement preset within the full
/// area. Presets tile without gaps or overlaps.
pub fn place(area: Rect, p: Placement) -> Rect {
    use Placement::*;
    let (col, row, cw, ch) = match p {
        Full => (0.0, 0.0, 1.0, 1.0),
        HalfLeft => (0.0, 0.0, 0.5, 1.0),
        HalfRight => (0.5, 0.0, 0.5, 1.0),
        ThirdLeft => (0.0, 0.0, 1.0 / 3.0, 1.0),
        ThirdMiddle => (1.0 / 3.0, 0.0, 1.0 / 3.0, 1.0),
        ThirdRight => (2.0 / 3.0, 0.0, 1.0 / 3.0, 1.0),
        TwoThirdsLeft => (0.0, 0.0, 2.0 / 3.0, 1.0),
        TwoThirdsRight => (1.0 / 3.0, 0.0, 2.0 / 3.0, 1.0),
        QuarterTopLeft => (0.0, 0.0, 0.5, 0.5),
        QuarterTopRight => (0.5, 0.0, 0.5, 0.5),
        QuarterBottomLeft => (0.0, 0.5, 0.5, 0.5),
        QuarterBottomRight => (0.5, 0.5, 0.5, 0.5),
        SixthTopLeft => (0.0, 0.0, 1.0 / 3.0, 0.5),
        SixthTopMiddle => (1.0 / 3.0, 0.0, 1.0 / 3.0, 0.5),
        SixthTopRight => (2.0 / 3.0, 0.0, 1.0 / 3.0, 0.5),
        SixthBottomLeft => (0.0, 0.5, 1.0 / 3.0, 0.5),
        SixthBottomMiddle => (1.0 / 3.0, 0.5, 1.0 / 3.0, 0.5),
        SixthBottomRight => (2.0 / 3.0, 0.5, 1.0 / 3.0, 0.5),
    };
    Rect { x: area.x + col * area.w, y: area.y + row * area.h, w: cw * area.w, h: ch * area.h }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLIDE: Rect = Rect { x: 0.0, y: 0.0, w: 720.0, h: 405.0 };

    #[test]
    fn full_area_deducts_margins_title_and_footer() {
        let d = Deductions {
            title: Some(Rect { x: 0.0, y: 0.0, w: 720.0, h: 90.0 }),
            slide_number: None,
            footer: Some(Rect { x: 0.0, y: 375.0, w: 720.0, h: 30.0 }),
            margin_left: 40.0,
            margin_right: 40.0,
        };
        let a = full_area(SLIDE, &d);
        assert_eq!(a.x, 40.0);
        assert_eq!(a.w, 640.0);
        assert_eq!(a.y, 90.0);      // title deducted at the top
        assert_eq!(a.h, 285.0);    // footer deducted at the bottom
    }

    #[test]
    fn no_deductions_yields_slide_minus_margins() {
        let d = Deductions { margin_left: 20.0, margin_right: 20.0, ..Default::default() };
        let a = full_area(SLIDE, &d);
        assert_eq!((a.x, a.w, a.y, a.h), (20.0, 680.0, 0.0, 405.0));
    }

    #[test]
    fn placement_presets_tile_without_gaps_or_overlaps() {
        let d = Deductions::default();
        let a = full_area(SLIDE, &d);
        let l = place(a, Placement::HalfLeft);
        let r = place(a, Placement::HalfRight);
        assert_eq!(l.x + l.w, r.x);            // halves join
        assert_eq!(l.w + r.w, a.w);           // no gap
        assert_eq!(l.h, r.h);
        let t = place(a, Placement::ThirdLeft);
        let two = place(a, Placement::TwoThirdsRight);
        assert_eq!(t.w + two.w, a.w);         // 1/3 + 2/3 = full
        assert_eq!(t.x + t.w, two.x);
        let q = place(a, Placement::SixthTopMiddle);
        assert_eq!(q.w, a.w / 3.0);
        assert_eq!(q.h, a.h / 2.0);
        let f = place(a, Placement::Full);
        assert_eq!((f.x, f.y, f.w, f.h), (a.x, a.y, a.w, a.h));
    }
}
