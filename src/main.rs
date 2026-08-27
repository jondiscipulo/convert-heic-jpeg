use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use clap::Parser;

use convert_heic_jpeg::{ConvertError, convert_to_jpeg, default_output_path};

#[derive(Parser)]
#[command(name = "convert-heic-jpeg", about = "Convert HEIC/HEIF images to JPEG")]
struct Cli {
    /// Input .heic/.heif file(s) or directories containing them
    #[arg(required = true)]
    inputs: Vec<PathBuf>,

    /// Directory to write output .jpg files (defaults to each input's folder)
    #[arg(short, long)]
    output_dir: Option<PathBuf>,

    /// JPEG quality (1-100)
    #[arg(short, long, default_value_t = 100, value_parser = clap::value_parser!(u8).range(1..=100))]
    quality: u8,

    /// Overwrite existing output files
    #[arg(short, long)]
    force: bool,
}

fn main() {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(converted) if converted > 0 => {
            println!("converted {converted} file(s)");
        }
        Ok(_) => {
            println!("no HEIC/HEIF files found");
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run(cli: &Cli) -> Result<usize, ConvertError> {
    let mut converted = 0;

    for input in &cli.inputs {
        let candidates = collect_candidates(input);
        if candidates.is_empty() {
            if !input.is_dir() {
                eprintln!("skipped: {} (not a HEIC/HEIF file)", input.display());
            }
            continue;
        }

        for candidate in candidates {
            let output = default_output_path(&candidate, cli.output_dir.as_deref());
            if output.exists() && !cli.force {
                eprintln!("skipped: {} (output exists, use --force)", output.display());
                continue;
            }
            if output == candidate {
                eprintln!(
                    "skipped: {} (input and output are the same)",
                    candidate.display()
                );
                continue;
            }

            match convert_to_jpeg(&candidate, &output, cli.quality) {
                Ok(()) => {
                    println!("{} -> {}", candidate.display(), output.display());
                    converted += 1;
                }
                Err(e) => eprintln!("failed: {} ({e})", candidate.display()),
            }
        }
    }

    Ok(converted)
}

/// Returns the .heic/.heif files for a path, recursing into directories.
fn collect_candidates(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    files.extend(collect_candidates(&entry_path));
                } else if is_heic_file(&entry_path) {
                    files.push(entry_path);
                }
            }
        }
    } else if is_heic_file(path) {
        files.push(path.to_path_buf());
    }

    files
}

fn is_heic_file(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.eq_ignore_ascii_case("heic") || ext.eq_ignore_ascii_case("heif"))
        .unwrap_or(false)
}
