// src/app.rs

pub mod cli;
pub mod context;
pub mod generator;

use anyhow::Result;
use clap::Parser;
use std::io::{self, Read};

use self::cli::Args;
use self::context::scan_directory;
use self::generator::{GeneratorContext, generate_prompt};

/// Main application logic
pub fn run() -> Result<()> {
    let args = Args::parse();

    let project_description = get_description(&args)?;

    let reference_code = if let Some(path) = args.scan {
        eprintln!("🔍 Scanning directory: {:?}", path);
        scan_directory(path)?
    } else {
        String::new()
    };

    let context = GeneratorContext {
        description: project_description,
        specific_constraints: args.context.unwrap_or_default(),
        reference_code,
    };

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

