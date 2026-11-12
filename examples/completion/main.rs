use clicmd::{
    CommandOnce, CommandOnceWith,
    clap::{CommandFactory, Subcommand},
    cli_main,
    commands::{CliWithSubCommands, completion::CompletionSubCommand},
    error::CliError,
};
use std::process::ExitCode;

#[derive(Debug, Subcommand)]
pub enum MyCommands {
    /// Add a file to the staging area
    Add {
        /// The file to add
        file: String,
    },
    /// Commit changes to the repository
    Commit {
        /// The message to include with the commit
        message: String,
    },

    Completions(CompletionSubCommand),
}

type Cli = CliWithSubCommands<MyCommands>;

impl CommandOnce for MyCommands {
    type Output = ExitCode;
    type Error = CliError;

    fn execute_once(self) -> Result<Self::Output, Self::Error> {
        match self {
            MyCommands::Add { file } => {
                println!("Add: {}", file);
            }
            MyCommands::Commit { message } => {
                println!("Commit: {}", message);
            }
            MyCommands::Completions(cmd) => {
                cmd.execute_once_with(<Cli as CommandFactory>::command());
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

fn main() -> ExitCode {
    cli_main::<Cli>()
}
