//! Resolves a parsed `Spec` into a `GanttChart` with concrete dates.

use crate::model::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum ResolveError {
    UnknownTask(String),
    CyclicDependency(String),
    NoProjectStart,
    BadPercent(String),
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveError::UnknownTask(n) => write!(f, "unknown task `{n}`"),
            ResolveError::CyclicDependency(n) => write!(f, "cyclic dependency involving `{n}`"),
            ResolveError::NoProjectStart => write!(f, "no project start date given (`project starts <date>`)"),
            ResolveError::BadPercent(s) => write!(f, "bad percent `{s}`"),
        }
    }
}

pub fn resolve(spec: &Spec) -> Result<GanttChart, ResolveError> {
    let mut scale = Scale::default();
    let mut tasks: Vec<Task> = Vec::new();
    let mut index: BTreeMap<String, usize> = BTreeMap::new();
    let mut dependencies: Vec<Dependency> = Vec::new();
    let mut today: Option<Date> = None;

    // First pass: create tasks, capture scale/percent/deps.
    for d in &spec.directives {
        match d {
            Directive::ProjectScale(u) => scale.unit = *u,
            Directive::ProjectStarts(date) => scale.start = Some(*date),
            Directive::Today => today = Some(Date::new(1970, 1, 1)), // placeholder; CLI/wasm may override
            Directive::Task(ts) => {
                let idx = tasks.len();
                let mut t = Task::new(ts.name.clone());
                t.label = ts.label.clone().unwrap_or_else(|| ts.name.clone());
                t.is_milestone = ts.is_milestone;
                t.color = ts.color;
                t.label_pos = ts.label_pos;
                t.percent = ts.percent.unwrap_or(0);
                tasks.push(t);
                index.insert(ts.name.clone(), idx);
            }
            Directive::PercentComplete { target, percent } => {
                let idx = *index
                    .get(target)
                    .ok_or_else(|| ResolveError::UnknownTask(target.clone()))?;
                tasks[idx].percent = *percent;
            }
            Directive::Dependency { from, to } => {
                dependencies.push(Dependency { from: from.clone(), to: to.clone() });
            }
            Directive::Title(_) => {}
        }
    }

    if tasks.is_empty() {
        return Ok(GanttChart {
            scale,
            theme: spec.theme.clone(),
            tasks,
            dependencies,
            today,
            title: spec.title.clone(),
        });
    }

    // Resolve dates iteratively (supports chains like [T2] at [T1]'s end).
    // Simple worklist: resolve what we can, repeat; detect cycles.
    struct Raw {
        start_spec: StartSpec,
        duration: Option<u64>,
        duration_unit: Unit,
        end_spec: Option<EndSpec>,
    }
    let mut raws: Vec<Raw> = Vec::new();
    for d in &spec.directives {
        if let Directive::Task(ts) = d {
            raws.push(Raw {
                start_spec: ts.start_spec.clone(),
                duration: ts.duration,
                duration_unit: ts.duration_unit,
                end_spec: ts.end_spec.clone(),
            });
            // `starts at [X]'s end` implies a dependency arrow X -> this task.
            if let StartSpec::Ref { name, .. } = &ts.start_spec {
                if index.contains_key(name) {
                    dependencies.push(Dependency { from: name.clone(), to: ts.name.clone() });
                }
            }
        }
    }

    let default_start = scale.start.ok_or(ResolveError::NoProjectStart)?;

    // end-of-task map used by refs, recomputed after each pass
    for _pass in 0..=tasks.len() {
        let mut progress = false;
        for i in 0..tasks.len() {
            if tasks[i].start.is_none() {
                let raw = &raws[i];
                let resolved: Option<Date> = match &raw.start_spec {
                    StartSpec::Fixed(d) => Some(*d),
                    StartSpec::Unspecified => Some(default_start),
                    StartSpec::Ref { name, offset_days, at_end } => {
                        let ref_idx = index
                            .get(name)
                            .ok_or_else(|| ResolveError::UnknownTask(name.clone()))?;
                        match (at_end, tasks[*ref_idx].end_date()) {
                            (true, Some(e)) => Some(e.add_days(*offset_days + 1)),
                            (false, Some(s)) => Some(s.add_days(*offset_days)),
                            _ => None,
                        }
                    }
                };
                if let Some(start) = resolved {
                    let raw = &raws[i];
                    tasks[i].start = Some(start);
                    tasks[i].duration_days = raw.duration.unwrap_or(0);
                    // normalize duration to days
                    match raw.duration_unit {
                        Unit::Day => {}
                        Unit::Week => tasks[i].duration_days *= 7,
                        Unit::Month => tasks[i].duration_days *= 30,
                    }
                    if let Some(end_spec) = &raw.end_spec {
                        let end = match end_spec {
                            EndSpec::Fixed(d) => Some(*d),
                            EndSpec::Ref { name, offset_days } => {
                                let ref_idx = index
                                    .get(name)
                                    .ok_or_else(|| ResolveError::UnknownTask(name.clone()))?;
                                tasks[*ref_idx].end_date().map(|e| e.add_days(*offset_days))
                            }
                        };
                        if end.is_some() {
                            tasks[i].end = end;
                        } else {
                            // ref not resolved yet: undo start, retry next pass
                            tasks[i].start = None;
                            continue;
                        }
                    }
                    progress = true;
                }
            }
        }
        if tasks.iter().all(|t| t.start.is_some()) {
            break;
        }
        if !progress {
            let stuck: Vec<&str> = tasks
                .iter()
                .filter(|t| t.start.is_none())
                .map(|t| t.name.as_str())
                .collect();
            return Err(ResolveError::CyclicDependency(stuck.join(", ")));
        }
    }

    // Validate dependency endpoints exist.
    for dep in &dependencies {
        if !index.contains_key(&dep.from) {
            return Err(ResolveError::UnknownTask(dep.from.clone()));
        }
        if !index.contains_key(&dep.to) {
            return Err(ResolveError::UnknownTask(dep.to.clone()));
        }
    }

    // Rows: one per task, in declaration order (like thinkcell's default).
    for (i, t) in tasks.iter_mut().enumerate() {
        t.row = i;
    }

    Ok(GanttChart {
        scale,
        theme: spec.theme.clone(),
        tasks,
        dependencies,
        today,
        title: spec.title.clone(),
    })
}
