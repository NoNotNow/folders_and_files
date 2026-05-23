use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory to scan
    #[arg(short, long)]
    input: PathBuf,

    /// Path to the output text file
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Scanning directory: {:?}", args.input);
    println!("Output file: {:?}", args.output);

    let output_file = File::create(&args.output)?;
    let mut writer = BufWriter::new(output_file);

    writeln!(writer, "Scan results for: {:?}", args.input)?;
    writeln!(writer, "Generated at: {}\n", Utc::now())?;
    writeln!(writer, "{:<10} | {:<25} | {}", "Size (B)", "Modified (UTC)", "Path")?;
    writeln!(writer, "{:-<10}-+-{:-<25}-+-{:-<50}", "", "", "")?;

    let mut count = 0;
    for entry in WalkDir::new(&args.input).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if metadata.is_file() {
            let size = metadata.len();
            let modified_str = metadata.modified()
                .map(|t| {
                    let dt: DateTime<Utc> = DateTime::from(t);
                    dt.format("%Y-%m-%d %H:%M:%S").to_string()
                })
                .unwrap_or_else(|_| "N/A".to_string());

            writeln!(
                writer,
                "{:<10} | {:<25} | {}",
                size,
                modified_str,
                path.display()
            )?;
            count += 1;
        }
    }

    writer.flush()?;
    println!("Done! Scanned {} files.", count);

    Ok(())
}
