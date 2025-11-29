pub mod cli;
pub mod context;
pub mod generator;

use self::cli::{Args, Commands, GenericArgs};
use self::context::scan_directory;
use anyhow::Result;
use clap::{CommandFactory, Parser};
use std::env;

use self::generator::{
    generate_architecture_prompt, generate_generic_prompt, generate_readme_prompt,
    generate_refactor_prompt, generate_review_prompt,
};

/// Main application logic

pub async fn run() -> Result<()> {
    let args = Args::parse();


    let reference_code = scan_directory(env::current_dir()?, args.preset.clone()).await?;

    let command = match args.command {
        Some(cmd) => cmd,
        None => {
            if let Some(prompt) = args.prompt {
                Commands::Generic(GenericArgs { prompt })
            } else {
                Args::command().print_help()?;
                return Ok(());
            }
        }
    };

    let output = match &command {
        Commands::Generic(cmd_args) => generate_generic_prompt(cmd_args, &reference_code),
        Commands::Architecture(cmd_args) => generate_architecture_prompt(cmd_args, &reference_code),
        Commands::CodeReview(cmd_args) => generate_review_prompt(cmd_args, &reference_code),
        Commands::Refactor(cmd_args) => generate_refactor_prompt(cmd_args, &reference_code),
        Commands::Readme(cmd_args) => generate_readme_prompt(cmd_args, &reference_code),
    };

    println!("{}", output);

    Ok(())
}