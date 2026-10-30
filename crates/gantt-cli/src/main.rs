//! gantt — render a PlantUML-like Gantt DSL file to SVG.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

fn print_usage() {
    eprintln!("gantt — thinkcell-style Gantt charts from a PlantUML-like DSL");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  gantt <input.gantt> [-o <output.svg>]");
    eprintln!("  gantt --examples                 List built-in examples from ggplot2 open data");
    eprintln!("  gantt --example <name> [--stdout]  Render a built-in example to SVG");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -o, --output <file>  Output SVG path (default: <input>.svg)");
    eprintln!("  -h, --help           Show this help");
    eprintln!("      --stdout         Write SVG to stdout");
}

fn print_examples() {
    eprintln!("Built-in examples (ggplot2 / R open datasets):");
    for ex in ganttcore::examples::all() {
        eprintln!("  {:<20} {} — {}", ex.name, ex.title, ex.description);
    }
    eprintln!();
    eprintln!("Render one with:  gantt --example <name> -o <name>.svg");
}

enum Mode {
    RenderFile { input: PathBuf, output: PathBuf, to_stdout: bool },
    ListExamples,
    RenderExample { name: String, to_stdout: bool },
}

fn parse_args() -> Result<Mode, String> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let to_stdout = argv.iter().any(|a| a == "--stdout");
    let args: Vec<String> = argv.into_iter().filter(|a| a != "--stdout").collect();
    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut example: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "--examples" => return Ok(Mode::ListExamples),
            "--example" => {
                i += 1;
                example = Some(args.get(i).ok_or("--example requires a name")?.clone());
            }
            "-o" | "--output" => {
                i += 1;
                output = Some(args.get(i).ok_or("-o requires a value")?.clone().into());
            }
            other if other.starts_with('-') => return Err(format!("unknown option `{other}`")),
            other => {
                if input.is_some() {
                    return Err(format!("unexpected extra argument `{other}`"));
                }
                input = Some(other.into());
            }
        }
        i += 1;
    }
    if let Some(name) = example {
        if input.is_some() {
            return Err("cannot combine --example with an input file".into());
        }
        return Ok(Mode::RenderExample { name, to_stdout });
    }
    let input = input.ok_or("missing input file (or use --examples / --example <name>)")?;
    let output = output.unwrap_or_else(|| {
        let mut p = input.clone().into_os_string();
        p.push(".svg");
        PathBuf::from(p)
    });
    Ok(Mode::RenderFile { input, output, to_stdout })
}

fn main() -> ExitCode {
    let mode = match parse_args() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {e}");
            print_usage();
            return ExitCode::from(2);
        }
    };

    match mode {
        Mode::ListExamples => {
            print_examples();
            ExitCode::SUCCESS
        }
        Mode::RenderExample { name, to_stdout } => {
            let Some(ex) = ganttcore::examples::get(&name) else {
                eprintln!("error: unknown example `{name}`");
                print_examples();
                return ExitCode::from(2);
            };
            match ganttcore::render_source_to_svg(&ex.dsl) {
                Ok(svg) => {
                    if to_stdout {
                        print!("{svg}");
                    } else {
                        let path = PathBuf::from(format!("{}.svg", ex.name));
                        if let Err(e) = std::fs::write(&path, &svg) {
                            eprintln!("error: cannot write {}: {e}", path.display());
                            return ExitCode::from(1);
                        }
                        eprintln!("wrote {} ({})", path.display(), ex.title);
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    ExitCode::from(1)
                }
            }
        }
        Mode::RenderFile { input, output, to_stdout } => {
            let src = match std::fs::read_to_string(&input) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: cannot read {}: {e}", input.display());
                    return ExitCode::from(1);
                }
            };

            match ganttcore::render_source_to_svg(&src) {
                Ok(svg) => {
                    if to_stdout {
                        print!("{svg}");
                        return ExitCode::SUCCESS;
                    }
                    let mut file = match std::fs::File::create(&output) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("error: cannot write {}: {e}", output.display());
                            return ExitCode::from(1);
                        }
                    };
                    if let Err(e) = file.write_all(svg.as_bytes()) {
                        eprintln!("error: write failed: {e}");
                        return ExitCode::from(1);
                    }
                    eprintln!("wrote {}", output.display());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    ExitCode::from(1)
                }
            }
        }
    }
}
