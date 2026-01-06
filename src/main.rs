use anyhow::{Ok, Result};
use clap::{Parser, ValueEnum};
use colored::*;
use formatter::Formatter;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

mod analyzer;
mod formatter;

#[derive(Parser)]
#[command(name = "prompt-fmt")]
#[command(about = "Advanced LLM prompt formatter - Save Tokens, Save Money!")]
struct Args {
    input: Option<PathBuf>, // input file path OR use stdin if not provided

    #[arg(short, long, default_value = "readable")] // formatting mode
    mode: FormatMode,

    #[arg(long, default_value = "gpt-4")] // model for token countiing
    model: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum FormatMode {
    Compact,  // Maximum Token saving (Aggressive)
    Readable, // Balance readability and cost
    Minimal,  // Extreme compression (single line)
}

impl From<FormatMode> for formatter::FormatMode {
    fn from(mode: FormatMode) -> Self {
        match mode {
            FormatMode::Compact => formatter::FormatMode::Compact,
            FormatMode::Readable => formatter::FormatMode::Readable,
            FormatMode::Minimal => formatter::FormatMode::Minimal,
        }
    }
}

fn read_input(path: Option<PathBuf>) -> Result<String> {
    if let Some(file_path) = path {
        Ok(fs::read_to_string(file_path)?)
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚀 Reading input...");
    let input_text = read_input(args.input)?;

    let formatter = Formatter::new(args.mode.clone().into());
    println!("✨ Formatting...");
    let formatted = formatter.format(&input_text);
    
    println!("📊 Analyzing tokens...\n");
    let stats = analyzer::compare_tokens(&input_text, &formatted, &args.model)?;

    println!(
        "\n{}\n",
        "=== Prompt Formatter Results ===".bright_cyan().bold()
    );
    println!("{}", "--- Token Analysis ---".yellow());
    println!(
        "Original tokens: {} (${:.6})",
        stats.original_tokens.to_string().red(),
        stats.cost_original
    );
    println!(
        "Formatted tokens: {} (${:.6})",
        stats.formatted_tokens.to_string().green(),
        stats.cost_formatted
    );
    println!(
        "Saved: {} tokens ({:.1}%)",
        stats.token_saved.to_string().bright_green().bold(),
        stats.savings_percent
    );
    println!(
        "Cost saved: {}\n",
        format!("${:.6}", stats.cost_saved).bright_green().bold()
    );
    println!("--- Formatted Output ---");
    println!("{}", formatted);

    Ok(())
}
