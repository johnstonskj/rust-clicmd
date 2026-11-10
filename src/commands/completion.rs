//!
//! Generate shell completions based on clap commands.
//!
//! This wraps the `clap_complete` crate in the same command style as the rest of the CLI commands.
//!

use clap::{Args, CommandFactory};
use clap_complete::{Shell, generate};
use std::{convert::Infallible, io, process::ExitCode};
use tracing::{info, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generate command completions for a number of shells.
///
/// # Example
/// 
/// ```rust
/// use clap::Subcommand;
/// use clicmd::commands::completion::CompletionSubCommand;
///
/// #[derive(Debug, Subcommand)]
/// pub enum Commands {
///     // ...
///
///     Completions(CompletionSubCommand),
/// }
/// ```
/// 
#[derive(Debug, Args)]
pub struct CompletionSubCommand {
    /// Shell to generate completions for (defaults to $SHELL).
    shell: Option<Shell>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// The command type `CompletionSubCommand` does not implement `Command` or `CommandWith`, as usual
/// due to type constraints with the parameter `C: CommandFactory`. Instead, this function takes a
/// reference to `CompletionSubCommand` and a type implementing `CommandFactory`, and so mimics the
/// method on `CommandWith` to keep client code relatively consistent.
///
/// # Example
///  
/// TBD
///
pub fn execute_with<C: CommandFactory>(
    cmd: &CompletionSubCommand,
    _: C,
) -> Result<ExitCode, Infallible> {
    trace!("execute_with(cmd: {cmd:?})");
    let shell = cmd.shell.unwrap_or_else(|| {
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

    let mut cmd = C::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut io::stdout());
    Ok(ExitCode::SUCCESS)
}
