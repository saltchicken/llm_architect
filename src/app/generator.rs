// src/app/generator.rs

use super::cli::PromptMode;

pub struct GeneratorContext {
    pub description: String,
    pub specific_constraints: String,
    pub reference_code: String,
}

pub fn generate_prompt(mode: PromptMode, ctx: &GeneratorContext) -> String {
    match mode {
        PromptMode::Architecture => generate_architecture_prompt(ctx),
        PromptMode::CodeReview => generate_review_prompt(ctx),
        PromptMode::Refactor => generate_refactor_prompt(ctx),

        PromptMode::Readme => generate_readme_prompt(ctx),
    }
}

fn generate_architecture_prompt(ctx: &GeneratorContext) -> String {
    // ‼️ Change: Removed conditional logic. The Rust-specific entry point rule is now hardcoded.
    let entry_point_rule = "6.  **Entry Point Structure:** Refactor the code so that main.rs is a minimal entry point. Move the application logic into a module folder named app. Use src/app.rs as the module root.";

    let constraints = format_constraints(&ctx.specific_constraints);
    let reference = format_reference(&ctx.reference_code);

    format!(
        r#"# PROMPT FOR LLM: PROJECT ARCHITECTURE & CODE GENERATION

## 1. ROLE DEFINITION
You are an expert Senior Software Engineer and System Architect specializing in **Rust**.
Your goal is to take the project description below and produce a complete, production-ready implementation plan and codebase.

## 2. PROJECT DESCRIPTION
**User Requirement:**
"{description}"
{constraints}{reference}
## 5. PREDETERMINED ENGINEERING REQUIREMENTS
Please adhere to the following strict design principles:

1.  **Modularity:** Break code into logical files and functions.
2.  **Error Handling:** Rigorous error handling (no silent failures).
3.  **Type Safety:** Leverage the type system.
4.  **Comments:** Self-documenting code preferred.
5.  **Configuration:** No magic numbers.
{entry_point_rule}
7.  **Refactoring Strategy:** Aggressive 'Extract Method'.
8.  **Testing:** Include a testing strategy.

## 6. REQUIRED OUTPUT FORMAT
### Phase 1: Architecture Design
* **File Structure**
* **Core Data Models**
* **Dependencies**

### Phase 2: Implementation
* **IMPORTANT:** Create a separate code block for EVERY file.

### Phase 3: Usage Instructions
"#,
        description = ctx.description,
        constraints = constraints,
        reference = reference,
        entry_point_rule = entry_point_rule
    )
}

fn generate_review_prompt(ctx: &GeneratorContext) -> String {
    let constraints = format_constraints(&ctx.specific_constraints);
    let reference = format_reference(&ctx.reference_code);

    format!(
        r#"# PROMPT FOR LLM: SENIOR CODE REVIEW

## 1. ROLE DEFINITION
You are a Principal Engineer specializing in **Rust**.
Your goal is to review the provided code/requirements and identify security flaws, performance bottlenecks, and anti-patterns.

## 2. CONTEXT
**Focus Area:**
"{description}"
{constraints}{reference}
## 3. REVIEW GUIDELINES
1.  **Security:** Check for injection vulnerabilities and unsafe data handling.
2.  **Performance:** Identify O(n^2) operations or unnecessary allocations.
3.  **Readability:** Enforce idiomatic Rust patterns.

## 4. REQUIRED OUTPUT
1.  **Executive Summary:** High-level health check.
2.  **Critical Issues:** Must-fix items.
3.  **Refactoring Suggestions:** Concrete code blocks showing the "Better" way.
"#,
        description = ctx.description,
        constraints = constraints,
        reference = reference,
    )
}

fn generate_refactor_prompt(ctx: &GeneratorContext) -> String {
    let constraints = format_constraints(&ctx.specific_constraints);
    let reference = format_reference(&ctx.reference_code);

    format!(
        r#"# PROMPT FOR LLM: MODERNIZATION & REFACTORING

## 1. ROLE DEFINITION
You are a specialist in technical debt reduction and **Rust** modernization.

## 2. GOAL
Refactor the codebase described below to meet modern standards (Clean Code, SOLID principles).
**Specific Goal:** "{description}"
{constraints}{reference}
## 3. REFACTORING RULES
1.  **Preserve Behavior:** Functionality must remain identical unless specified.
2.  **Split Giant Functions:** No function > 30 lines.
3.  **Dependency Injection:** Remove hardcoded dependencies.

## 4. REQUIRED OUTPUT
1.  **Before/After Analysis:** Briefly explain why the change is needed.
2.  **Refactored Code:** Complete, compile-ready files.
"#,
        description = ctx.description,
        constraints = constraints,
        reference = reference,
    )
}

fn generate_readme_prompt(ctx: &GeneratorContext) -> String {
    // Default style since we don't have a dedicated CLI arg for style yet
    let style = "Professional and Concise";

    let role = format!(
        "# PROMPT FOR LLM: README GENERATION\n\n\
        ## 1. ROLE DEFINITION\n\
        You are an expert Technical Writer and Developer Advocate.\n\
        Your tone should be **{}**.\n\
        Your goal is to analyze the provided source code and generate a comprehensive, production-ready README.md file.",
        style
    );

    let constraints = if !ctx.specific_constraints.is_empty() {
        format!(
            "\n**Specific Constraints:**\n- {}\n",
            ctx.specific_constraints
        )
    } else {
        String::new()
    };

    let task = format!(
        "## 2. USER REQUIREMENT\n\
        **Goal:** {}\n\
        {}",
        ctx.description, constraints
    );

    let context = if ctx.reference_code.is_empty() {
        String::new()
    } else {
        format!(
            "## 3. SOURCE CODE CONTEXT\n\
            The following is the actual file structure and content of the project. \
            Use this to derive installation steps, dependencies, and features.\n\n\
            ```xml\n\
            {}\n\
            ```",
            ctx.reference_code
        )
    };

    let requirements = r#"## 4. OUTPUT REQUIREMENTS
    Please generate a single `README.md` file code block. Ensure the following sections are included (if applicable based on the code):
     
    1.  **Title & Badges:** Project name and relevant status badges (CI, License, version).
    2.  **Description:** A clear 'Elevator Pitch' based on the code's functionality.
    3.  **Features:** Bullet points extracted from the actual implemented logic.
    4.  **Tech Stack:** derived from `Cargo.toml`, `package.json`, etc.
    5.  **Prerequisites:** What needs to be installed (Rust, Node, etc).
    6.  **Installation:** Step-by-step commands.
    7.  **Usage:** Examples of how to run the tool (CLI flags, API calls).
    8.  **Configuration:** specific environment variables or config options found in the code.
     
    **Important Content Rule:** Do not include placeholder text like "Insert description here" - **infer it from the code provided.**
    ### **File Generation & Output Formatting Rule**
    When the user's request requires the generation of a file, a complete code snippet, or a document intended to be copied (like a system prompt or a configuration file), you must follow a specific output format.
    **The default output format is a self-contained HTML document that presents the raw source code within a `<textarea>` element.**
    This HTML document must include:
    1. **A Clear Header:** A title and brief description of the content.
    2. **A `<textarea>` Element:** This element must contain the complete, raw, un-rendered source code of the requested file. It should be set to `readonly`.
    3. **A "Copy to Clipboard" Button:** A prominent button that, when clicked, copies the entire content of the `<textarea>` to the user's clipboard.
    4. **User Feedback:** The copy functionality must provide clear visual feedback, such as changing the button text to "Copied!" for a few seconds. The JavaScript should be robust and compatible with the canvas environment.
    5. **Professional Styling:** The page must be styled using Tailwind CSS for a clean, modern, and usable interface.
    This rule should only be overridden if the user explicitly asks for a different format, such as "show me the rendered markdown" or "just give me the raw code block."
     
    ---
    *Begin by analyzing the code structure above, then generate the HTML-wrapped README.*"#;

    format!("{}\n\n{}\n\n{}\n\n{}", role, task, context, requirements)
}

// Helpers
fn format_constraints(ctx: &str) -> String {
    if ctx.is_empty() {
        String::new()
    } else {
        format!("\n## CONSTRAINTS\nUser provided:\n- {}\n", ctx)
    }
}

fn format_reference(code: &str) -> String {
    if code.is_empty() {
        String::new()
    } else {
        format!(
            r#"
## REFERENCE CODEBASE
The user provided the following context:

```xml
{}
```
"#,
            code
        )
    }
}
