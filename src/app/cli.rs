use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Sets a custom prompt
    #[arg(short, long)]
    pub prompt: Option<String>,

    #[arg(long)]
    pub preset: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a generic prompt
    Generic(GenericArgs),
    /// Generate a full project architecture and implementation plan
    Architecture(ArchitectureArgs),
    /// Generate a prompt for reviewing existing code
    CodeReview(CodeReviewArgs),
    /// Generate a prompt for refactoring specific logic
    Refactor(RefactorArgs),
    /// Generate a prompt for a README file
    Readme(ReadmeArgs),
}
// ... existing code ...
#[derive(Parser, Debug)]
pub struct ArchitectureArgs {
    /// The main idea or description of the project
    #[arg(short, long)]
    pub description: Option<String>,
    /// Specific constraints or library requirements
    #[arg(short, long)]
    pub context: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CodeReviewArgs {
    /// What specific area to focus the review on
    #[arg(short, long)]
    pub focus: Option<String>,
}

#[derive(Parser, Debug)]
pub struct RefactorArgs {
    /// The specific goal of the refactor (e.g., "Modernize error handling")
    #[arg(short, long)]
    pub goal: String,
}

#[derive(Parser, Debug)]
pub struct ReadmeArgs {
    /// Tone of the readme (Specific to this generator only!)
    #[arg(long, default_value = "Professional and Concise")]
    pub style: String,
    /// Extra details to include
    #[arg(short, long)]
    pub details: Option<String>,
}

#[derive(Parser, Debug)]
pub struct GenericArgs {
    /// The prompt to generate
    #[arg(short, long)]
    pub prompt: String,
}
