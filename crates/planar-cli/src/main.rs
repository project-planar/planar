use std::path::PathBuf;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use console::{style, Emoji};

use crate::settings::get_registry;

mod setup;
mod settings;
mod init;
mod build;
mod global;
mod inspect;


static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");

#[derive(Parser)]
#[command(name = "planar")]
#[command(about = "Polyglot Semantic Intelligence Platform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    
    /// Install or update grammars
    Setup {
        #[arg(short, long)]
        force: bool,
    },
    /// Initialize a new Planar project
    Init {
        /// Project name (optional, defaults to current directory)
        name: Option<String>,
    },

    Build {
        /// Path to the project root
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Verbosity level: -v (DEBUG), -vv (TRACE)
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,

    },

    /// Manage global configuration
    Global {
        #[command(subcommand)]
        action: GlobalAction,
    },

    /// Inspect a compiled .pdla artifact
    Inspect {
        /// Path to the .pdla file
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum GlobalAction {
    /// Set a configuration value (keys: std-path, cache-dir, token)
    Set { key: String, value: String },
    /// List all global configuration values
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            init::run(name)?; 
        }
        Commands::Setup { force } => {
            println!("{} {}Setup Planar environment...", style("planar").bold().cyan(), LOOKING_GLASS);
            setup::run(force, get_registry()?).await?;
        }
        Commands::Build { path, verbose } => {
            init_tracing(verbose); 
            build::run(path, verbose > 0).map_err(|e| anyhow!(e))?;
        }
        Commands::Global { action } => match action {
            GlobalAction::Set { key, value } => global::run_set(key, value)?,
            GlobalAction::List => global::run_list()?,
        }
        Commands::Inspect { path } => {
            inspect::run(path)?;
        }
    }

    Ok(())
}


fn init_tracing(verbosity: u8) {
    if verbosity == 0 { return; }

    let level = match verbosity {
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_file(verbosity > 1) 
        .with_line_number(verbosity > 1)
        .with_target(true) 
        .init();
}
