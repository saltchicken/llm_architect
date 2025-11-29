use anyhow::{Context, Result};
use code_context::app::generate;
use code_context::build_config;

use sql_context::{AppConfig, generate_report};
use std::path::PathBuf;


pub async fn scan_directory(path: PathBuf, preset_override: Option<String>) -> Result<String> {
    let folder_name = path.file_name().and_then(|n| n.to_str());
    let preset_key = preset_override.as_deref().or(folder_name);

    let config = build_config(preset_key, None, None, None, false)
        .context("Failed to resolve code context configuration")?;


    let mut output = generate(config, path)
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to scan directory")?;


    dotenvy::dotenv().ok();


    if let Ok(db_url) = std::env::var("DB_URL") {
        let db_name = db_url.split('/').last().unwrap_or("Unknown").to_string();


        let sql_config = AppConfig { db_url, db_name };


        // We catch errors here so that a DB failure doesn't prevent code context generation
        match generate_report(&sql_config).await {
            Ok(sql_report) => {
                output.push_str("\n\n<database_schema>\n");
                output.push_str(&sql_report);
                output.push_str("\n</database_schema>");
            }
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to generate SQL context: {}", e);
                output.push_str(&format!(
                    "\n\n<!-- SQL Context Generation Failed: {} -->",
                    e
                ));
            }
        }
    }

    Ok(output)
}