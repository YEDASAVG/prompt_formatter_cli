use anyhow::{Ok, Result};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use formatter::{FormatMode, Formatter};

mod analyzer;
mod formatter;
mod templates;
mod validator;

// #[derive(Parser)]
// #[command(name = "prompt-fmt")]
// #[command(about = "Advanced LLM prompt formatter - Save Tokens, Save Money!")]
// struct Args {
//     input: Option<PathBuf>, // input file path OR use stdin if not provided

//     #[arg(short, long)] // output file path if not provided
//     output: Option<PathBuf>,

//     #[arg(short, long, default_value = "readable")] // formatting mode
//     mode: FormatMode,

//     #[arg(long, default_value = "gpt-4")] // model for token countiing
//     model: String,

//     #[arg(short, long)] // only validate
//     validate: bool,

//     #[arg(short, long)] // show stats
//     stats: bool,

//     #[arg(short, long)]
//     compare: bool,

//     #[arg(long)]
//     vars: Option<String>,
// }

// #[derive(Debug, Clone, ValueEnum)]
// enum FormatMode {
//     Compact,  // Maximum Token saving (Aggressive)
//     Readable, // Balance readability and cost
//     Minimal,  // Extreme compression (single line)
// }
fn main() -> Result<()>{
    // let args = Args::parse();
    // println!("Prompt Formatter - Starting...\n");

    let test_text = "Hello.  \n\n\n\nWorld  ";
    let formatter= Formatter::new(FormatMode::Readable);
    let result = formatter.format(test_text);
    println!("Input: {:?}", test_text);
    println!("Output: {:?}", result);
    // println!("Model: {:?}", args.model);

    Ok(())
}
