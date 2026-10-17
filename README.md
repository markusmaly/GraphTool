# GraphTool — Business charting with thinkcell-style Gantt charts

A Rust application with a PowerPoint/deckcraft-style GUI: a horizontal page
canvas where thinkcell-like charts are placed as **vector graphics**, driven
by a PlantUML-like DSL. One codebase runs natively on **Windows, macOS,
Linux (Zorin/Ubuntu)** and in the **browser** (WebAssembly).

## Repository layout

| Crate | Purpose |
|---|---|
| `crates/ganttcore` | DSL parser, date resolver, layout engine, SVG writer (pure Rust, no deps) |
| `crates/gantt-cli` | CLI: render `.gantt` files to SVG |
| `crates/gantt-wasm` | Minimal WASM binding + browser demo page (no wasm-bindgen) |
| `crates/gt-app` | **GUI application** (egui) — ribbon, page canvas, live DSL editor |

## GUI (gt-app)

A deckcraft-style application:

- **Ribbon menus**: Home, Design, Tidying, Visualising
- **Blank horizontal page** (slide-like) where charts are placed
- Charts render as **vector graphics** via egui's GPU painter (no raster)
- Select, move, resize charts; live DSL editor and properties panel
- Save/Load projects (`.gtp` JSON)

### Run natively (Zorin/Ubuntu)

```bash
sudo apt install libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev  # build deps
cargo run --release -p gt-app
```

On Windows/macOS just `cargo run --release -p gt-app`.

### Run in the browser

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build crates/gt-app --target web --out-dir web/pkg
cd crates/gt-app/web
python3 -m http.server 8000
# open http://localhost:8000
```

### Ribbon reference

| Menu | Actions |
|---|---|
| **Home** | New chart, Open, Save, Delete, Zoom, toggle panels |
| **Design** | Corporate-identity palette (bar/milestone/grid/today colors), bar & row heights |
| **Tidying** | Align left/top, distribute horizontally |
| **Visualising** | Scale (day/week/month), today marker, weekend shading, bar label position |

## CLI (gantt)

```bash
cargo run --release -p gantt-cli -- examples/simple.gantt -o out.svg
```

## DSL example

```text
@startgantt
projectscale daily
project starts 2025-01-06
title Product Launch Plan
skinparam barColor MidnightBlue

[T1] Prototype design starts 2025-01-06 and lasts 8 days
[T1] is 60% complete
[T2] Prototype build starts at [T1]'s end and lasts 10 days
[M1] Launch happens at [T2]'s end
@endgantt
```

## Status / roadmap

- [x] PlantUML-like DSL (tasks, milestones, durations, dependencies, % complete)
- [x] thinkcell-style layout: scale axis, bars, milestones, today marker
- [x] Corporate-identity theming via `skinparam` and Design ribbon
- [x] SVG output (OS independent)
- [x] GUI: ribbon (Home/Design/Tidying/Visualising), page canvas, live editor
- [x] Native builds (Win/macOS/Linux) + WASM build
- [ ] More chart types (bar, line, waterfall — thinkcell portfolio)
- [ ] Export page as SVG/PDF
- [ ] Undo/redo
