use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{Input, Confirm};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::utils::progress;

#[derive(Debug, Serialize)]
struct PublishRequest {
    title: String,
    description: String,
    tags: Vec<String>,
    code: String,
}

#[derive(Debug, Deserialize)]
struct PublishResponse {
    id: String,
    url: String,
    message: String,
}

pub async fn execute(file: String) -> Result<()> {
    println!("{}", "Publishing animation to marketplace...".bright_green().bold());
    println!("{} {}\n", "File:".bright_yellow(), file.bright_cyan());

    // Validate file exists
    let file_path = Path::new(&file);
    if !file_path.exists() {
        anyhow::bail!("Animation file not found: {}", file);
    }

    // Read the animation code
    let code = fs::read_to_string(file_path)
        .context("Failed to read animation file")?;

    // Interactive prompts for metadata
    println!("{}", "Enter animation details:".bright_blue().bold());
    println!();

    let title: String = Input::new()
        .with_prompt("Title")
        .interact_text()?;

    let description: String = Input::new()
        .with_prompt("Description")
        .interact_text()?;

    let tags_input: String = Input::new()
        .with_prompt("Tags (comma-separated)")
        .interact_text()?;

    let tags: Vec<String> = tags_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    println!();

    // Confirmation
    println!("{}", "Publishing with the following details:".bright_yellow());
    println!("  Title: {}", title.bright_cyan());
    println!("  Description: {}", description.bright_cyan());
    println!("  Tags: {}", tags.join(", ").bright_cyan());
    println!();

    let confirmed = Confirm::new()
        .with_prompt("Proceed with publishing?")
        .default(true)
        .interact()?;

    if !confirmed {
        println!("{}", "Publishing cancelled.".yellow());
        return Ok(());
    }

    // Load config for marketplace URL and auth
    let config = Config::load()?;

    // Create publish request
    let publish_data = PublishRequest {
        title,
        description,
        tags,
        code,
    };

    // Upload with progress
    let pb = progress::create_spinner("Uploading to marketplace...");

    let result = upload_to_marketplace(&config, publish_data).await?;

    pb.finish_with_message(format!("{} Upload complete!", "✓".green()));

    // Display result
    println!("\n{}", "✨ Animation published successfully!".bright_green().bold());
    println!("{} Animation ID: {}", "→".bright_blue(), result.id.bright_cyan());

    if !result.url.is_empty() {
        println!("{} URL: {}", "→".bright_blue(), result.url.bright_cyan());
    }

    println!("\n{}", result.message.bright_white());

    Ok(())
}

async fn upload_to_marketplace(
    config: &Config,
    data: PublishRequest,
) -> Result<PublishResponse> {
    let marketplace_url = std::env::var("MARKETPLACE_API_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());

    let url = format!("{}/api/animations", marketplace_url);

    let client = reqwest::Client::new();

    let mut request = client.post(&url)
        .json(&data);

    // Add auth token if available
    if let Some(token) = &config.marketplace_token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let response = request
        .send()
        .await
        .context("Failed to connect to marketplace API")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

        anyhow::bail!(
            "Marketplace API error ({}): {}\n\nNote: Make sure the marketplace API is running on {}",
            status,
            error_text,
            marketplace_url
        );
    }

    let result: PublishResponse = response.json().await
        .context("Failed to parse marketplace response")?;

    Ok(result)
}
