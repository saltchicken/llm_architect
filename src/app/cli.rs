// src/app/cli.rs

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The main idea or description of the project/task
    #[arg(short, long)]
    pub description: Option<String>,

    /// The specific programming language or tech stack
    // ‼️ Change: Default value changed to "Rust" to reflect project focus
    #[arg(short, long, default_value = "Rust")]
    pub stack: String,

    /// Specific constraints or library requirements
    #[arg(short, long)]
    pub context: Option<String>,

    /// Directory to scan for code context
    #[arg(long)]
    pub scan: Option<PathBuf>,

    /// Read description from Stdin
    #[arg(long)]
    pub stdin: bool,

    #[arg(short, long, value_enum, default_value_t = PromptMode::Architecture)]
    pub mode: PromptMode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum PromptMode {
    /// Generate a full project architecture and implementation plan
    Architecture,
    /// Generate a prompt for reviewing existing code
    CodeReview,
    /// Generate a prompt for refactoring specific logic
    Refactor,

    Readme,
}

