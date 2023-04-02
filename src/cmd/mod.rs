//! commands
use crate::Result;
use clap::{Parser, Subcommand};

mod key;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Command,
}

impl Cli {
    /// Run the cli
    pub fn run() -> Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Command::Key(key) => key.run()?,
        }

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(subcommand)]
    Key(key::Key),
}
