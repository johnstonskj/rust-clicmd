//!
//! This library provides some composable components for command-line applications.
//!
//! Rather than focus on the Cli structures as primary abstractions, this library emphasizes
//! composable *command* components. So actions in the application are represented as commands
//! which are then composed into initialization, command-dispatch, and cleanup, rather than being
//! tightly coupled to the CLI structure.
//!
//! # Usage
//!
//! ## Example Single Command CLI
//! 
//! Note that `clicmd` also re-exports some commonly used types and macros from `clap` to save the
//! client having to take an addition *explicit* dependency. Obviously, if you wish to use `clap`
//! directly, you still can.
//!
#![doc = concat!(
    "```rust\n",
    include_str!("../examples/single/main.rs"),
    "\n```"
)]
//!
//! See `examples/single/main.rs` for this complete example.
//!
//! ## Example Multi Command CLI
//!
//! In this example the client only need to define the `Subcommand` enum and the associated command
//! handlers, using the library's `CliWithSubCommands` abstraction to manage the subcommand
//! dispatch. This also takes care of the verbosity and color initialization.
//! 
#![doc = concat!(
    "```rust\n",
    include_str!("../examples/multi/main.rs"),
    "\n```"
)]
//!
//! See `examples/multi/main.rs` for this complete example.
//!
//! # Notes
//!
//! You will notice that some features such as `color` are optional and yet some aspects of the
//! associated components are still usable and composable within the application. This allows the
//! default behavior of the application to remain consistent regardless of which optional features,
//! and by implication exposed command-line options, are enabled.
//!
//! # Feature flags
#![doc = document_features::document_features!()]

use colorchoice::ColorChoice;
use std::{error::Error, process::ExitCode, sync::OnceLock};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

///
/// This macro wraps calls to commands, handling their execution and error reporting. It specifically
/// supports `Command`, `CommandWith`, and sequences of `Command` executions.
///
/// # Syntax
///
/// ```bnf
/// execute ::= "execute!" "("
///     ( command | command_with | command_sequence )
/// ")"
///
/// command ::= expr
/// command_with ::= command_name "(" argument ")"
/// command_sequence ::= command ( "," command )+
///
/// command_name ::= ident
/// argument ::= expr
/// ```
///
#[macro_export]
macro_rules! execute {
    ($cmd:expr) => {
        match $cmd.execute() {
            Ok(response) => response,
            Err(e) => {
                $crate::messages::Message::from_error(e, "", "", None).print();
                return ::std::process::ExitCode::FAILURE;
            }
        }
    };
    ($cmd:ident ( $arg:expr ) ) => {
        match $cmd.execute_with($arg) {
            Ok(response) => response,
            Err(e) => {
                $crate::messages::Message::from_error(e, "", "", None).print();
                return ::std::process::ExitCode::FAILURE;
            }
        }
    };
    ($cmd:expr $(, $then:expr )+ ) => {{
        execute!($cmd);
        $(
            execute!($then);
        )+
        ::std::process::ExitCode::SUCCESS
    }};
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A trait representing a command that can be executed once only, without additional arguments.
/// 
pub trait Command {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: Error;

    /// Executes the command, consuming self.
    fn execute(self) -> Result<Self::Output, Self::Error>;
}

///
/// A trait representing a command that can be executed once only, with a single argument.
/// 
pub trait CommandWith {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: Error;
    type Value: ?Sized;

    fn execute_with(self, value: Self::Value) -> Result<Self::Output, Self::Error>;
}

///
/// A specialized result type for CLI commands, returning an `ExitCode` on success and an error of
/// type `E` on failure.
/// 
pub type CliResult<E> = Result<ExitCode, E>;

// ------------------------------------------------------------------------------------------------
// Private Globals
// ------------------------------------------------------------------------------------------------

static VERBOSE_COUNT: OnceLock<u8> = OnceLock::new();
static QUIET_COUNT: OnceLock<u8> = OnceLock::new();

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Returns the number of times the verbose flag was included on the command line.
/// 
pub fn get_verbose_count() -> u8 {
    *VERBOSE_COUNT.get().unwrap_or(&0)
}

///
/// Sets the number of times the verbose flag was included on the command line.
/// 
pub(crate) fn set_quiet_count(count: u8) -> bool {
    QUIET_COUNT.set(count).is_ok()
}

///
/// Returns the number of times the quiet flag was included on the command line.
/// 
pub fn get_quiet_count() -> u8 {
    *QUIET_COUNT.get().unwrap_or(&0)
}

///
/// Sets the number of times the quiet flag was included on the command line.
/// 
pub(crate) fn set_verbose_count(count: u8) -> bool {
    VERBOSE_COUNT.set(count).is_ok()
}

///
/// Return the selected, or default, color choice.
/// 
pub fn get_color_choice() -> ColorChoice {
    colorchoice::ColorChoice::global()
}

///
/// This function serves as the main entry point for a CLI application.
/// 
/// This function should be called from the `main` function of your CLI application, using the
/// generic argument to denote the implementation of your clap-based CLI command.
/// 
/// ```rust,ignore
/// fn main() -> std::process::ExitCode {
///     clicmd::cli_main::<MyApplicationCli>()
/// }
/// ```
///
pub fn cli_main<C>() -> ExitCode
where
    C: clap::Parser + Command<Output = ExitCode>,
{
    execute!(C::parse())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

///
/// Re-export most commonly used items from `clap`.
/// This allows users to access commonly used `clap` items without directly depending on `clap`. 
/// 
pub mod clap {
    pub use clap::{ArgAction, Args, CommandFactory, Parser, Subcommand, ValueEnum};
}

pub mod commands;

pub mod error;

#[cfg(feature = "messages")]
pub mod messages;
