use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub async fn execute(file: String, quality: String, output: Option<String>) -> Result<()> {
    println!("{}", "Rendering animation...".bright_green().bold());
    println!("{} {}", "Input file:".bright_yellow(), file.bright_cyan());
    println!("{} {}\n", "Quality:".bright_yellow(), quality.bright_cyan());

    // Validate input file exists
    let input_path = PathBuf::from(&file);
    if !input_path.exists() {
        anyhow::bail!("Animation file not found: {}", file);
    }

    // Validate file is Python
    if input_path.extension().and_then(|s| s.to_str()) != Some("py") {
        anyhow::bail!("Input file must be a Python (.py) file");
    }

    // Determine quality settings
    let quality_settings = match quality.as_str() {
        "low" => QualitySettings {
            resolution: "480p15",
            flag: "-ql",
        },
        "medium" => QualitySettings {
            resolution: "720p30",
            flag: "-qm",
        },
        "high" => QualitySettings {
            resolution: "1080p60",
            flag: "-qh",
        },
        _ => {
            anyhow::bail!("Invalid quality setting. Use: low, medium, or high");
        }
    };

    // Determine output path
    let output_path = determine_output_path(&input_path, output)?;

    println!(
        "{} Rendering to: {}\n",
        "→".bright_blue(),
        output_path.display().to_string().bright_cyan()
    );

    // Create progress bar
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}% {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Call Python engine to render
    let result = render_with_manim(&input_path, &output_path, &quality_settings, &pb)?;

    pb.finish_with_message("Rendering complete!".to_string());

    if result {
        println!(
            "\n{} Animation rendered successfully!",
            "✓".green().bold()
        );
        println!(
            "{} Output: {}",
            "→".bright_blue(),
            output_path.display().to_string().bright_cyan()
        );
    } else {
        anyhow::bail!("Rendering failed");
    }

    Ok(())
}

struct QualitySettings {
    resolution: &'static str,
    flag: &'static str,
}

fn render_with_manim(
    input: &Path,
    output: &Path,
    quality: &QualitySettings,
    pb: &ProgressBar,
) -> Result<bool> {
    pb.set_message("Initializing render...");

    // Check if manim is available
    let manim_check = Command::new("manim")
        .arg("--version")
        .output();

    if manim_check.is_err() {
        pb.finish_with_message("Manim not found!".to_string());
        anyhow::bail!(
            "Manim is not installed. Install it with: pip install manim\n\
             Or use the Python engine: cd ../engine && pip install -e ."
        );
    }

    // Extract the scene name from the Python file
    let scene_name = extract_scene_name(input)?;

    // Prepare output directory
    let output_dir = output.parent().unwrap_or(Path::new("."));
    fs::create_dir_all(output_dir)
        .context("Failed to create output directory")?;

    pb.set_message(format!("Rendering scene: {}", scene_name));

    // Run manim render command
    let mut child = Command::new("manim")
        .arg("render")
        .arg(quality.flag)
        .arg(input)
        .arg(&scene_name)
        .arg("-o")
        .arg(output.file_name().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to start manim render")?;

    // Read output and update progress
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                // Parse progress from manim output
                if line.contains("%") {
                    if let Some(percent) = extract_percentage(&line) {
                        pb.set_position(percent);
                    }
                }
                pb.set_message(line.trim().to_string());
            }
        }
    }

    let status = child.wait().context("Failed to wait for manim process")?;

    Ok(status.success())
}

fn extract_scene_name(file: &Path) -> Result<String> {
    let content = fs::read_to_string(file)
        .context("Failed to read animation file")?;

    // Look for class definitions that inherit from Scene
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("class ") && (line.contains("Scene") || line.contains("(Scene)")) {
            // Extract class name
            if let Some(class_name) = line
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.split('(').next())
            {
                return Ok(class_name.to_string());
            }
        }
    }

    anyhow::bail!("No Scene class found in animation file")
}

fn extract_percentage(line: &str) -> Option<u64> {
    // Try to find percentage in format like "50%" or "Rendering: 50%"
    for part in line.split_whitespace() {
        if part.ends_with('%') {
            if let Ok(num) = part.trim_end_matches('%').parse::<u64>() {
                return Some(num);
            }
        }
    }
    None
}

fn determine_output_path(input: &Path, output: Option<String>) -> Result<PathBuf> {
    if let Some(path) = output {
        return Ok(PathBuf::from(path));
    }

    // Default: same directory as input, with .mp4 extension
    let mut output_path = input.to_path_buf();
    output_path.set_extension("mp4");

    Ok(output_path)
}
