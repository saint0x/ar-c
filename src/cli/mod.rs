pub mod new;
pub mod build;
pub mod upload;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::compiler::AriaCompiler;

/// Common CLI utilities and shared functionality
pub struct CliConfig {
    pub verbose: bool,
    pub quiet: bool,
}

impl CliConfig {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }
}

/// Print status message with proper formatting
pub fn print_status(status: &str, message: &str) {
    println!("    {} {}", 
        console::style(status).bold().green(), 
        message
    );
}

/// Print error message with proper formatting
pub fn print_error(message: &str) {
    eprintln!("    {} {}", 
        console::style("error").bold().red(), 
        message
    );
}

/// Print warning message with proper formatting
pub fn print_warning(message: &str) {
    println!("    {} {}", 
        console::style("warning").bold().yellow(), 
        message
    );
}

/// Print info message with proper formatting
pub fn print_info(message: &str) {
    println!("    {} {}", 
        console::style("info").bold().blue(), 
        message
    );
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Builds the project into an .aria bundle
    Build {
        #[clap(default_value = ".")]
        path: String,

        #[clap(short, long)]
        output: Option<String>,

        #[clap(short, long)]
        verbose: bool,
    },
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { path, output, verbose } => {
            let compiler = AriaCompiler::new();
            let output_path = output.clone().map(std::path::PathBuf::from).unwrap_or_else(|| {
                // Default output logic
                let dir_name = std::path::Path::new(path).file_name().and_then(|n| n.to_str()).unwrap_or("bundle");
                std::path::PathBuf::from(format!("dist/{}.aria", dir_name))
            });

            print_info(&format!("Building project in '{}'", path));
            if *verbose {
                print_info(&format!("Outputting to '{}'", output_path.display()));
            }

            match compiler.compile_project(path, &output_path, *verbose).await {
                Ok(result) => {
                    print_status("Finished", "Build completed successfully.");
                    print_info(&format!("Bundle created: {}", output_path.display()));
                    print_info(&format!("  - Tools: {}", result.tools_count));
                    print_info(&format!("  - Agents: {}", result.agents_count));
                }
                Err(e) => {
                    print_error(&format!("Build failed: {}", e));
                    return Err(e);
                }
            }
        }
    }
    Ok(())
} 