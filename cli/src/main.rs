use anyhow::Result;
use clap::Parser;
use colored::Colorize;

mod cli;
mod commands;
mod config;
mod llm;
mod utils;

use cli::{Cli, Commands};

const LOGO: &str = r#"
    ___          _                ______
   /   |  ____  (_)___ ___  ____ / ____/___  _________ ____
  / /| | / __ \/ / __ `__ \/ __ `/ /_  / __ \/ ___/ __ `/ _ \
 / ___ |/ / / / / / / / / / /_/ / __/ / /_/ / /  / /_/ /  __/
/_/  |_/_/ /_/_/_/ /_/ /_/\__,_/_/    \____/_/   \__, /\___/
                                                /____/
"#;

fn print_logo() {
    println!("{}", LOGO.bright_cyan().bold());
    println!(
        "{}\n",
        "AI-Powered Animation Creation Tool".bright_white().italic()
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print logo for most commands (except config get which should be clean)
    if !matches!(cli.command, Commands::Config { .. }) {
        print_logo();
    }

    match cli.command {
        Commands::Create { prompt, render, output } => {
            commands::create::execute(prompt, render, output).await?;
        }
        Commands::Render { file, quality, output } => {
            commands::render::execute(file, quality, output).await?;
        }
        Commands::Config { action } => {
            commands::config::execute(action)?;
        }
        Commands::Publish { file } => {
            commands::publish::execute(file).await?;
        }
        Commands::Search { query, limit } => {
            commands::search::execute(query, limit).await?;
        }
    }

    Ok(())
}
