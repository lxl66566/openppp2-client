use std::{path::PathBuf, sync::LazyLock as Lazy};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The config file path for
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// The subcommand.
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    /// Run openppp2 without displaying the TUI menu.
    #[clap(visible_alias("i"))]
    Use {
        /// Use default config with given ip and port. Ex. `-d 127.0.0.1:2777`.
        #[arg(short, long)]
        #[clap(conflicts_with = "config")]
        default: Option<String>,
        /// The config file path for running openppp2.
        #[arg(short, long)]
        #[clap(conflicts_with = "default")]
        config: Option<PathBuf>,
    },
}

pub static CLI: Lazy<Cli> = Lazy::new(Cli::parse);
