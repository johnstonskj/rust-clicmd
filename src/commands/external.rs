//!
//! Provides the external subcommand functionality for CLI commands.
//!
//! This module wraps the `clap` external subcommand functionality to integrate with the
//! [`CommandOnce`] trait. It provides a default implementation for handling external subcommands
//! which will also pass on the same verbosity and color flags to the sub-command.
//!
//! Note that [ExternalSubCommand] implements both the [CommandOnce] and [CommandOnceWith] traits,
//! allowing it to be executed directly or with a specified prefix. The intent of the prefix is to
//! support the convention where sub-command binary names are composed of the primary command name
//! followed by a hyphen and the sub-command name.
//!
//! # Examples
//! 
//! In this case the external subcommand `ls` will be executed with the `--color` argument.
//!
//! ```rust
//! use clicmd::{CommandOnce, commands::external::ExternalSubCommand};
//!
//! let cmd = ExternalSubCommand::from(
//!     vec!["ls".into(), "--color".into()]
//! ).execute_once().unwrap();
//! ```
//!
//! In this case the external subcommand `my-ls` will be executed with the `--color` argument.
//! 
//! ```rust
//! use clicmd::{CommandOnceWith, commands::external::ExternalSubCommand};
//!
//! ExternalSubCommand::from(
//!     vec!["ls".into(), "--color".into()]
//! ).execute_once_with("my".into()).unwrap();
//! ```
//!

use crate::{
    CommandOnce, CommandOnceWith,
    error::CliError,
    get_color_choice, get_quiet_count, get_verbose_count,
    messages::{Message, Note},
};
use colorchoice::ColorChoice;
use search_path::SearchPath;
use std::{
    ffi::OsStr,
    path::PathBuf,
    process::{Command as OsCommand, ExitCode},
};
use tracing::{error, info};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! external_command {
    () => {
        #[command(external_subcommand)]
        External (
            #[arg(num_args = 1..)]
            Vec<String>,
        ),
   };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Command-Line mixin that provides external subcommand functionality.
///
/// The array of values **must** contain at least one element, representing the external sub-command
/// executable name to run, all other values are arguments to that executable.
///
/// # Example
/// 
/// There are a couple of important points to note when using external subcommands:
/// 
/// 1. The command itself has to be present in the format shown, with the `external_subcommand`
///    attribute and with *only* the string vector.
/// 2. The first element in the array represents the external sub-command executable name, hence
///    the `num_args` constraint, and subsequent elements are treated as arguments to that
///    executable.
/// 3. The `ExternalSubCommand` struct is used to wrap the arguments and execute the external 
///    command, and is easily constructed using the `From` trait with the vector of strings from 
///    above.
/// 4. This command implements both `execute_once` and `execute_once_with`, the difference being
///    that the latter form takes a string that will be a prefix used to construct the executable
///    name.
///
/// ```rust
/// # use clap::Subcommand;
/// # use clicmd::{
/// #     CommandOnce, 
/// #     CommandOnceWith, 
/// #     error::CliError, 
/// #     commands::{CliWithSubCommands, external::ExternalSubCommand},
/// # };
/// # use std::process::ExitCode;
/// #[derive(Debug, Subcommand)]
/// #[command(allow_external_subcommands = true)]
/// pub enum MyCommands {
///     // ...
///
///     #[command(external_subcommand)]
///     External(#[arg(num_args = 1..)] Vec<String>),
/// }
/// 
/// type Cli = CliWithSubCommands<MyCommands>;
/// 
/// impl CommandOnce for MyCommands {
///     type Output = ExitCode;
///     type Error = CliError;
/// 
///     fn execute_once(self) -> Result<Self::Output, Self::Error> {
///         match self {
///             // ...
///             MyCommands::External(args) => {
///                 ExternalSubCommand::from(args) 
///                     .execute_once_with("got".into())?;
///             }
///         }
///         Ok(ExitCode::SUCCESS)
///     }
/// }
/// ```
/// 
/// See the code in `examples/got/main.rs` for a complete working example.
///
/// Alternatively, to save hard coding the prefix string, you can dynamically obtain the command
/// name from the CLI command factory.
/// 
/// ```rust,ignore
/// let command = <Cli as CommandFactory>::command();
/// let name = command.get_name();
/// ExternalSubCommand::from(args).execute_once_with(name.into())?;
/// ```
///
/// Or, you can use the crate name as the prefix.
/// 
/// ```rust,ignore
/// const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
/// ExternalSubCommand::from(args).execute_once_with(CRATE_NAME.into())?;
/// ```
/// 
#[derive(Debug)]
pub struct ExternalSubCommand {
    args: Vec<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Vec<String>> for ExternalSubCommand {
    fn from(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl CommandOnce for ExternalSubCommand {
    type Output = ExitCode;
    type Error = CliError;

    fn execute_once(self) -> Result<ExitCode, CliError> {
        self.execute_inner(None)
    }
}

impl CommandOnceWith for ExternalSubCommand {
    type Output = ExitCode;
    type Error = CliError;
    type Value = String;

    fn execute_once_with(self, prefix_name: String) -> Result<ExitCode, CliError> {
        self.execute_inner(Some(&prefix_name))
    }
}

impl ExternalSubCommand {
    fn execute_inner(self, prefix_name: Option<&String>) -> Result<ExitCode, CliError> {
        match self.to_process_command(prefix_name) {
            Ok(mut command) => match command.status() {
                Ok(status) => {
                    info!("sub-process returned status {status}");
                    Ok(if status.success() {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    })
                }
                Err(e) => {
                    error!("sub-process execution failed with error {e}");
                    sub_process_error(&command.get_program(), e).print();
                    Ok(ExitCode::FAILURE)
                }
            },
            Err(_) => Ok(ExitCode::FAILURE),
        }
    }
}

impl ExternalSubCommand {
    pub fn to_process_command(&self, prefix_name: Option<&String>) -> Result<OsCommand, CliError> {
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

            let mut command = OsCommand::new(exec_path.clone());
            command.args(arguments.as_slice());
            Ok(command)
        } else {
            missing_executable(&command_name).print();
            Err(CliError::SubProcessExecutableMissing(PathBuf::from(
                command_name,
            )))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn missing_executable(name: &str) -> Message {
    Message::error("no executable found for external command")
        .with_context(format!("exectable name: `{}`", name))
        .with_note(Note::help(format!(
            "check that the executable `{}` exists and is in $PATH",
            name
        )))
}

fn sub_process_error<E: std::error::Error>(path: &OsStr, error: E) -> Message {
    Message::error("an error occured trying to run a sub-process")
        .with_context(format!("Executable path `{:?}`", path))
        .with_note(Note::note(format!("reported error: {error}")))
}
