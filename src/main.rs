use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use serde::Serialize;
use prettytable::{Table, row, cell};

/// A CLI log analyzer
#[derive(Parser, Debug)]
#[command(name = "loglyzer")]
#[command(version = "1.0")]
#[command(about = "Analyze log files and extract patterns", long_about = None)]
struct Cli {
    /// Path to log file
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    format: OutputFormat,

    /// Show only error logs
    #[arg(short, long)]
    errors_only: bool,

    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

/// Parsed log entry
#[derive(Debug, Serialize, Clone)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

/// Parses a log line into a LogEntry
fn parse_log_line(line: &str) -> Option<LogEntry> {
    let re = Regex::new(
        r"^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(\w+)\] (.+)$"
    ).unwrap();

    re.captures(line).map(|cap| LogEntry {
        timestamp: cap[1].to_string(),
        level: cap[2].to_string(),
        message: cap[3].to_string(),
    })
}

/// Reads all logs from file
fn read_logs(path: &PathBuf) -> Result<Vec<LogEntry>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut entries = vec![];

    for line in reader.lines() {
        let line = line?;
        if let Some(entry) = parse_log_line(&line) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

/// Renders logs in a pretty table
fn render_table(entries: &[LogEntry]) {
    let mut table = Table::new();
    table.add_row(row!["Timestamp", "Level", "Message"]);

    for e in entries {
        table.add_row(row![e.timestamp, e.level, e.message]);
    }

    table.printstd();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Reading file: {:?}", cli.input);
    }

    let mut entries = read_logs(&cli.input)?;

    if cli.errors_only {
        entries = entries.into_iter().filter(|e| e.level == "ERROR").collect();
    }

    if cli.verbose {
        println!("Parsed {} entries", entries.len());
    }

    match cli.format {
        OutputFormat::Text => render_table(&entries),
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&entries)?);
        }
        OutputFormat::Csv => {
            println!("timestamp,level,message");
            for e in entries {
                println!("{},{},{}", e.timestamp, e.level, e.message);
            }
        }
    }

    Ok(())
}
