use clap::Parser;
use std::io::{self, Read};

/// This allows us to pass the project description and target stack easily.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The main idea or description of the project you want to build
    #[arg(short, long)]
    description: Option<String>,

    /// The specific programming language or tech stack (e.g., "Rust", "React", "Python")
    #[arg(short, long, default_value = "General Software")]
    stack: String,

    /// If set, reads the description from Stdin instead of an argument
    #[arg(long)]
    stdin: bool,
}

fn main() {
    let args = Args::parse();

    let project_description = if args.stdin {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer.trim().to_string()
    } else {
        args.description
            .unwrap_or_else(|| "A generic software project".to_string())
    };

    let design_doc = generate_design_prompt(&project_description, &args.stack);

    println!("{}", design_doc);
}

/// It wraps the user's simple idea in a wrapper of professional engineering constraints.
fn generate_design_prompt(description: &str, stack: &str) -> String {
    format!(
        r#"# PROMPT FOR LLM: PROJECT ARCHITECTURE & CODE GENERATION

## 1. ROLE DEFINITION
You are an expert Senior Software Engineer and System Architect specializing in **{stack}**.
Your goal is to take the project description below and produce a complete, production-ready implementation plan and codebase.

## 2. PROJECT DESCRIPTION
**User Requirement:**
"{description}"

## 3. PREDETERMINED ENGINEERING REQUIREMENTS
Please adhere to the following strict design principles:

1.  **Modularity:** Break code into logical files and functions. Do not dump everything into one file unless explicitly small.
2.  **Error Handling:** rigorous error handling (no silent failures). Use idiomatic patterns for {stack}.
3.  **Type Safety:** Leverage the type system to prevent runtime errors where possible.
4.  **Comments:** Add brief documentation for complex logic, but self-documenting variable names are preferred.
5.  **Configuration:** Avoid magic numbers. Use constants or configuration files/env variables.
6.  **Entry Point Structure:** Refactor the code so that main.rs is a minimal entry point. Move the application logic into a module named app. Use src/app.rs as the module root (sibling file approach) to expose the submodules located inside the src/app/ folder. app.rs should act as the facade that initializes the database, config, and state.
7.  **Refactoring Strategy:** Apply 'Extract Method' aggressively. If a function is longer than 30 lines or handles multiple responsibilities, break it down into smaller, named helper functions.

## 4. REQUIRED OUTPUT FORMAT
Please response in the following order:

### Phase 1: Architecture Design
* **File Structure:** A tree view of the proposed directory structure.
* **Core Data Models:** Define the key structs/classes/database schemas.
* **Dependencies:** List external libraries required.

### Phase 2: Implementation
* Provide the code for the core logic.
* **IMPORTANT:** Create a separate code block for EVERY file.
* Label each code block with the filename (e.g., `src/main.rs`).

### Phase 3: Usage Instructions
* How to build/run the project.
* Example usage commands.

---
*Please begin by analyzing the User Requirement and generating the Phase 1 Design.*
"#,
        stack = stack,
        description = description
    )
}
