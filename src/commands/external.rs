//!
//! Provides the external subcommand functionality for CLI commands.
//!
//! This module wraps the `clap` external subcommand functionality to integrate with the `Command`
//! trait. It provides a default implementation for handling external subcommands which will also
//! pass on the same verbosity and color flags to the sub-command.
//!
//! Note that `ExternalSubCommand` implements both the `Command` and `CommandWith` traits, allowing
//! it to be executed directly or with a specified prefix. The intent of the prefix is to support
//! the convention where sub-command binary names are composed of the primary command name followed
//! by a hyphen and the sub-command name.
//! 
//! # Examples
//! 
//! ```rust
//! use clicmd::{Command, commands::external::ExternalSubCommand}; 
//! 
//! let cmd = ExternalSubCommand {
//!     args: vec!["ls".into()],
//! };
//! cmd.execute().unwrap();
//! ```
//!

use crate::{
    Command, CommandWith,
    error::CliError,
    get_color_choice, get_quiet_count, get_verbose_count,
    messages::{Message, Note},
};
use colorchoice::ColorChoice;
use search_path::SearchPath;
use std::{
    path::{Path, PathBuf},
    process::{Command as OsCommand, ExitCode},
};
use tracing::{error, info};
use clap::Args;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// External sub-command handling for commands not natively supported by the CLI.
/// 
/// The array of values **must** contain at least one element, representing the external sub-command
/// executable name to run, all other values are arguments to that executable.
/// 
/// # Example
/// 
/// ```rust
/// use clap::Subcommand;
/// use clicmd::commands::external::ExternalSubCommand;
///
/// #[derive(Debug, Subcommand)]
/// #[command(allow_external_subcommands = true)]
/// pub enum Commands {
///     // ...
///
///     External(
///         ExternalSubCommand
///     ),
/// }
/// ```
/// 
#[derive(Debug, Args)]
pub struct ExternalSubCommand {
    #[arg(num_args = 1..)]
    pub args: Vec<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Command for ExternalSubCommand {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<ExitCode, CliError> {
        self.execute_inner(None)
    }
}

impl CommandWith for ExternalSubCommand {
    type Output = ExitCode;
    type Error = CliError;
    type Value = String;

    fn execute_with(self, prefix_name: String) -> Result<ExitCode, CliError> {
        self.execute_inner(Some(prefix_name))
    }
}

impl ExternalSubCommand {
    fn execute_inner(self, prefix_name: Option<String>) -> Result<ExitCode, CliError> {
        let command_name = if let Some(prefix) = prefix_name {
            format!("{prefix}-{}", self.args[0])
        } else {
            self.args[0].clone()
        };

        let search_path = SearchPath::path().unwrap();
        if let Some(exec_path) = search_path.find_file(&PathBuf::from(&command_name)) {
            let mut arguments = Vec::new();

            let verbosity = get_verbose_count();
            if verbosity > 0 {
                arguments.push(format!("-{}", "v".repeat(verbosity as usize)))
            }

            let quietness = get_quiet_count();
            if quietness > 0 {
                arguments.push(format!("-{}", "q".repeat(quietness as usize)))
            }

            let color_choice = get_color_choice();
            if color_choice != ColorChoice::Auto {
                arguments.push(format!(
                    "--color {}",
                    if color_choice != ColorChoice::Always {
                        "always"
                    } else {
                        "never"
                    }
                ));
            }

            arguments.extend(self.args[1..].iter().cloned());

            info!(
                "executing command `{:?}`, with arguments {:?}",
                exec_path, arguments
            );

            match OsCommand::new(exec_path.clone())
                .args(arguments.as_slice())
                .status()
            {
                Ok(status) => {
                    info!("sub-process {:?} returned status {status}", &exec_path);
                    Ok(if status.success() {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    })
                }
                Err(e) => {
                    error!("sub-process {exec_path:?} returned error {e}");
                    sub_process_error(exec_path, e).print();
                    Ok(ExitCode::FAILURE)
                }
            }
        } else {
            missing_executable(command_name).print();
            Ok(ExitCode::FAILURE)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn missing_executable<S: AsRef<str>>(name: S) -> Message {
    Message::error("no executable found for external command")
        .with_context(format!("Command `{}`", name.as_ref()))
        .with_note(Note::help(format!(
            "check that the executable `rfham-{}` exists and is in $PATH",
            name.as_ref()
        )))
}

fn sub_process_error<P: AsRef<Path>, E: std::error::Error>(path: P, error: E) -> Message {
    Message::error("an error occured trying to run a sub-process")
        .with_context(format!("Executable path `{:?}`", path.as_ref()))
        .with_note(Note::note(format!("reported error: {error}")))
}
