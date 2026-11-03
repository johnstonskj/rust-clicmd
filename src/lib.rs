//!
//! This library provides some composable components for command-line applications.
//!
//! Rather than focus on the Cli structures as primary abstractions, this library emphasizes
//! composable *command* components. So actions in the application are represented as commands
//! which are then composed into initialization, command-dispatch, and cleanup, rather than being
//! tightly coupled to the CLI structure.
//!
//! You will notice that some features such as `color` and `telemetry` are optional and yet the
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
// Globals
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

pub fn cli_main<C>() -> ExitCode
where
    C: clap::Parser + Command<Output = ExitCode>,
{
    execute!(C::parse())
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod commands;

pub mod error;

#[cfg(feature = "messages")]
pub mod messages;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {}
