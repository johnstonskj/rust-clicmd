use clicmd::{
    CommandOnce, clap::Subcommand, cli_main, commands::CliWithSubCommands, error::CliError,
};
use std::process::ExitCode;

#[derive(Debug, Subcommand)]
pub enum GreetingCommands {
    /// Print a message
    Hello {
        /// The message to print
        message: String,
    },
    /// Print a message
    Goodbye {
        /// The message to print
        message: String,
    },
}

type Cli = CliWithSubCommands<GreetingCommands>;

impl CommandOnce for GreetingCommands {
    type Output = ExitCode;
    type Error = CliError;

    fn execute_once(self) -> Result<Self::Output, Self::Error> {
        match self {
            GreetingCommands::Hello { message } => {
                println!("Hello: {}", message);
            }
            GreetingCommands::Goodbye { message } => {
                println!("Goodbye: {}", message);
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

fn main() -> ExitCode {
    cli_main::<Cli>()
}
