use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "push-backup",
    version,
    about = "Persist multiple git remote URLs and apply them to the current repo"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add or update a remote base URL in the local config
    Add { name: String, base: String },
    /// Remove a remote from the local config
    Remove { name: String },
    /// List saved remotes
    List,
    /// Show details of remotes (all if no name specified, or a specific one)
    Show { name: Option<String> },
    /// Apply saved remotes to the current git repository
    Apply { repo: String },
    /// Push current branch to all configured remotes
    Push {
        #[arg(short = 'd', long = "dry-run")]
        dry_run: bool,
    },
    /// Export configuration to a file
    Export {
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },
    /// Import configuration from a file
    Import {
        #[arg(short = 'i', long = "input")]
        input: PathBuf,
        #[arg(short = 'm', long = "merge")]
        merge: bool,
    },
}
