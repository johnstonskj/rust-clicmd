use clicmd::{
    Command,
    clap::Parser,
    cli_main,
    commands::{Verbosity, color::Color},
    error::CliError,
};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(flatten)]
    verbosity: Verbosity,

    #[command(flatten)]
    color: Color,

    message: Option<String>,
}

impl Command for Cli {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        self.verbosity.execute()?;
        self.color.execute();
         println!(
            "Message: {:?}",
            self.message.unwrap_or_else(|| "Hello world".into())
        );
        Ok(ExitCode::SUCCESS)
    }
}

fn main() -> ExitCode {
    cli_main::<Cli>()
}
