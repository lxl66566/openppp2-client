use std::{path::PathBuf, sync::LazyLock as Lazy};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The config file path for openppp2-client.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// The subcommand.
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    /// Run openppp2 without displaying the TUI menu.
    Use {
        /// Use default config with given ip and port, or a path to config file.
        /// e.g. `127.0.0.1:2777` or `openppp2-client.json`.
        config: String,
    },
}

pub static CLI: Lazy<Cli> = Lazy::new(Cli::parse);
