use anyhow::Result;
use colored::Colorize;

use crate::cli::ConfigAction;
use crate::config::Config;

pub fn execute(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Set {
            backend,
            model,
            api_key,
            output_dir,
        } => {
            set_config(backend, model, api_key, output_dir)?;
        }
        ConfigAction::Get { key } => {
            get_config(key)?;
        }
        ConfigAction::List => {
            list_config()?;
        }
    }

    Ok(())
}

fn set_config(
    backend: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    output_dir: Option<String>,
) -> Result<()> {
    let mut config = Config::load().unwrap_or_default();

    let mut updated = false;

    if let Some(b) = backend {
        // Validate backend
        if !["ollama", "gemini", "claude"].contains(&b.as_str()) {
            anyhow::bail!("Invalid backend. Valid options: ollama, gemini, claude");
        }
        println!(
            "{} Setting backend to: {}",
            "✓".green(),
            b.bright_cyan()
        );
        config.backend = b;
        updated = true;
    }

    if let Some(m) = model {
        println!(
            "{} Setting model to: {}",
            "✓".green(),
            m.bright_cyan()
        );
        config.model = m;
        updated = true;
    }

    if let Some(key) = api_key {
        println!("{} API key updated", "✓".green());
        config.api_key = Some(key);
        updated = true;
    }

    if let Some(dir) = output_dir {
        println!(
            "{} Setting output directory to: {}",
            "✓".green(),
            dir.bright_cyan()
        );
        config.output_dir = Some(dir);
        updated = true;
    }

    if updated {
        config.save()?;
        println!(
            "\n{} Configuration saved to: {}",
            "✓".green().bold(),
            Config::config_path()?.display().to_string().bright_cyan()
        );
    } else {
        println!("{}", "No configuration changes specified".yellow());
        println!("\nUsage examples:");
        println!("  animaforge config set --backend ollama");
        println!("  animaforge config set --model llama2");
        println!("  animaforge config set --api-key YOUR_KEY");
        println!("  animaforge config set --output-dir ./my_animations");
    }

    Ok(())
}

fn get_config(key: Option<String>) -> Result<()> {
    let config = Config::load()?;

    if let Some(k) = key {
        match k.as_str() {
            "backend" => println!("{}", config.backend),
            "model" => println!("{}", config.model),
            "api_key" => {
                if let Some(key) = &config.api_key {
                    println!("{}", mask_api_key(key));
                } else {
                    println!("(not set)");
                }
            }
            "output_dir" => {
                if let Some(dir) = &config.output_dir {
                    println!("{}", dir);
                } else {
                    println!("./animations (default)");
                }
            }
            _ => {
                anyhow::bail!("Unknown config key: {}", k);
            }
        }
    } else {
        // Show all config
        list_config()?;
    }

    Ok(())
}

fn list_config() -> Result<()> {
    let config = Config::load()?;

    println!("{}", "Current Configuration:".bright_green().bold());
    println!("────────────────────────────────────");

    println!(
        "{:15} {}",
        "Backend:".bright_yellow(),
        config.backend.bright_cyan()
    );

    println!(
        "{:15} {}",
        "Model:".bright_yellow(),
        config.model.bright_cyan()
    );

    if let Some(key) = &config.api_key {
        println!(
            "{:15} {}",
            "API Key:".bright_yellow(),
            mask_api_key(key).bright_cyan()
        );
    } else {
        println!(
            "{:15} {}",
            "API Key:".bright_yellow(),
            "(not set)".bright_black()
        );
    }

    if let Some(dir) = &config.output_dir {
        println!(
            "{:15} {}",
            "Output Dir:".bright_yellow(),
            dir.bright_cyan()
        );
    } else {
        println!(
            "{:15} {}",
            "Output Dir:".bright_yellow(),
            "./animations (default)".bright_cyan()
        );
    }

    if let Some(endpoint) = &config.ollama_endpoint {
        println!(
            "{:15} {}",
            "Ollama URL:".bright_yellow(),
            endpoint.bright_cyan()
        );
    }

    println!("────────────────────────────────────");
    println!(
        "\n{} {}",
        "Config file:".bright_black(),
        Config::config_path()?.display().to_string().bright_black()
    );

    Ok(())
}

fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        return "********".to_string();
    }

    let visible_chars = 4;
    let masked_len = key.len() - (visible_chars * 2);

    format!(
        "{}{}{}",
        &key[..visible_chars],
        "*".repeat(masked_len),
        &key[key.len() - visible_chars..]
    )
}
