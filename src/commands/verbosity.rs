//!
//! Initialize tracing settings for the application using the `tracing_subscriber` crate.
//!
//! The [`Verbosity`] struct provides command-line flags for controlling the verbosity of the
//! application. It also implements [`CommandOnce`] for initializing the tracing settings. This
//! module also re-exports the [`LevelFilter`] type from `tracing_subscriber` as this is calculated
//! based on the combination of verbosity and quiet flags.
//!

use crate::{
    CommandOnce,
    clap::{ArgAction, Args},
    error::CliError,
    get_quiet_count, get_verbose_count, set_quiet_count, set_verbose_count,
};
use std::process::ExitCode;
use tracing_subscriber::filter::EnvFilter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// Re-export this as the type derived from the command-line arguments.
pub use tracing_subscriber::filter::LevelFilter;

///
/// A Command-Line mixin that provides both the `--verbose` and `--quiet` flags.
///
#[derive(Debug, Default, Args)]
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
// Implementations ❯ Verbosity
// ------------------------------------------------------------------------------------------------

impl CommandOnce for Verbosity {
    type Output = ExitCode;
    type Error = CliError;

    fn execute_once(self) -> Result<Self::Output, Self::Error> {
        if !set_verbose_count(self.verbose) {
            Err(CliError::InitGlobalFailed("verbose-count"))
        } else if !set_quiet_count(self.quiet) {
            Err(CliError::InitGlobalFailed("quiet-count"))
        } else {
            let max_level = self.to_tracing_filter();
            tracing_subscriber::fmt()
                .pretty()
                .with_env_filter(
                    EnvFilter::builder()
                        .with_default_directive(max_level.into())
                        .from_env()?,
                )
                .init();

            tracing::info!("tracing initialized with level filter: {:?}", self);

            Ok(ExitCode::SUCCESS)
        }
    }
}

impl Verbosity {
    ///
    /// Return the number of times the verbose flag was set.
    ///
    #[inline(always)]
    pub fn verbose_count(&self) -> u8 {
        self.verbose
    }

    ///
    /// Return the number of times the quiet flag was set.
    ///
    #[inline(always)]
    pub fn quiet_count(&self) -> u8 {
        self.quiet
    }

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
    ///
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
