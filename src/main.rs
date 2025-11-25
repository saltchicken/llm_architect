// src/main.rs
// ‼️ Refactored: Main is now strictly an entry point.
// Logic has been moved to src/app.rs as per Engineering Requirement #6.

mod app;

fn main() {
    // ‼️ Delegate execution to the app module
    if let Err(e) = app::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

