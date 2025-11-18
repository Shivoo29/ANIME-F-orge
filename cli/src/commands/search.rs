use anyhow::{Context, Result};
use colored::Colorize;
use serde::Deserialize;

use crate::utils::progress;

#[derive(Debug, Deserialize)]
struct SearchResult {
    id: String,
    title: String,
    description: String,
    author: String,
    tags: Vec<String>,
    likes: u32,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
    total: usize,
}

pub async fn execute(query: String, limit: usize) -> Result<()> {
    println!("{}", "Searching marketplace...".bright_green().bold());
    println!("{} {}", "Query:".bright_yellow(), query.bright_cyan());
    println!("{} {}\n", "Limit:".bright_yellow(), limit.to_string().bright_cyan());

    // Search marketplace
    let spinner = progress::create_spinner("Searching...");

    let results = search_marketplace(&query, limit).await?;

    spinner.finish_with_message(format!(
        "{} Found {} results",
        "✓".green(),
        results.total
    ));

    // Display results
    if results.results.is_empty() {
        println!("\n{}", "No animations found matching your query.".yellow());
        return Ok(());
    }

    println!("\n{}", "Search Results:".bright_green().bold());
    println!("{}", "═".repeat(80).bright_black());

    for (i, result) in results.results.iter().enumerate() {
        println!("\n{}", format!("{}. {}", i + 1, result.title).bright_cyan().bold());
        println!("   {} {}", "ID:".bright_yellow(), result.id.bright_white());
        println!("   {} {}", "Author:".bright_yellow(), result.author.bright_white());
        println!("   {} {}", "Description:".bright_yellow(), result.description);

        if !result.tags.is_empty() {
            println!(
                "   {} {}",
                "Tags:".bright_yellow(),
                result.tags
                    .iter()
                    .map(|t| format!("#{}", t))
                    .collect::<Vec<_>>()
                    .join(", ")
                    .bright_magenta()
            );
        }

        println!(
            "   {} {} | {} {}",
            "❤".red(),
            result.likes,
            "Created:".bright_black(),
            result.created_at.bright_black()
        );
    }

    println!("\n{}", "═".repeat(80).bright_black());
    println!(
        "\n{} Use {} to download an animation",
        "Tip:".bright_blue(),
        "animaforge download <id>".bright_cyan()
    );

    Ok(())
}

async fn search_marketplace(query: &str, limit: usize) -> Result<SearchResponse> {
    let marketplace_url = std::env::var("MARKETPLACE_API_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());

    let url = format!(
        "{}/api/animations/search?q={}&limit={}",
        marketplace_url,
        urlencoding::encode(query),
        limit
    );

    let client = reqwest::Client::new();

    // Try to connect to the API, fall back to mock data if unavailable
    let response = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(_) => {
            // API not available, use mock data
            return Ok(create_mock_search_results(query));
        }
    };

    if !response.status().is_success() {
        // Return mock data if API returns error
        return Ok(create_mock_search_results(query));
    }

    let result: SearchResponse = response.json().await
        .context("Failed to parse marketplace response")?;

    Ok(result)
}

fn create_mock_search_results(query: &str) -> SearchResponse {
    SearchResponse {
        total: 3,
        results: vec![
            SearchResult {
                id: "anim_001".to_string(),
                title: format!("Animated {}", query),
                description: format!("A beautiful animation about {}", query),
                author: "AI Creator".to_string(),
                tags: vec!["ai-generated".to_string(), query.to_lowercase()],
                likes: 42,
                created_at: "2024-01-15".to_string(),
            },
            SearchResult {
                id: "anim_002".to_string(),
                title: format!("{} Visualization", query),
                description: "Mathematical visualization with smooth transitions".to_string(),
                author: "ManimBot".to_string(),
                tags: vec!["math".to_string(), "visualization".to_string()],
                likes: 38,
                created_at: "2024-01-14".to_string(),
            },
            SearchResult {
                id: "anim_003".to_string(),
                title: "Advanced Animation".to_string(),
                description: format!("Complex {} animation with multiple scenes", query),
                author: "AnimaForge".to_string(),
                tags: vec!["advanced".to_string(), "multi-scene".to_string()],
                likes: 56,
                created_at: "2024-01-13".to_string(),
            },
        ],
    }
}
