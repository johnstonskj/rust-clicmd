//!
//! Generate shell completions based on clap commands.
//!
//! This wraps the functionality of the `clap_complete` crate in the same command style as the rest
//! of the CLI commands.
//!

use crate::{
    CommandOnceWith,
    clap::{Args, Command},
};
use clap_complete::{Shell, generate};
use std::{convert::Infallible, io, process::ExitCode};
use tracing::{info, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Command-Line mixin that provides the `completion` subcommand.
///
/// # Example
///
/// The key element in the example below is the call to `execute_once_with` on the 
/// `CompletionSubCommand`. The command needs to know what the command line looks like and so it
/// takes the full CLI command as an argument, and the `clap::CommandFactory` trait is used to
/// construct the command passed as the argument.
/// 
/// ```rust
/// # use clicmd::{
/// #     CommandOnce,
/// #     CommandOnceWith,
/// #     clap::{CommandFactory, Subcommand},
/// #     commands::{CliWithSubCommands, completion::CompletionSubCommand},
/// #     error::CliError,
/// # };
/// #[derive(Debug, Subcommand)]
/// pub enum MyCommands {
///     // ...
///     Completions(CompletionSubCommand),
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
///             MyCommands::Completions(cmd) => {
///                 cmd.execute_once_with(<Cli as CommandFactory>::command());
///             }
///         }
///         Ok(ExitCode::SUCCESS)
///     }
/// }
/// ```
///
/// See the code in `examples/completion/main.rs` for a complete working example.
///
#[derive(Debug, Default, Args)]
pub struct CompletionSubCommand {
    /// Shell to generate completions for (defaults to $SHELL).
    shell: Option<Shell>,
}

impl CommandOnceWith for CompletionSubCommand {
    type Output = ExitCode;
    type Error = Infallible;
    type Value = Command;

    fn execute_once_with(self, mut command: Self::Value) -> Result<Self::Output, Self::Error> {
        trace!("CompletionCommand::execute_once_with({self:?})");
        
        let shell = self.shell.unwrap_or_else(|| {
            // Try to detect from $SHELL
            std::env::var("SHELL")
                .ok()
                .and_then(|s| {
                    let basename = std::path::Path::new(&s)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                    basename.parse::<Shell>().ok()
                })
                .unwrap_or(Shell::Bash)
        });

        info!("creating completions for shell `{shell:?}`");
        let name = command.get_name().to_string();
        generate(shell, &mut command, name, &mut io::stdout());

        Ok(ExitCode::SUCCESS)
    }
}
