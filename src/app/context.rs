use anyhow::{Context, Result};
// ‼️ Change: Import build_config to utilize the new preset logic
use code_context::app::generate;
use code_context::build_config;
use std::path::PathBuf;

// ‼️ Change: Added preset_override parameter
pub fn scan_directory(path: PathBuf, preset_override: Option<String>) -> Result<String> {
    // ‼️ Change: Use the folder name as the fallback
    let folder_name = path.file_name().and_then(|n| n.to_str());

    // ‼️ Change: Priority: Explicit CLI Override -> Folder Name -> Default (None)
    let preset_key = preset_override.as_deref().or(folder_name);

    // ‼️ Change: Build config dynamically using the library
    let config = build_config(
        preset_key, // Use the resolved key
        None,       // No dynamic include overrides provided here
        None,       // No dynamic exclude overrides provided here
        None,       // No dynamic tree overrides provided here
        false,      // Not tree-only
    )
    .context("Failed to resolve code context configuration")?;

    generate(config, path)
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to scan directory")
}
