# GraphTool — Gantt

A Rust implementation of thinkcell-style Gantt charts, driven by a
PlantUML-like text DSL. One core library, three frontends:

- **CLI** (`gantt`): render a `.gantt` file to SVG on any OS.
- **WASM/Web**: the same core compiled to WebAssembly, usable in the browser
  via a small JS binding and an `index.html` demo.
- **Library** (`ganttcore`): parser + layout engine, reusable anywhere.

## Quick start (CLI)

```bash
cargo build --release
cargo run -p gantt-cli -- examples/simple.gantt -o out.svg
```

## DSL example

```text
@startgantt
projectscale daily
project starts 2025-01-06

skinparam barColor MidnightBlue
skinparam barFillHighlight DodgerBlue
skinparam milestoneColor Crimson

[T1] Prototype design starts 2025-01-06 and lasts 8 days
[T1] is 40% complete
[T2] Prototype build starts at [T1]'s end and lasts 10 days
[M1] First demo happens at [T2]'s end
@endgantt
```

## Browser demo

```bash
cd crates/gantt-wasm
cargo build --target wasm32-unknown-unknown --release
# serve index.html and the .wasm with any static server, e.g.:
python3 -m http.server
```

## Status / roadmap

- [x] PlantUML-like DSL (tasks, milestones, durations, dependencies, % complete)
- [x] thinkcell-style layout: scale axis, bars, milestones, today marker
- [x] Corporate-identity theming via `skinparam` (colors by name)
- [x] SVG output (OS independent)
- [x] WASM build for in-browser rendering
- [ ] Dependency arrow routing polish
- [ ] PDF/PNG export
