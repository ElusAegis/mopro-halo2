use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod build;
mod export;
mod init;
mod test;
mod update;
mod utils;

/// CLI for multi-platform project management
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initializes a new project
    Init {
        #[arg(long, default_value = "circom")]
        adapter: String,
        #[arg(long, num_args = 1.., default_value = "core")]
        platforms: Vec<String>,
        #[arg(long, default_value = "mopro-example-app")]
        project_name: String,
    },
    /// Builds the project for specified platforms
    Build {
        #[arg(long, default_value = "mopro-config.toml")]
        config: String,
        #[arg(long, default_value = "circom")]
        adapter: String,
        #[arg(long, num_args = 1.., default_value = "core")]
        platforms: Vec<String>,
    },
    /// Updates bindings for the specified platforms
    Update {
        #[arg(long, default_value = "mopro-config.toml")]
        config: String,
        #[arg(long, default_value = "circom")]
        adapter: String,
        #[arg(long, num_args = 1.., default_value = "core")]
        platforms: Vec<String>,
    },
    /// Runs tests for the specified platform and test cases
    Test {
        #[arg(long, default_value = "mopro-config.toml")]
        config: String,
        #[arg(long, default_value = "circom")]
        adapter: String,
        #[arg(long, num_args = 1.., default_value = "core")]
        platforms: Vec<String>,
        #[arg(long)]
        test_case: Option<String>,
    },
    /// Exports platform bindings to some other directory
    ExportBindings {
        #[arg(short, long)]
        destination: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            adapter,
            platforms,
            project_name,
        } => init::init_project(adapter, platforms, project_name),
        Commands::Build {
            config,
            adapter,
            platforms,
        } => build::build_project(config, adapter, platforms),
        Commands::Update {
            config,
            adapter,
            platforms,
        } => update::update_project(config, adapter, platforms),
        Commands::Test {
            config,
            adapter,
            platforms,
            test_case,
        } => test::test_project(config, adapter, platforms, test_case),
        Commands::ExportBindings { destination } => export::export_bindings(destination),
    }
}
