//! Parser for the PlantUML-like Gantt DSL.

use crate::lexer::{tokenize, Tok, Token};
use crate::model::*;

pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}
impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

pub fn parse(src: &str) -> Result<Spec, ParseError> {
    let tokens = tokenize(src);
    let mut spec = Spec { directives: Vec::new(), theme: Theme::default(), title: None };
    let mut i = 0;

    // Optional @startgantt ... @endgantt wrapper.
    if let Some(t) = tokens.first() {
        if let Tok::Word(w) = &t.tok {
            if w == "@startgantt" {
                i = 1;
            }
        }
    }

    while i < tokens.len() {
        match &tokens[i].tok {
            Tok::Word(w) if w == "@endgantt" => break,
            Tok::Word(w) if w == "projectscale" || w == "printscale" => {
                i += 1;
                match tokens.get(i).map(|t| &t.tok) {
                    Some(Tok::Word(u)) => {
                        if let Some(unit) = Unit::from_str_opt(u) {
                            spec.directives.push(Directive::ProjectScale(unit));
                        } else {
                            return Err(ParseError { line: tokens[i].line, message: format!("unknown scale unit `{u}`") });
                        }
                        i += 1;
                    }
                    _ => {
                        return Err(ParseError { line: tokens[i - 1].line, message: "expected scale unit after projectscale".into() });
                    }
                }
            }
            Tok::Word(w) if w == "project" => {
                // `project starts 2025-01-06` (also allow `project starts 2025-01-06 ...` extras)
                let line = tokens[i].line;
                match (tokens.get(i + 1).map(|t| &t.tok), tokens.get(i + 2).map(|t| &t.tok)) {
                    (Some(Tok::Word(s)), Some(Tok::Date(d))) if s == "starts" => {
                        if let Some(date) = parse_date(d) {
                            spec.directives.push(Directive::ProjectStarts(date));
                            i += 3;
                            // consume anything else on the same line
                            while matches!(tokens.get(i).map(|t| (t.line, &t.tok)), Some((l, _)) if l == line) {
                                i += 1;
                            }
                        } else {
                            return Err(ParseError { line, message: format!("bad date `{d}`") });
                        }
                    }
                    _ => {
                        return Err(ParseError { line, message: "expected `starts <date>` after project".into() });
                    }
                }
            }
            Tok::Word(w) if w == "today" => {
                spec.directives.push(Directive::Today);
                i += 1;
            }
            Tok::Word(w) if w == "title" => {
                // title runs to end of line; reconstruct from tokens on same line
                let line = tokens[i].line;
                let start_idx = i + 1;
                let mut j = start_idx;
                while matches!(tokens.get(j).map(|t| t.line), Some(l) if l == line) {
                    j += 1;
                }
                let words: Vec<String> = tokens[start_idx..j]
                    .iter()
                    .map(|t| match &t.tok {
                        Tok::Word(w) => w.clone(),
                        Tok::Number(n) => format_number(*n),
                        Tok::Percent(p) => format!("{}%", p),
                        Tok::Date(d) => d.clone(),
                        Tok::LBracket => "[".into(),
                        Tok::RBracket => "]".into(),
                    })
                    .collect();
                spec.title = Some(words.join(" ")).filter(|s| !s.is_empty());
                i = j;
            }
            Tok::Word(w) if w == "skinparam" => {
                // skinparam name value  (value = rest of line)
                let line = tokens[i].line;
                let name_tok = tokens.get(i + 1);
                let name = match name_tok.map(|t| &t.tok) {
                    Some(Tok::Word(n)) => n.clone(),
                    _ => return Err(ParseError { line, message: "expected parameter name after skinparam".into() }),
                };
                let start_idx = i + 2;
                let mut j = start_idx;
                while matches!(tokens.get(j).map(|t| t.line), Some(l) if l == line) {
                    j += 1;
                }
                let words: Vec<String> = tokens[start_idx..j]
                    .iter()
                    .map(|t| match &t.tok {
                        Tok::Word(w) => w.clone(),
                        Tok::Number(n) => format_number(*n),
                        Tok::Percent(p) => format!("{}%", p),
                        Tok::Date(d) => d.clone(),
                        Tok::LBracket => "[".into(),
                        Tok::RBracket => "]".into(),
                    })
                    .collect();
                let value = words.join(" ");
                i = j;
                apply_skinparam(&mut spec, &name, &value)
                    .map_err(|m| ParseError { line, message: m })?;
            }
            Tok::LBracket => {
                // Task line: [Name] ... or [T1] is 40% complete, or [T2] starts at [T1]'s end...
                let parsed = parse_task_line(&tokens, &mut i)?;
                match parsed {
                    ParsedTaskLine::Task(ts) => spec.directives.push(Directive::Task(ts)),
                    ParsedTaskLine::Percent { target, percent } => {
                        spec.directives.push(Directive::PercentComplete { target, percent })
                    }
                }
            }
            Tok::Word(w) => {
                return Err(ParseError { line: tokens[i].line, message: format!("unexpected `{w}`") });
            }
            other => {
                return Err(ParseError { line: tokens[i].line, message: format!("unexpected token {other:?}") });
            }
        }
    }
    Ok(spec)
}

fn format_number(n: f64) -> String {
    if n.fract() == 0.0 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}

pub fn parse_date(s: &str) -> Option<Date> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year = parts[0].parse().ok()?;
    let month = parts[1].parse().ok()?;
    let day = parts[2].parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some(Date::new(year, month, day))
}

enum ParsedTaskLine {
    Task(TaskSpec),
    Percent { target: String, percent: u8 },
}

fn parse_task_line(tokens: &[Token], i: &mut usize) -> Result<ParsedTaskLine, ParseError> {
    let start_line = tokens[*i].line;
    // Collect all tokens on this line.
    let line = start_line;
    let start_idx = *i;
    let mut j = start_idx;
    while matches!(tokens.get(j).map(|t| t.line), Some(l) if l == line) {
        j += 1;
    }
    let toks = &tokens[start_idx..j];

    // Expect [Name]
    if !matches!(toks.first().map(|t| &t.tok), Some(Tok::LBracket)) {
        return Err(ParseError { line, message: "expected `[`".into() });
    }
    let name = match toks.get(1).map(|t| &t.tok) {
        Some(Tok::Word(w)) => w.clone(),
        _ => return Err(ParseError { line, message: "expected task name after `[`".into() }),
    };
    if !matches!(toks.get(2).map(|t| &t.tok), Some(Tok::RBracket)) {
        return Err(ParseError { line, message: "expected `]` after task name".into() });
    }

    // Leading display-label tokens until a recognized keyword. Numbers and
    // percents are allowed inside labels (e.g. `Hornet 4 Drive`).
    const KEYWORDS: [&str; 7] = ["is", "starts", "happens", "lasts", "ends", "on", "as"];
    let mut k = 3;
    let mut label_words: Vec<String> = Vec::new();
    while k < toks.len() {
        match &toks[k].tok {
            Tok::Word(w) => {
                if KEYWORDS.contains(&w.as_str()) {
                    break;
                }
                label_words.push(w.clone());
                k += 1;
            }
            Tok::Number(n) => {
                label_words.push(format_number(*n));
                k += 1;
            }
            Tok::Percent(p) => {
                label_words.push(format!("{}%", p));
                k += 1;
            }
            _ => break,
        }
    }
    let inline_label = if label_words.is_empty() { None } else { Some(label_words.join(" ")) };
    let rest = &toks[k..];
    *i = j;

    // Forms:
    //  [T] is 40% complete
    //  [T] lasts 5 days
    //  [T] starts 2025-01-10
    //  [T] starts at [X]'s end
    //  [T] starts at [X]'s end and lasts 5 days
    //  [T] starts 2025-01-10 and lasts 5 days
    //  [T] ends 2025-01-20
    //  [T] happens 2025-01-15  (milestone)
    //  [T] happens at [X]'s end (milestone)
    //  [T] happens 2025-01-15 and lasts 0 days
    //  [T] task display label (label form: `[T] label` -> custom label)
    if rest.is_empty() {
        // Label-only task line: keep as task with unspecified start.
        return Ok(ParsedTaskLine::Task(TaskSpec {
            name,
            label: None,
            start_spec: StartSpec::Unspecified,
            duration: None,
            duration_unit: Unit::Day,
            end_spec: None,
            percent: None,
            is_milestone: false,
            color: None,
            label_pos: None,
        }));
    }

    match &rest[0].tok {
        Tok::Word(w) if w == "is" => {
            // [T] is 40% complete
            if rest.len() >= 3 {
                if let Tok::Percent(p) = rest[1].tok {
                    let percent = p.clamp(0.0, 100.0) as u8;
                    return Ok(ParsedTaskLine::Percent { target: name, percent });
                }
            }
            Err(ParseError { line, message: "expected `<n>% complete` after `is`".into() })
        }
        Tok::Word(w) if w == "starts" => {
            let (start_spec, used) = parse_start(rest, 1, line)?;
            let mut duration = None;
            let mut duration_unit = Unit::Day;
            let mut label = None;
            let mut color = None;
            // Optional "and lasts N days"
            if let Some(pos) = find_word(rest, used, "and") {
                if matches!(rest.get(pos + 1).map(|t| &t.tok), Some(Tok::Word(w)) if w == "lasts") {
                    let (d, u, after) = parse_duration(&rest[pos + 2..], line)?;
                    duration = Some(d);
                    duration_unit = u;
                    // further modifiers after duration
                    if let Some(extra) = parse_task_modifiers(&rest[after..], line) {
                        label = extra.label;
                        color = extra.color;
                    }
                }
            } else if let Some(extra) = parse_task_modifiers(&rest[used..], line) {
                label = extra.label;
                color = extra.color;
            }
            Ok(ParsedTaskLine::Task(TaskSpec {
                name,
                label: label.or(inline_label.clone()),
                start_spec,
                duration,
                duration_unit,
                end_spec: None,
                percent: None,
                is_milestone: false,
                color,
                label_pos: None,
            }))
        }
        Tok::Word(w) if w == "happens" => {
            // Milestone
            let (start_spec, used) = parse_start(rest, 1, line)?;
            let mut label = None;
            let mut color = None;
            if let Some(extra) = parse_task_modifiers(&rest[used..], line) {
                label = extra.label;
                color = extra.color;
            }
            Ok(ParsedTaskLine::Task(TaskSpec {
                name,
                label: label.or(inline_label.clone()),
                start_spec,
                duration: Some(0),
                duration_unit: Unit::Day,
                end_spec: None,
                percent: None,
                is_milestone: true,
                color,
                label_pos: None,
            }))
        }
        Tok::Word(w) if w == "lasts" => {
            let (d, u, after) = parse_duration(rest, 1)?;
            let mut label = None;
            let mut color = None;
            if let Some(extra) = parse_task_modifiers(&rest[after..], line) {
                label = extra.label;
                color = extra.color;
            }
            Ok(ParsedTaskLine::Task(TaskSpec {
                name,
                label: label.or(inline_label.clone()),
                start_spec: StartSpec::Unspecified,
                duration: Some(d),
                duration_unit: u,
                end_spec: None,
                percent: None,
                is_milestone: false,
                color,
                label_pos: None,
            }))
        }
        Tok::Word(w) if w == "ends" => {
            let end_spec = match rest.get(1).map(|t| &t.tok) {
                Some(Tok::Date(d)) => EndSpec::Fixed(parse_date(d).ok_or(ParseError { line, message: format!("bad date `{d}`") })?),
                Some(Tok::LBracket) => {
                    let (refname, offset) = parse_ref(&rest[1..], line)?;
                    EndSpec::Ref { name: refname, offset_days: offset }
                }
                _ => return Err(ParseError { line, message: "expected date or reference after `ends`".into() }),
            };
            let mut label = None;
            let mut color = None;
            if let Some(extra) = parse_task_modifiers(&rest[3..], line) {
                label = extra.label;
                color = extra.color;
            }
            Ok(ParsedTaskLine::Task(TaskSpec {
                name,
                label: label.or(inline_label.clone()),
                start_spec: StartSpec::Unspecified,
                duration: None,
                duration_unit: Unit::Day,
                end_spec: Some(end_spec),
                percent: None,
                is_milestone: false,
                color,
                label_pos: None,
            }))
        }
        Tok::Word(w) if w == "on" => {
            // [T] on 2025-01-15 (milestone shorthand)
            let date = match rest.get(1).map(|t| &t.tok) {
                Some(Tok::Date(d)) => parse_date(d).ok_or(ParseError { line, message: format!("bad date `{d}`") })?,
                _ => return Err(ParseError { line, message: "expected date after `on`".into() }),
            };
            Ok(ParsedTaskLine::Task(TaskSpec {
                name,
                label: inline_label.clone(),
                start_spec: StartSpec::Fixed(date),
                duration: Some(0),
                duration_unit: Unit::Day,
                end_spec: None,
                percent: None,
                is_milestone: true,
                color: None,
                label_pos: None,
            }))
        }
        _ => Err(ParseError { line, message: format!("unexpected token after task name: {rest:?}") }),
    }
}

struct Extras {
    label: Option<String>,
    color: Option<Color>,
    label_pos: Option<BarLabelPos>,
}

/// Parses trailing task modifiers: `[as <label>]`, `[in <color>]`,
/// `[label left|right|center|hidden]`.
fn parse_task_modifiers(toks: &[Token], _line: usize) -> Option<Extras> {
    if toks.is_empty() {
        return None;
    }
    let mut extras = Extras { label: None, color: None, label_pos: None };
    let mut any = false;
    let mut i = 0;
    while i < toks.len() {
        match &toks[i].tok {
            Tok::Word(w) if w == "as" => {
                if let Some(Tok::Word(l)) = toks.get(i + 1).map(|t| &t.tok) {
                    extras.label = Some(l.clone());
                    any = true;
                    i += 2;
                } else {
                    return None;
                }
            }
            Tok::Word(w) if w == "in" => {
                match toks.get(i + 1).map(|t| &t.tok) {
                    Some(Tok::Word(c)) => {
                        if let Some(color) = color_by_name(c) {
                            extras.color = Some(color);
                            any = true;
                        }
                        i += 2;
                    }
                    _ => return None,
                }
            }
            Tok::Word(w) if w == "label" => {
                match toks.get(i + 1).map(|t| &t.tok) {
                    Some(Tok::Word(p)) => {
                        let lp = match p.as_str() {
                            "left" => Some(BarLabelPos::Left),
                            "right" => Some(BarLabelPos::Right),
                            "center" => Some(BarLabelPos::Center),
                            "hidden" => Some(BarLabelPos::Hidden),
                            _ => None,
                        };
                        if let Some(lp) = lp {
                            extras.label_pos = Some(lp);
                            any = true;
                        }
                        i += 2;
                    }
                    _ => return None,
                }
            }
            _ => return None,
        }
    }
    if any { Some(extras) } else { None }
}

fn find_word(toks: &[Token], from: usize, word: &str) -> Option<usize> {
    toks.iter().enumerate().skip(from).find(|(_, t)| matches!(&t.tok, Tok::Word(w) if w == word)).map(|(i, _)| i)
}

/// Parses `at [X]'s end` / `[X]'s start` or a bare date. Returns spec and
/// how many tokens were consumed (index after the start expression).
fn parse_start(rest: &[Token], mut from: usize, line: usize) -> Result<(StartSpec, usize), ParseError> {
    // Skip optional `at`
    if let Some(Tok::Word(w)) = rest.get(from).map(|t| &t.tok) {
        if w == "at" {
            from += 1;
        }
    }
    if let Some(t) = rest.get(from) {
        match &t.tok {
            Tok::Date(d) => {
                let date = parse_date(d).ok_or(ParseError { line, message: format!("bad date `{d}`") })?;
                Ok((StartSpec::Fixed(date), from + 1))
            }
            Tok::LBracket => {
                let (name, _end) = parse_ref(&rest[from..], line)?;
                // [X]'s end / [X]'s start / bare [X]
                let mut at_end = true;
                let mut used = from + 3; // [ X ] 's
                if let Some(Tok::Word(w)) = rest.get(from + 3).map(|t| &t.tok) {
                    if w == "end" {
                        at_end = true;
                        used += 1;
                    } else if w == "start" {
                        at_end = false;
                        used += 1;
                    }
                }
                Ok((StartSpec::Ref { name, offset_days: 0, at_end }, used))
            }
            _ => Err(ParseError { line, message: "expected date or `[ref]` after `starts`".into() }),
        }
    } else {
        Err(ParseError { line, message: "expected date or reference after `starts`".into() })
    }
}

/// Parses `[Name]'s` returning (name, tokens consumed including `[`, `]`).
fn parse_ref(toks: &[Token], line: usize) -> Result<(String, i64), ParseError> {
    if !matches!(toks.first().map(|t| &t.tok), Some(Tok::LBracket)) {
        return Err(ParseError { line, message: "expected `[`".into() });
    }
    let name = match toks.get(1).map(|t| &t.tok) {
        Some(Tok::Word(w)) => w.clone(),
        _ => return Err(ParseError { line, message: "expected name inside brackets".into() }),
    };
    if !matches!(toks.get(2).map(|t| &t.tok), Some(Tok::RBracket)) {
        return Err(ParseError { line, message: "expected `]`".into() });
    }
    // Optional `'s` is part of the word in `[X]'s end`? The lexer will produce
    // `[X]` `'s` as separate tokens only if whitespace-separated; handle both.
    Ok((name, 0))
}

fn parse_duration(toks: &[Token], _line: usize) -> Result<(u64, Unit, usize), ParseError> {
    let n = match toks.first().map(|t| &t.tok) {
        Some(Tok::Number(n)) => *n as u64,
        _ => return Err(ParseError { line: toks.first().map(|t| t.line).unwrap_or(0), message: "expected duration number".into() }),
    };
    let unit = match toks.get(1).map(|t| &t.tok) {
        Some(Tok::Word(u)) => Unit::from_str_opt(u).ok_or(ParseError {
            line: toks[1].line,
            message: format!("unknown duration unit `{u}`"),
        })?,
        _ => Unit::Day,
    };
    Ok((n, unit, 2))
}

fn apply_skinparam(spec: &mut Spec, name: &str, value: &str) -> Result<(), String> {
    let t = &mut spec.theme;
    match name.to_ascii_lowercase().as_str() {
        "backgroundcolor" | "background" => t.background = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "barcolor" => t.bar_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "barhighlightcolor" | "barfillhighlight" => t.bar_highlight_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "milestonecolor" => t.milestone_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "labelcolor" => t.label_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "axiscolor" => t.axis_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "scalecolor" => t.scale_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "gridcolor" => t.grid_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "todaycolor" => t.today_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "dependencycolor" => t.dependency_color = color_by_name(value).ok_or(format!("unknown color `{value}`"))?,
        "fontfamily" => t.font_family = value.to_string(),
        "barheight" => t.bar_height = value.parse().map_err(|_| format!("bad number `{value}`"))?,
        "rowheight" => t.row_height = value.parse().map_err(|_| format!("bad number `{value}`"))?,
        "barradius" => t.bar_border_radius = value.parse().map_err(|_| format!("bad number `{value}`"))?,
        "barlabelpos" => {
            t.bar_label_pos = match value.to_ascii_lowercase().as_str() {
                "left" => BarLabelPos::Left,
                "right" => BarLabelPos::Right,
                "center" => BarLabelPos::Center,
                "hidden" => BarLabelPos::Hidden,
                _ => return Err(format!("bad barLabelPos `{value}`")),
            }
        }
        "showtoday" => t.show_today = value.eq_ignore_ascii_case("true") || value == "1",
        "weekendshading" => t.weekend_shading = value.eq_ignore_ascii_case("true") || value == "1",
        _ => return Err(format!("unknown skinparam `{name}`")),
    }
    Ok(())
}
