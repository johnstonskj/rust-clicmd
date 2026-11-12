//!
//! This module, and sub-modules provide actual command implementations for CLI construction.
//!
//! - [`CliWithSubCommands`]: A generic top-level command structure that supports sub-commands,
//!   verbosity, and color options.
//!   - [Verbosity]: A structure for handling verbose and quiet flags.
//! - [`ExternalSubCommand`](crate::commands::external::ExternalSubCommand): A sub-command that can
//!   be executed as a child process by the main CLI.
//! - [`Completion`](crate::commands::completion::CompletionSubCommand): A sub-command that handles
//!   shell completion for the CLI.
//!
//!

use crate::{
    CommandOnce,
    clap::{Parser, Subcommand},
    commands::{color::Color, verbosity::Verbosity},
    error::CliError,
};
use std::{fmt::Debug, process::ExitCode};
use tracing::trace;
use tracing_subscriber::filter::LevelFilter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct CliWithSubCommands<S>
where
    S: Debug + Subcommand + CommandOnce<Output = ExitCode, Error = CliError>,
{
    #[command(flatten)]
    #[cfg(feature = "verbosity")]
    verbosity: Verbosity,

    #[command(flatten)]
    #[cfg(feature = "color")]
    color: Color,

    #[command(subcommand)]
    command: S,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ CliWithSubCommands
// ------------------------------------------------------------------------------------------------

impl<S> CommandOnce for CliWithSubCommands<S>
where
    S: Debug + Subcommand + CommandOnce<Output = ExitCode, Error = CliError>,
{
    type Output = ExitCode;
    type Error = CliError;

    fn execute_once(self) -> Result<Self::Output, Self::Error> {
        trace!(
            "CliWithSubCommands::execute(verbosity: {:?}, color: {:?})",
            self.verbosity, self.color
        );
        self.verbosity.execute_once()?;
        self.color.execute_once();
        self.command.execute_once()
    }
}

impl<S> CliWithSubCommands<S>
where
    S: Debug + Subcommand + CommandOnce<Output = ExitCode, Error = CliError>,
{
    ///
    /// Return the `Verbosity` structure set by the command-line.
    ///
    #[inline(always)]
    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }

    ///
    /// Return the number of times the verbose flag was set.
    ///
    #[inline(always)]
    pub fn verbose_count(&self) -> u8 {
        self.verbosity.verbose_count()
    }

    ///
    /// Return the number of times the quiet flag was set.
    ///
    #[inline(always)]
    pub fn quiet_count(&self) -> u8 {
        self.verbosity.quiet_count()
    }

    ///
    /// Return the tracing *maximum* filter value based on the verbosity and quiet flags.
    ///
    #[inline(always)]
    pub fn to_tracing_filter(&self) -> LevelFilter {
        self.verbosity.to_tracing_filter()
    }

    ///
    /// Return the `Color` structure set by the command-line.
    ///
    #[inline(always)]
    pub fn color(&self) -> &Color {
        &self.color
    }

    #[inline(always)]
    pub fn command(&self) -> &S {
        &self.command
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod color;

#[cfg(feature = "completion")]
pub mod completion;

#[cfg(feature = "external")]
pub mod external;

pub mod verbosity;
