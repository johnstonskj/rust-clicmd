use clicmd::{
    CommandOnce, CommandOnceWith,
    clap::{Subcommand},
    cli_main,
    commands::{CliWithSubCommands, external::ExternalSubCommand},
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

    #[command(external_subcommand)]
    External(#[arg(num_args = 1..)] Vec<String>),
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
            MyCommands::External(args) => {
                ExternalSubCommand::from(args).execute_once_with("got".into())?;
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

fn main() -> ExitCode {
    cli_main::<Cli>()
}
