// src/app/context.rs

use anyhow::{Context, Result};
use code_context::app::{generate, models::RuntimeConfig};
use std::path::PathBuf;

pub fn scan_directory(path: PathBuf) -> Result<String> {
    let config = RuntimeConfig {
        // NOTE: This is designed for Rust projects
        include: vec!["**/*.rs".to_string(), "**/*.toml".to_string()],
        exclude: vec![],
        include_in_tree: vec![],
        tree_only_output: false,
    };

    generate(config, path)
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to scan directory")
}
