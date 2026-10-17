//! gantt — render a PlantUML-like Gantt DSL file to SVG.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

struct Args {
    input: PathBuf,
    output: PathBuf,
}

fn print_usage() {
    eprintln!("gantt — thinkcell-style Gantt charts from a PlantUML-like DSL");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  gantt <input.gantt> [-o <output.svg>]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -o, --output <file>  Output SVG path (default: <input>.svg)");
    eprintln!("  -h, --help           Show this help");
    eprintln!("      --stdout         Write SVG to stdout");
}

fn parse_args() -> Result<Args, String> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    args.retain(|a| a != "--stdout");
    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
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
    let input = input.ok_or("missing input file")?;
    let output = output.unwrap_or_else(|| {
        let mut p = input.clone().into_os_string();
        p.push(".svg");
        PathBuf::from(p)
    });
    Ok(Args { input, output })
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            print_usage();
            return ExitCode::from(2);
        }
    };

    let src = match std::fs::read_to_string(&args.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {}: {e}", args.input.display());
            return ExitCode::from(1);
        }
    };

    match ganttcore::render_source_to_svg(&src) {
        Ok(svg) => {
            let mut file = match std::fs::File::create(&args.output) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error: cannot write {}: {e}", args.output.display());
                    return ExitCode::from(1);
                }
            };
            if let Err(e) = file.write_all(svg.as_bytes()) {
                eprintln!("error: write failed: {e}");
                return ExitCode::from(1);
            }
            eprintln!("wrote {}", args.output.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}
