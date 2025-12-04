pub mod cli;
pub mod context;
pub mod generator;

use self::cli::{Args, Commands, GenericArgs};
use self::context::scan_directory;
use anyhow::Result;
use clap::{CommandFactory, Parser};
use std::env;

use self::generator::{
    generate_architecture_prompt, generate_generic_prompt, generate_improve_prompt,
    generate_readme_prompt, generate_refactor_prompt, generate_review_prompt,
};

/// Main application logic
pub async fn run() -> Result<()> {
    let args = Args::parse();

    // ‼️ Capture preset before moving args.command (partial move safety)
    let preset = args.preset.clone();

    // ‼️ Resolve the command FIRST to determine if scanning is necessary
    let command = match args.command {
        Some(cmd) => cmd,
        None => {
            if let Some(prompt) = args.prompt {
                Commands::Generic(GenericArgs { prompt })
            } else if args.improve {
                Commands::Improve
            } else {
                Args::command().print_help()?;
                return Ok(());
            }
        }
    };

    // ‼️ Conditional Scanning: Skip scan_directory for Architecture command
    let should_scan = !matches!(command, Commands::Architecture(_));

    let reference_code = if should_scan {
        scan_directory(env::current_dir()?, preset).await?
    } else {
        String::new()
    };

    let output = match &command {
        Commands::Generic(cmd_args) => generate_generic_prompt(cmd_args, &reference_code),
        Commands::Architecture(cmd_args) => generate_architecture_prompt(cmd_args),
        Commands::CodeReview(cmd_args) => generate_review_prompt(cmd_args, &reference_code),
        Commands::Refactor(cmd_args) => generate_refactor_prompt(cmd_args, &reference_code),
        Commands::Readme(cmd_args) => generate_readme_prompt(cmd_args, &reference_code),
        Commands::Improve => generate_improve_prompt(&reference_code),
    };

    println!("{}", output);

    Ok(())
}
