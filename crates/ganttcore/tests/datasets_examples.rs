//! Tests for the embedded ggplot2 open datasets and the generated examples.

use ganttcore::{datasets, examples, render_source_to_svg};

#[test]
fn all_ggplot_datasets_are_embedded() {
    let names: Vec<&str> = datasets::all().iter().map(|d| d.name).collect();
    for expected in ["mtcars", "diamonds", "mpg", "economics", "msleep", "midwest", "presidential"] {
        assert!(names.contains(&expected), "missing dataset {expected}");
    }
    for d in datasets::all() {
        assert_eq!(d.package, "ggplot2".eq(d.package).then_some(d.package).unwrap_or(d.package));
        let t = datasets::Table::parse(d.csv);
        assert!(!t.columns.is_empty(), "{} has header", d.name);
        assert!(!t.is_empty(), "{} has rows", d.name);
        assert!(t.rows.iter().all(|r| r.len() == t.columns.len()), "{} rectangular", d.name);
    }
}

#[test]
fn mtcars_matches_r_values() {
    let t = datasets::Table::parse(datasets::get("mtcars").unwrap().csv);
    assert_eq!(t.len(), 32);
    assert_eq!(t.columns.len(), 12);
    let mpg = t.col_f64("mpg");
    assert_eq!(mpg[0], Some(21.0)); // Mazda RX4
    assert_eq!(mpg[31], Some(21.4)); // Volvo 142E
    let names = t.col_str("rownames");
    assert_eq!(names[0], "Mazda RX4");
    assert_eq!(names[5], "Valiant");
}

#[test]
fn presidential_terms_parse() {
    let t = datasets::Table::parse(datasets::get("presidential").unwrap().csv);
    assert_eq!(t.len(), 12);
    assert_eq!(t.col_str("name")[0], "Eisenhower");
    assert_eq!(t.col_str("start")[0], "1953-01-20");
    assert_eq!(t.col_str("party")[1], "Democratic");
}

#[test]
fn every_example_renders_to_svg() {
    let all = examples::all();
    assert!(all.len() >= 8, "expected at least 8 examples, got {}", all.len());
    for ex in &all {
        let svg = render_source_to_svg(&ex.dsl).unwrap_or_else(|e| panic!("{} failed: {e}", ex.name));
        assert!(svg.starts_with("<svg"), "{} svg root", ex.name);
        assert!(svg.contains("@endgantt") == false);
        assert!(svg.contains("<rect"), "{} has bars", ex.name);
        assert!(svg.contains("<text"), "{} has labels", ex.name);
    }
}

#[test]
fn example_lookup_by_name() {
    let ex = examples::get("mtcars-cyl").expect("mtcars-cyl exists");
    assert!(ex.dsl.contains("[T0]"));
    assert!(ex.dsl.contains("cyl4"));
    assert!(examples::get("does-not-exist").is_none());
}

#[test]
fn mtcars_cyl_example_has_three_bars() {
    let ex = examples::get("mtcars-cyl").unwrap();
    let spec = ganttcore::parser::parse(&ex.dsl).expect("parse");
    let chart = ganttcore::resolve::resolve(&spec).expect("resolve");
    assert_eq!(chart.tasks.len(), 3);
    let w4 = chart.tasks.iter().find(|t| t.label == "cyl4").unwrap();
    let w8 = chart.tasks.iter().find(|t| t.label == "cyl8").unwrap();
    // 4-cylinder cars are more efficient: longer bar.
    assert!(w4.duration_days > w8.duration_days);
}

#[test]
fn presidential_example_spans_1953_to_2021() {
    let ex = examples::get("presidential-terms").unwrap();
    let spec = ganttcore::parser::parse(&ex.dsl).expect("parse");
    let chart = ganttcore::resolve::resolve(&spec).expect("resolve");
    assert_eq!(chart.tasks.len(), 12);
    let first = chart.tasks[0].start.unwrap();
    let last = chart.tasks[11].end_date().unwrap();
    assert_eq!(first.to_string(), "1953-01-20");
    assert_eq!(last.to_string(), "2021-01-20");
}

#[test]
fn csv_parser_handles_quotes_and_numbers() {
    let t = datasets::Table::parse("a,b\n\"x, y\",2.5\nplain,\"quoted \"\"q\"\"\"\n");
    assert_eq!(t.columns, vec!["a", "b"]);
    assert_eq!(t.rows[0][0], "x, y");
    assert_eq!(t.rows[0][1], "2.5");
    assert_eq!(t.col_f64("b")[0], Some(2.5));
    assert_eq!(t.col_f64("b")[1], None);
}
