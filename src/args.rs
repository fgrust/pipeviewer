use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Read from a file instead of stdin
    pub infile: Option<PathBuf>,

    /// Write output to a file instead of stdout
    #[arg(short, long, value_name = "FILE")]
    pub outfile: Option<PathBuf>,

    /// Silent option
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub silent: bool,
}
