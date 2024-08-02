use std::fs::File;
use std::io::{stdout, BufRead, BufReader, BufWriter, Write};

use anyhow::{Context, Ok, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file '{}'.", args.path.display()))?;
    let reader = BufReader::new(file);
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            writeln!(writer, "{line}")?;
        }
    }

    writer.flush().unwrap();

    Ok(())
}
