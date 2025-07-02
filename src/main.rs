use clap::{Parser, Subcommand};
use gametorch::animations;
use serde_json;
use std::env;

/// GameTorch command-line interface.
///
/// This binary provides access to GameTorch functionality via the `gametorch` command.
/// Your API key is loaded from the `GAMETORCH_API_KEY` environment variable.
#[derive(Parser)]
#[command(name = "gametorch")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Use local server (http://localhost:8000) instead of production.
    #[arg(short = 'l', long = "local", global = true)]
    local: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Animation-related operations
    Animations {
        #[command(subcommand)]
        action: AnimationCommands,
    },
}

#[derive(Subcommand)]
pub enum AnimationCommands {
    /// Retrieve an existing animation
    Get {
        /// The identifier of the animation to fetch
        id: String,
    },
    /// Generate a new animation
    Generate {
        /// The prompt or parameters used for generation
        prompt: String,
        /// Block until rendering finishes and download ZIP.
        #[arg(short = 'b', long = "block")]
        block: bool,
        /// Output file for the resulting ZIP when using --block.
        #[arg(short = 'o', long = "output-file")]
        output_file: Option<String>,
        /// Optional input image file path to include in generation
        #[arg(short = 'i', long = "input-image", value_name = "FILE")]
        input_image: Option<String>,
        /// Optional animation model ID (defaults to 6)
        #[arg(long = "model-id", value_name = "ID", conflicts_with = "model_name")]
        model_id: Option<u32>,
        /// Optional animation model name (defaults to 'alpha/v2.1')
        #[arg(long = "model-name", value_name = "NAME", conflicts_with = "model_id")]
        model_name: Option<String>,
        /// Suppress informational logs
        #[arg(short = 's', long = "silent")]
        silent: bool,
        /// Duration in seconds (allowed values: 5 or 10, defaults to 5)
        #[arg(short = 'd', long = "duration", value_name = "SECONDS", default_value_t = 5)]
        duration: u32,
    },
    /// Crop an animation
    Crop {
        /// Input file or animation identifier
        input: String,
        /// (Optional) Output file path. If omitted, will overwrite input or write to default.
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Retrieve API key from environment variable
    let api_key = env::var("GAMETORCH_API_KEY").unwrap_or_else(|_| {
        eprintln!(
            "Error: environment variable GAMETORCH_API_KEY not set.\n\
            Please set it before using this CLI."
        );
        std::process::exit(1);
    });

    // Determine base URL depending on --local flag
    let base_url = if cli.local {
        "http://localhost:8000"
    } else {
        "https://gametorch.app"
    };

    // Dispatch based on the parsed commands
    match cli.command {
        Commands::Animations { action } => match action {
            AnimationCommands::Get { id } => {
                match animations::get(&api_key, base_url, &id).await {
                    Ok(json) => {
                        println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    }
                    Err(err) => {
                        eprintln!("Failed to fetch animation: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            AnimationCommands::Generate { prompt, block, output_file, input_image, model_id, model_name, silent, duration } => {
                match animations::generate(&api_key, base_url, &prompt, duration, block, output_file.as_deref(), input_image.as_deref(), model_id, model_name.as_deref(), silent).await {
                    Ok(json) => {
                        println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    }
                    Err(err) => {
                        eprintln!("Failed to generate animation: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            AnimationCommands::Crop { input, output } => {
                println!("[stub] Cropping animation from '{}' to '{:?}' with API key: {} (base URL: {})", input, output, api_key, base_url);
            }
        },
    }
}
