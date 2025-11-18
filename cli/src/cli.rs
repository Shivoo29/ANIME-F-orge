use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "animaforge")]
#[command(about = "AI-Powered Animation Creation Tool", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create animation from text prompt
    Create {
        /// Text description of the animation to create
        prompt: String,

        /// Automatically render after creation
        #[arg(short, long)]
        render: bool,

        /// Output file path (default: auto-generated)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Render animation from code file
    Render {
        /// Path to animation code file
        file: String,

        /// Rendering quality (low, medium, high)
        #[arg(short, long, default_value = "medium")]
        quality: String,

        /// Output video file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Manage CLI configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Publish animation to marketplace
    Publish {
        /// Path to animation file or directory
        file: String,
    },

    /// Search marketplace for animations
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set a configuration value
    Set {
        /// Configuration key (backend, model, api_key, output_dir)
        #[arg(long)]
        backend: Option<String>,

        #[arg(long)]
        model: Option<String>,

        #[arg(long)]
        api_key: Option<String>,

        #[arg(long)]
        output_dir: Option<String>,
    },

    /// Get a configuration value
    Get {
        /// Configuration key to retrieve
        key: Option<String>,
    },

    /// List all configuration values
    List,
}
