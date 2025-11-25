# **LLM Architect**

**LLM Architect** is a CLI tool written in Rust designed to bridge the gap between a simple project idea and a professional-grade Large Language Model (LLM) prompt.  
It takes a basic project description and wraps it in strict engineering constraints (modularity, error handling, type safety), generating a highly structured prompt that you can feed into tools like ChatGPT, Claude, or Gemini to get better, more maintainable code results.

## **🚀 Features**

* **Prompt Engineering Automation**: Automatically structures requests into "Architecture," "Implementation," and "Usage" phases.  
* **Stack Awareness**: detailed prompt adjustments based on the target technology (e.g., enforces specific module structures for Rust projects).  
* **Context Injection**: feed in existing code files or specific library constraints to guide the LLM.  
* **Stdin Support**: Pipe descriptions directly from other tools or files.

## **📦 Installation**

Ensure you have Rust and Cargo installed.  
\# Clone the repository  
git clone \[https://github.com/yourusername/llm\_architect.git\](https://github.com/yourusername/llm\_architect.git)  
cd llm\_architect

\# Build the project  
cargo build \--release

## **🛠 Usage**

You can run the tool using cargo run. The output is printed to stdout, so you can redirect it to a file or copy it directly.

### **Basic Usage**

Generate a prompt for a Python web scraper:  
cargo run \-- \--description "A web scraper that tracks GPU prices" \--stack "Python"

### **Using Specific Context**

Add constraints, such as specific libraries you want to use:  
cargo run \-- \\  
  \-d "A real-time chat application" \\  
  \-s "React \+ Firebase" \\  
  \--context "Must use Tailwind CSS for styling and Firestore for the DB"

### **Piping Input (Stdin)**

Useful for long descriptions stored in text files:  
cat my\_idea.txt | cargo run \-- \--stdin \--stack "Rust" \> prompt.md

### **Injecting Reference Code**

If you want the LLM to follow the style of an existing file:  
cargo run \-- \\  
  \-d "Add a new endpoint for user authentication" \\  
  \-s "Go" \\  
  \--code-context "./src/existing\_handler.go"

## **⚙️ CLI Options**

| Flag | Short | Description | Default |
| :---- | :---- | :---- | :---- |
| \--description | \-d | The main idea/description of the project. | None |
| \--stack | \-s | Target tech stack (e.g., "Rust", "Node"). | "General Software" |
| \--context | \-c | Specific constraints or libraries to use. | None |
| \--code-context |  | Path to a file containing reference code. | None |
| \--stdin |  | Read description from standard input. | false |

## **🧠 How It Works**

When you run the tool, it generates a prompt that instructs the AI to follow specific **Predetermined Engineering Requirements**:

1. **Modularity**: Forbids dumping code into single files.  
2. **Error Handling**: Demands rigorous, idiomatic error handling.  
3. **Refactoring Strategy**: Enforces breaking down large functions.  
4. **Entry Point Structure**: Specifically for Rust, it demands main.rs be minimal and logic moved to src/app.rs.

## **📝 License**

[MIT](https://www.google.com/search?q=LICENSE)

