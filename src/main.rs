use clap::Parser;
use std::fs; // ‼️ Added fs to read the reference file
use std::io::{self, Read};

/// ‼️ Struct defining the command line arguments using Clap.
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

    /// ‼️ specific constraints or library preferences (e.g. "Use Tailwind", "Postgres DB")
    #[arg(short, long)]
    context: Option<String>,

    /// ‼️ New: Path to a file containing code context (output from the code_context tool)
    #[arg(long)]
    code_context: Option<String>,

    /// If set, reads the description from Stdin instead of an argument
    #[arg(long)]
    stdin: bool,
}

fn main() {
    let args = Args::parse();

    // ‼️ Logic to handle input source (Command line arg vs Stdin pipe)
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

    // ‼️ New: Read the code context file if provided
    let reference_code = if let Some(path) = &args.code_context {
        fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!("⚠️ Warning: Could not read code context file: {}", path);
            String::new()
        })
    } else {
        String::new()
    };

    // ‼️ Generate the Prompt / Design Doc (Passing the new reference_code arg)
    let extra_context = args.context.unwrap_or_default();
    let design_doc = generate_design_prompt(
        &project_description,
        &args.stack,
        &extra_context,
        &reference_code,
    );

    // ‼️ Output directly to stdout so it can be piped or copied
    println!("{}", design_doc);
}

/// ‼️ This function constructs the actual text that will be fed into the LLM.
/// It wraps the user's simple idea in a wrapper of professional engineering constraints.
fn generate_design_prompt(
    description: &str,
    stack: &str,
    extra_context: &str,
    reference_code: &str,
) -> String {
    // ‼️ Dynamic Logic: tailored entry point rule based on stack
    let entry_point_rule = if stack.to_lowercase().contains("rust") {
        "6.  **Entry Point Structure:** Refactor the code so that main.rs is a minimal entry point. Move the application logic into a module named app. Use src/app.rs as the module root."
    } else {
        "6.  **Entry Point Structure:** Keep the main entry file (e.g., index.js, main.py) minimal. Delegate initialization and logic to a dedicated App class or module."
    };

    // ‼️ Dynamic Logic: specific library constraints section
    let mut specific_constraints = String::new();
    if !extra_context.is_empty() {
        specific_constraints = format!(
            "\n## 3. SPECIFIC LIBRARY/CONTEXT CONSTRAINTS\nUser provided constraints:\n- {}\n",
            extra_context
        );
    }

    // ‼️ Dynamic Logic: reference code section (handles the XML-like output from code_context)
    let mut reference_section = String::new();
    if !reference_code.is_empty() {
        reference_section = format!(
            r#"
## 4. REFERENCE CODEBASE / CONTEXT
The user has provided the following existing codebase structure and content to base the new project off of. 
Use this to align with existing patterns, directory structures, and configurations.

```xml
{}
```
"#,
            reference_code
        );
    }

    format!(
        r#"# PROMPT FOR LLM: PROJECT ARCHITECTURE & CODE GENERATION

## 1. ROLE DEFINITION
You are an expert Senior Software Engineer and System Architect specializing in **{stack}**.
Your goal is to take the project description below and produce a complete, production-ready implementation plan and codebase.

## 2. PROJECT DESCRIPTION
**User Requirement:**
"{description}"
{specific_constraints}{reference_section}
## 5. PREDETERMINED ENGINEERING REQUIREMENTS
‼️ (The following constraints are hardcoded to ensure high-quality LLM output)
Please adhere to the following strict design principles:

1.  **Modularity:** Break code into logical files and functions. Do not dump everything into one file unless explicitly small.
2.  **Error Handling:** rigorous error handling (no silent failures). Use idiomatic patterns for {stack}.
3.  **Type Safety:** Leverage the type system to prevent runtime errors where possible.
4.  **Comments:** Add brief documentation for complex logic, but self-documenting variable names are preferred.
5.  **Configuration:** Avoid magic numbers. Use constants or configuration files/env variables.
{entry_point_rule}
7.  **Refactoring Strategy:** Apply 'Extract Method' aggressively. If a function is longer than 30 lines or handles multiple responsibilities, break it down into smaller, named helper functions.
8.  **Testing:** Include a testing strategy. Where applicable, provide basic unit tests for core logic.

## 6. REQUIRED OUTPUT FORMAT
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
        description = description,
        specific_constraints = specific_constraints,
        reference_section = reference_section,
        entry_point_rule = entry_point_rule
    )
}
