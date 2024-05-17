use std::path::PathBuf;

use clap::Parser;
use once_cell::sync::Lazy;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// directories that place openppp2 config json file
    #[arg(short, long)]
    pub config_dir: Vec<PathBuf>,
}

pub static CLI: Lazy<Cli> = Lazy::new(|| {
    let mut t = Cli::parse();
    t.config_dir.push(PathBuf::from("."));
    t
});
