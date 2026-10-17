use ganttcore::{parser, render_source_to_svg, resolve};

const SIMPLE: &str = r#"
@startgantt
projectscale daily
project starts 2025-01-06
title Sample project

skinparam barColor MidnightBlue

[T1] Prototype design starts 2025-01-06 and lasts 8 days
[T1] is 40% complete
[T2] Prototype build starts at [T1]'s end and lasts 10 days
[M1] First demo happens at [T2]'s end
@endgantt
"#;

#[test]
fn parses_and_resolves() {
    let spec = parser::parse(SIMPLE).expect("parse");
    let chart = resolve::resolve(&spec).expect("resolve");
    assert_eq!(chart.tasks.len(), 3);
    let t1 = &chart.tasks[0];
    assert_eq!(t1.start.unwrap().to_string(), "2025-01-06");
    assert_eq!(t1.percent, 40);
    let t2 = &chart.tasks[1];
    // T1 ends 2025-01-13 (start + 8 - 1); ref start = end + 1
    assert_eq!(t2.start.unwrap().to_string(), "2025-01-14");
    let m1 = &chart.tasks[2];
    assert!(m1.is_milestone);
    assert_eq!(m1.start.unwrap().to_string(), "2025-01-24");
    // Dependency arrow implied by `starts at [T1]'s end`
    assert_eq!(chart.dependencies.len(), 2);
}

#[test]
fn renders_svg() {
    let svg = render_source_to_svg(SIMPLE).expect("render");
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("Prototype design"));
    assert!(svg.contains("<path d=\"M ")); // milestone diamond path
    assert!(svg.contains("#DC143C")); // milestone color
}

#[test]
fn date_math_roundtrip() {
    use ganttcore::Date;
    for days in [-1000i64, -1, 0, 1, 100, 19_000] {
        let d = Date::from_epoch_days(days);
        assert_eq!(d.to_epoch_days(), days, "roundtrip for {days}");
    }
    let d = Date::new(2025, 1, 6); // a Monday
    assert_eq!(d.weekday(), 1);
    let sat = d.add_days(5);
    assert!(sat.is_weekend());
}

#[test]
fn cyclic_dependency_is_reported() {
    let src = r#"
project starts 2025-01-06
[A] starts at [B]'s end and lasts 2 days
[B] starts at [A]'s end and lasts 2 days
"#;
    let spec = parser::parse(src).expect("parse");
    let err = resolve::resolve(&spec).unwrap_err();
    assert!(err.to_string().contains("cyclic"), "got: {err}");
}

#[test]
fn skinparam_colors() {
    let src = r#"
@startgantt
project starts 2025-01-06
skinparam barColor Crimson
[T] Task starts 2025-01-06 and lasts 3 days
@endgantt
"#;
    let svg = render_source_to_svg(src).expect("render");
    assert!(svg.contains("#DC143C"), "crimson bar color expected");
}
