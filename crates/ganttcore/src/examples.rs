//! Example charts generated from the embedded ggplot2 open datasets
//! (mtcars, diamonds, mpg, economics, msleep, midwest, presidential).
//!
//! Each example is a ready-to-render DSL document, so it works identically
//! in the CLI, the WASM demo and the egui GUI.

use crate::datasets::{self, Table};

/// One ready-made example chart.
#[derive(Debug, Clone)]
pub struct Example {
    /// Slug, e.g. `"mtcars-cyl"`.
    pub name: &'static str,
    /// Human-readable title (also used as chart title and menu label).
    pub title: &'static str,
    /// Short description of the underlying dataset slice.
    pub description: &'static str,
    /// The DSL source.
    pub dsl: String,
}

/// All built-in examples.
pub fn all() -> Vec<Example> {
    vec![
        mtcars_by_cyl(),
        mtcars_models(),
        mpg_manufacturers(),
        diamonds_by_cut(),
        msleep_by_vore(),
        midwest_poverty(),
        presidential_terms(),
        economics_unemployment(),
    ]
}

/// Look up an example by slug.
pub fn get(name: &str) -> Option<Example> {
    all().into_iter().find(|e| e.name == name)
}

const PALETTE: [&str; 8] = [
    "DodgerBlue", "Teal", "Crimson", "Orange", "Purple", "Green", "Amber", "Azure",
];

fn header(title: &str, start: &str, scale: &str) -> String {
    format!(
        "@startgantt\nprojectscale {scale}\nproject starts {start}\ntitle {title}\n\nskinparam barColor MidnightBlue\nskinparam barHighlightColor DodgerBlue\nskinparam milestoneColor Crimson\n\n"
    )
}

/// Keep only tokens the DSL lexer treats as words (drops pure numbers,
/// percents and dates) so the result is a valid inline task label.
fn label(s: &str) -> String {
    let words: Vec<&str> = s
        .split_whitespace()
        .filter(|t| {
            t.parse::<f64>().is_err() && !t.ends_with('%') && !is_date_token(t)
        })
        .collect();
    if words.is_empty() {
        "Row".to_string()
    } else {
        words.join(" ")
    }
}

fn is_date_token(t: &str) -> bool {
    t.len() == 10
        && t.as_bytes()[4] == b'-'
        && t.as_bytes()[7] == b'-'
        && t[..4].chars().all(|c| c.is_ascii_digit())
        && t[5..7].chars().all(|c| c.is_ascii_digit())
        && t[8..].chars().all(|c| c.is_ascii_digit())
}

fn table(name: &str) -> Table {
    Table::parse(datasets::get(name).expect("dataset exists").csv)
}

fn avg_by_group(keys: &[&str], values: &[Option<f64>]) -> Vec<(String, f64, usize)> {
    let mut order: Vec<String> = Vec::new();
    let mut sum: Vec<f64> = Vec::new();
    let mut n: Vec<usize> = Vec::new();
    for (k, v) in keys.iter().zip(values.iter()) {
        let Some(v) = v else { continue };
        match order.iter().position(|o| o == k) {
            Some(i) => {
                sum[i] += v;
                n[i] += 1;
            }
            None => {
                order.push(k.to_string());
                sum.push(*v);
                n.push(1);
            }
        }
    }
    order
        .into_iter()
        .zip(sum.iter().zip(n.iter()))
        .map(|(k, (s, n))| (k, s / *n as f64, *n))
        .collect()
}

fn push_bar(out: &mut String, idx: usize, label: &str, start: &str, days: u64, percent: u8, color: &str) {
    let pct = percent.min(100);
    out.push_str(&format!(
        "[T{idx}] {label} starts {start} and lasts {days} days in {color}\n[T{idx}] is {pct}% complete\n"
    ));
}

/// mtcars: average mpg per cylinder count (the classic ggplot bar chart).
fn mtcars_by_cyl() -> Example {
    let t = table("mtcars");
    let groups = avg_by_group(&t.col_str("cyl"), &t.col_f64("mpg"));
    let mut groups = groups;
    groups.sort_by(|a, b| a.0.cmp(&b.0));
    let max = groups.iter().map(|g| g.1).fold(0.0f64, f64::max);
    let start = "1973-01-01";
    let mut dsl = header("mtcars avg mpg by cylinders", start, "daily");
    for (i, (cyl, avg, _n)) in groups.iter().enumerate() {
        let days = (*avg).round().max(1.0) as u64;
        let pct = ((*avg / max) * 100.0).round() as u8;
        push_bar(&mut dsl, i, &format!("cyl{cyl}"), start, days, pct, PALETTE[i % PALETTE.len()]);
    }
    dsl.push_str("@endgantt\n");
    Example {
        name: "mtcars-cyl",
        title: "mtcars: avg mpg by cylinders",
        description: "Mean miles per gallon grouped by cylinder count (4/6/8) from R's mtcars.",
        dsl,
    }
}

/// mtcars: every car model as a bar, length = mpg.
fn mtcars_models() -> Example {
    let t = table("mtcars");
    let names = t.col_str("rownames");
    let mpg = t.col_f64("mpg");
    let mut cars: Vec<(&str, f64)> = names
        .iter()
        .zip(mpg.iter())
        .filter_map(|(n, m)| m.map(|m| (*n, m)))
        .collect();
    cars.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let max = cars.iter().map(|c| c.1).fold(0.0f64, f64::max);
    let start = "1973-01-01";
    let mut dsl = header("mtcars mpg by car model 1973", start, "daily");
    for (i, (name, m)) in cars.iter().enumerate() {
        let days = (*m).round().max(1.0) as u64;
        let pct = ((*m / max) * 100.0).round() as u8;
        push_bar(&mut dsl, i, &label(name), start, days, pct, PALETTE[i % PALETTE.len()]);
    }
    dsl.push_str("@endgantt\n");
    Example {
        name: "mtcars-models",
        title: "mtcars: mpg by car model",
        description: "All 32 cars of R's mtcars, bar length = miles per gallon.",
        dsl,
    }
}

/// mpg: average highway mpg per manufacturer.
fn mpg_manufacturers() -> Example {
    let t = table("mpg");
    let groups = avg_by_group(&t.col_str("manufacturer"), &t.col_f64("hwy"));
    let mut groups = groups;
    groups.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let max = groups.iter().map(|g| g.1).fold(0.0f64, f64::max);
    let start = "1999-01-01";
    let mut dsl = header("mpg avg highway mpg by manufacturer", start, "daily");
    for (i, (mfr, avg, n)) in groups.iter().enumerate() {
        let days = (*avg).round().max(1.0) as u64;
        let pct = ((*avg / max) * 100.0).round() as u8;
        let lab = label(&format!("{mfr} n{n}"));
        push_bar(&mut dsl, i, &lab, start, days, pct, PALETTE[i % PALETTE.len()]);
    }
    dsl.push_str("@endgantt\n");
    Example {
        name: "mpg-manufacturers",
        title: "mpg: highway mpg by manufacturer",
        description: "Average highway mpg per manufacturer from ggplot2's mpg (234 cars, 1999-2008).",
        dsl,
    }
}

/// diamonds: average price per cut grade.
fn diamonds_by_cut() -> Example {
    let t = table("diamonds");
    let cuts = ["Fair", "Good", "Premium", "Very Good", "Ideal"];
    let col = t.col_str("cut");
    let price = t.col_f64("price");
    let mut groups = avg_by_group(&col, &price);
    groups.retain(|g| cuts.contains(&g.0.as_str()));
    groups.sort_by_key(|g| cuts.iter().position(|c| *c == g.0).unwrap_or(99));
    let max = groups.iter().map(|g| g.1).fold(0.0f64, f64::max);
    let start = "2008-01-01";
    let mut dsl = header("diamonds avg price by cut USD", start, "daily");
    for (i, (cut, avg, _n)) in groups.iter().enumerate() {
        let days = (*avg / 250.0).round().max(1.0) as u64;
        let pct = ((*avg / max) * 100.0).round() as u8;
        push_bar(&mut dsl, i, &label(cut), start, days, pct, PALETTE[i % PALETTE.len()]);
    }
    dsl.push_str("@endgantt\n");
    Example {
        name: "diamonds-cut",
        title: "diamonds: avg price by cut",
        description: "Mean diamond price per cut grade from ggplot2's diamonds (sample embedded).",
        dsl,
    }
}

/// msleep: average total sleep per diet (vore).
fn msleep_by_vore() -> Example {
    let t = table("msleep");
    let vore = t.col_str("vore");
    let sleep = t.col_f64("sleep_total");
    let mut groups = avg_by_group(&vore, &sleep);
    groups.retain(|g| !g.0.is_empty());
    groups.sort_by(|a, b| a.0.cmp(&b.0));
    let max = groups.iter().map(|g| g.1).fold(0.0f64, f64::max);
    let start = "2000-01-01";
    let mut dsl = header("msleep avg sleep hours by diet", start, "daily");
    for (i, (v, avg, _n)) in groups.iter().enumerate() {
        let days = (*avg * 2.0).round().max(1.0) as u64;
        let pct = ((*avg / max) * 100.0).round() as u8;
        push_bar(&mut dsl, i, &label(v), start, days, pct, PALETTE[i % PALETTE.len()]);
    }
    dsl.push_str("@endgantt\n");
    Example {
        name: "msleep-vore",
        title: "msleep: avg sleep by diet",
        description: "Mean daily sleep (hours) grouped by diet from ggplot2's msleep (83 mammals).",
        dsl,
    }
}

/// midwest: percent below poverty for the 12 hardest-hit counties.
fn midwest_poverty() -> Example {
    let t = table("midwest");
    let county = t.col_str("county");
    let state = t.col_str("state");
    let pov = t.col_f64("percbelowpoverty");
    let mut rows: Vec<(&str, &str, f64)> = county
        .iter()
        .zip(state.iter())
        .zip(pov.iter())
        .filter_map(|((c, s), p)| p.map(|p| (*c, *s, p)))
        .collect();
    rows.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    let start = "2000-01-01";
    let mut dsl = header("midwest perc below poverty top counties", start, "daily");
    for (i, (c, s, p)) in rows.iter().enumerate() {
        let days = (*p).round().max(1.0) as u64;
        let pct = ((*p) * 2.0).round().min(100.0) as u8;
        push_bar(&mut dsl, i, &label(&format!("{c} {s}")), start, days, pct, PALETTE[i % PALETTE.len()]);
    }
    dsl.push_str("@endgantt\n");
    Example {
        name: "midwest-poverty",
        title: "midwest: poverty by county",
        description: "Percent below poverty for the 12 highest-poverty counties in ggplot2's midwest.",
        dsl,
    }
}

/// presidential: one bar per US president, 1953-2021.
fn presidential_terms() -> Example {
    let t = table("presidential");
    let names = t.col_str("name");
    let starts = t.col_str("start");
    let ends = t.col_str("end");
    let parties = t.col_str("party");
    let start0 = starts.first().copied().unwrap_or("1953-01-20");
    let mut dsl = header("US presidents 1953 2021 by term", start0, "daily");
    for i in 0..t.len() {
        let s = crate::parser::parse_date(starts[i]).expect("valid start date");
        let e = crate::parser::parse_date(ends[i]).expect("valid end date");
        let days = (e.to_epoch_days() - s.to_epoch_days() + 1).max(1) as u64;
        let color = if parties[i] == "Republican" { "Crimson" } else { "DodgerBlue" };
        push_bar(&mut dsl, i, &label(names[i]), &s.to_string(), days, 100, color);
    }
    dsl.push_str("@endgantt\n");
    Example {
        name: "presidential-terms",
        title: "presidential: US terms 1953-2021",
        description: "Each US presidential term 1953-2021 as a bar, colored by party (ggplot2 presidential).",
        dsl,
    }
}

/// economics: biennial snapshots of personal savings rate vs unemployment.
fn economics_unemployment() -> Example {
    let t = table("economics");
    let dates = t.col_str("date");
    let psavert = t.col_f64("psavert");
    let start0 = dates.first().copied().unwrap_or("1967-07-01");
    let mut dsl = header("economics savings rate 1967 2013", start0, "monthly");
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    for i in 0..t.len() {
        let d = crate::parser::parse_date(dates[i]).expect("valid date");
        let pct = (psavert[i].unwrap_or(0.0) * 10.0).round().clamp(0.0, 100.0) as u8;
        let lab = format!("{}{}", months[(d.month - 1) as usize], d.year % 100);
        dsl.push_str(&format!(
            "[T{i}] {lab} starts {} and lasts 30 days in {}\n[T{i}] is {pct}% complete\n",
            d,
            PALETTE[i % PALETTE.len()]
        ));
    }
    dsl.push_str("@endgantt\n");
    Example {
        name: "economics-savings",
        title: "economics: savings rate over time",
        description: "Biennial snapshots of the US personal savings rate, 1967-2013 (ggplot2 economics).",
        dsl,
    }
}
