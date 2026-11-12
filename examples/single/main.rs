use clicmd::{
    CommandOnce,
    clap::Parser,
    cli_main,
    commands::{color::Color, verbosity::Verbosity},
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

impl CommandOnce for Cli {
    type Output = ExitCode;
    type Error = CliError;

    fn execute_once(self) -> Result<Self::Output, Self::Error> {
        self.verbosity.execute_once()?;
        self.color.execute_once();
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
