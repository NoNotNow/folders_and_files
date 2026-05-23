use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Write, stdin, stdout};
use std::path::PathBuf;
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory to scan
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Path to the output text file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn prompt_for_input(prompt: &str) -> PathBuf {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    PathBuf::from(input.trim())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Abfragen, falls Argumente fehlen
    let input_path = match args.input {
        Some(path) => path,
        None => prompt_for_input("Enter input directory path: "),
    };

    let output_path = match args.output {
        Some(path) => path,
        None => prompt_for_input("Enter output file path: "),
    };

    println!("Scanning directory: {:?}", input_path);
    println!("Output file: {:?}", output_path);

    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);

    writeln!(writer, "Scan results for: {:?}", input_path)?;
    writeln!(writer, "Generated at: {}\n", Utc::now())?;
    writeln!(writer, "{:<10} | {:<25} | {}", "Size (B)", "Modified (UTC)", "Path")?;
    writeln!(writer, "{:-<10}-+-{:-<25}-+-{:-<50}", "", "", "")?;

    let mut count = 0;
    for entry in WalkDir::new(&input_path).into_iter().filter_map(|e| e.ok()) {
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