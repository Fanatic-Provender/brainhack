use {clap::Parser, std::path::PathBuf};

#[derive(Debug, Parser)]
pub struct Cli {
    pub file: PathBuf,
}
