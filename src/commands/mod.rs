//!
//! This module, and sub-modules provide actual command implementations for CLI construction.
//!
//! - [`CliWithSubCommands`]: A generic top-level command structure that supports sub-commands,
//!   verbosity, and color options.
//!   - [Verbosity]: A structure for handling verbose and quiet flags.
//! - [`ExternalSubCommand`](crate::commands::external::ExternalSubCommand): A sub-command that can
//!   be executed as a child process by the main CLI.
//! - [`Completion`](crate::commands::completion::Completion): A sub-command that handles shell
//!   completion for the CLI.
//!
//!

use crate::Command;
use crate::{
    error::CliError, get_quiet_count, get_verbose_count, set_quiet_count, set_verbose_count,
};
use ::tracing::trace;
use clap::{ArgAction, Args, Parser, Subcommand};
use colorchoice_clap::Color;
use std::{fmt::Debug, process::ExitCode};
use tracing_subscriber::filter::LevelFilter;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct CliWithSubCommands<S>
where
    S: Debug + Subcommand + Command<Output = ExitCode, Error = CliError> + Debug,
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

#[derive(Debug, Args)]
pub struct Verbosity {
    #[doc = "Increase logging verbosity by one level per occurance."]
    #[arg(
        long,
        short = 'v',
        action = ArgAction::Count,
        global = true,
    )]
    #[cfg(feature = "verbosity")]
    verbose: u8,

    #[doc = "Decrease logging verbosity by one level per occurance."]
    #[arg(
        long,
        short = 'q',
        action = ArgAction::Count,
        global = true,
        conflicts_with = "verbose",
    )]
    #[cfg(feature = "verbosity")]
    quiet: u8,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ CliWithSubCommands
// ------------------------------------------------------------------------------------------------

impl<S> Command for CliWithSubCommands<S>
where
    S: Debug + Subcommand + Command<Output = ExitCode, Error = CliError>,
{
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        trace!(
            "CliWithSubCommands::execute(verbosity: {:?}, color: {:?})",
            self.verbosity, self.color
        );
        self.verbosity.to_tracing_filter().execute()?;
        self.color.execute();
        self.command.execute()
    }
}

impl<S> CliWithSubCommands<S>
where
    S: Debug + Subcommand + Command<Output = ExitCode, Error = CliError> + Debug,
{
    ///
    /// Return the `Verbosity` structure set by the command-line.
    ///
    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }

    ///
    /// Return the number of times the verbose flag was set.
    ///
    pub fn verbose_count(&self) -> u8 {
        self.verbosity.verbose
    }

    ///
    /// Return the number of times the quiet flag was set.
    ///
    pub fn quiet_count(&self) -> u8 {
        self.verbosity.quiet
    }

    ///
    /// Return the tracing *maximum* filter value based on the verbosity and quiet flags.
    ///
    pub fn to_tracing_filter(&self) -> LevelFilter {
        self.verbosity.to_tracing_filter()
    }

    ///
    /// Return the `Color` structure set by the command-line.
    ///
    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn command(&self) -> &S {
        &self.command
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Verbosity
// ------------------------------------------------------------------------------------------------

impl Command for Verbosity {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        if !set_verbose_count(self.verbose) {
            Err(CliError::InitGlobalFailed("verbose-count"))
        } else if !set_quiet_count(self.quiet) {
            Err(CliError::InitGlobalFailed("quiet-count"))
        } else {
            Ok(ExitCode::SUCCESS)
        }
    }
}

impl Verbosity {
    ///
    /// Return the tracing *maximum* filter value based on the verbosity and quiet flags.
    ///
    /// This function provides a bride between the command-line options and the tracing system
    /// allowing as `LevelFilter` also implements `Command`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn main() {
    ///     let command_line = Cli::parse();
    ///
    ///     execute! (
    ///         command_line.verbosity.to_tracing_filter(),
    ///         command_line
    ///     );
    /// }
    /// ```
    pub fn to_tracing_filter(&self) -> LevelFilter {
        const DEFAULT_LEVEL_ERROR: u8 = 1;
        let offset = get_verbose_count() as i16 - get_quiet_count() as i16;
        match i16::from(DEFAULT_LEVEL_ERROR).saturating_add(offset) {
            i16::MIN..=0 => LevelFilter::OFF,
            1 => LevelFilter::ERROR,
            2 => LevelFilter::WARN,
            3 => LevelFilter::INFO,
            4 => LevelFilter::DEBUG,
            5..=i16::MAX => LevelFilter::TRACE,
        }
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

pub mod tracing;
