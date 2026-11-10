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
//! TBD
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

pub trait Command {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: Error;

    /// Executes the command, consuming self.
    fn execute(self) -> Result<Self::Output, Self::Error>;
}

pub trait CommandWith {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: Error;
    type Value: ?Sized;

    fn execute_with(self, value: Self::Value) -> Result<Self::Output, Self::Error>;
}

pub type CliResult<E> = Result<ExitCode, E>;

// ------------------------------------------------------------------------------------------------
// Private Globals
// ------------------------------------------------------------------------------------------------

static VERBOSE_COUNT: OnceLock<u8> = OnceLock::new();
static QUIET_COUNT: OnceLock<u8> = OnceLock::new();

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_verbose_count() -> u8 {
    *VERBOSE_COUNT.get().unwrap_or(&0)
}

pub(crate) fn set_quiet_count(count: u8) -> bool {
    QUIET_COUNT.set(count).is_ok()
}

pub fn get_quiet_count() -> u8 {
    *QUIET_COUNT.get().unwrap_or(&0)
}

pub(crate) fn set_verbose_count(count: u8) -> bool {
    VERBOSE_COUNT.set(count).is_ok()
}

pub fn get_color_choice() -> ColorChoice {
    colorchoice::ColorChoice::global()
}

///
/// TBD
/// 
/// Note, the call to `C::execute` uses the macro `execute!` , and the error handling it provides.
/// 
/// # Example Single Command CLI
/// 
/// ```rust
/// use clap::{Parser, ValueEnum};
/// use clicmd::{Command, commands::Verbosity, error::CliError, cli_main};
/// use colorchoice_clap::Color;
/// use std::process::ExitCode;
/// 
/// #[derive(Debug, Parser)]
/// #[command(version, about, long_about = None)] // Read from `Cargo.toml`
/// pub struct Cli {
///     #[command(flatten)]
///     verbosity: Verbosity,
/// 
///     #[command(flatten)]
///     color: Color,
/// 
///     message: Option<String>,
/// }
/// 
/// impl Command for Cli {
///     type Output = ExitCode;
///     type Error = CliError;
///
///     fn execute(self) -> Result<Self::Output, Self::Error> {
///         println!(
///             "Message: {:?}", 
///             self.message
///                 .unwrap_or_else(|| "Hello world".into())
///         );
///         Ok(ExitCode::SUCCESS)
///     }
/// }
/// 
/// fn main() -> ExitCode {
///     cli_main::<Cli>()
/// }
/// ``` 
/// 
/// See `examples/single/src/main.rs` for a complete example.
/// 
/// # Example Multi Command CLI
/// 
/// ```rust,ignore
/// use clicmd::cli_main;
/// use std::process::ExitCode;
/// 
/// fn main() -> ExitCode {
///     cli_main::<Cli>()
/// }
/// ```
/// 
/// See `examples/multi/src/main.rs` for a complete example.
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

pub mod commands;

pub mod error;

#[cfg(feature = "messages")]
pub mod messages;
