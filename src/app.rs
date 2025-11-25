// src/app.rs
// ‼️ Module Root: Orchestrates the application flow.

pub mod cli;
pub mod context;
pub mod generator;

use anyhow::Result;
use clap::Parser;
use std::io::{self, Read};

// ‼️ Removed unused `PromptMode` import to fix warning
use self::cli::Args;
use self::context::scan_directory;
use self::generator::{GeneratorContext, generate_prompt};

/// Main application logic
pub fn run() -> Result<()> {
    // ‼️ Parse arguments using the refactored CLI module
    let args = Args::parse();

    // ‼️ Handle Stdin vs Argument description
    let project_description = get_description(&args)?;

    // ‼️ Handle Context Scanning (moved to helper module)
    let reference_code = if let Some(path) = args.scan {
        eprintln!("🔍 Scanning directory: {:?}", path);
        scan_directory(path)?
    } else {
        String::new()
    };

    let context = GeneratorContext {
        description: project_description,
        stack: args.stack,
        specific_constraints: args.context.unwrap_or_default(),
        reference_code,
    };

    // ‼️ Generate output based on the selected Mode
    let output = generate_prompt(args.mode, &context);

    println!("{}", output);

    Ok(())
}

/// Helper to extract description from stdin or args
fn get_description(args: &Args) -> Result<String> {
    if args.stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    } else {
        Ok(args
            .description
            .clone()
            .unwrap_or_else(|| "A generic software project".to_string()))
    }
}
