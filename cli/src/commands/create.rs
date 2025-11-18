use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::llm::ollama::OllamaClient;
use crate::utils::progress;

pub async fn execute(prompt: String, auto_render: bool, output: Option<String>) -> Result<()> {
    println!("{}", "Creating animation from prompt...".bright_green().bold());
    println!("{} {}\n", "Prompt:".bright_yellow(), prompt);

    // Load config
    let config = Config::load()?;

    // Create spinner for LLM generation
    let spinner = progress::create_spinner("Generating animation code with AI...");

    // Generate code using LLM
    let animation_code = match config.backend.as_str() {
        "ollama" => {
            let client = OllamaClient::new(
                config.ollama_endpoint.as_deref().unwrap_or("http://localhost:11434")
            );
            client.generate_animation_code(&prompt, &config.model).await?
        }
        "gemini" | "claude" => {
            spinner.finish_with_message(format!("{} Backend not yet implemented, using mock", "⚠".yellow()));
            generate_mock_code(&prompt)
        }
        _ => {
            anyhow::bail!("Unknown backend: {}", config.backend);
        }
    };

    spinner.finish_with_message(format!("{} Animation code generated!", "✓".green()));

    // Validate the generated code
    let validate_spinner = progress::create_spinner("Validating animation code...");
    let is_valid = validate_code(&animation_code)?;

    if is_valid {
        validate_spinner.finish_with_message(format!("{} Code validation passed!", "✓".green()));
    } else {
        validate_spinner.finish_with_message(format!("{} Code validation failed!", "✗".red()));
        anyhow::bail!("Generated code is not valid");
    }

    // Determine output path
    let output_path = determine_output_path(&config, output)?;

    // Save the code
    fs::write(&output_path, &animation_code)
        .context("Failed to save animation code")?;

    println!(
        "\n{} Animation code saved to: {}",
        "✓".green(),
        output_path.display().to_string().bright_cyan()
    );

    // Auto-render if requested
    if auto_render {
        println!("\n{}", "Auto-rendering enabled...".bright_yellow());
        crate::commands::render::execute(
            output_path.to_string_lossy().to_string(),
            "medium".to_string(),
            None,
        )
        .await?;
    }

    println!("\n{}", "✨ Animation creation complete!".bright_green().bold());

    Ok(())
}

fn generate_mock_code(prompt: &str) -> String {
    format!(
        r#"from manim import *

class GeneratedAnimation(Scene):
    """
    Animation created from prompt: {}
    """
    def construct(self):
        # Create title
        title = Text("AnimaForge", font_size=72)
        subtitle = Text("AI-Powered Animation", font_size=36)
        subtitle.next_to(title, DOWN)

        # Animate title
        self.play(Write(title))
        self.play(FadeIn(subtitle))
        self.wait(1)

        # Create and animate a circle
        circle = Circle(radius=2, color=BLUE)
        self.play(Create(circle))
        self.play(circle.animate.set_fill(BLUE, opacity=0.5))

        # Add some text
        text = Text("{}", font_size=24)
        text.move_to(circle.get_center())
        self.play(Write(text))

        self.wait(2)

        # Fade out everything
        self.play(
            FadeOut(title),
            FadeOut(subtitle),
            FadeOut(circle),
            FadeOut(text)
        )
"#,
        prompt, prompt
    )
}

fn validate_code(code: &str) -> Result<bool> {
    // Basic validation: check if it's valid Python and contains Manim imports
    let has_manim_import = code.contains("from manim import") || code.contains("import manim");
    let has_scene_class = code.contains("class") && code.contains("Scene");

    Ok(has_manim_import && has_scene_class)
}

fn determine_output_path(config: &Config, output: Option<String>) -> Result<PathBuf> {
    if let Some(path) = output {
        return Ok(PathBuf::from(path));
    }

    // Use output_dir from config or default
    let output_dir = config.output_dir.as_deref().unwrap_or("./animations");
    let output_dir = PathBuf::from(output_dir);

    // Create directory if it doesn't exist
    fs::create_dir_all(&output_dir)
        .context("Failed to create output directory")?;

    // Generate unique filename
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let filename = format!("animation_{}.py", timestamp);
    Ok(output_dir.join(filename))
}
