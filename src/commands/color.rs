//!
//! Initialize color settings for the application, including global color overrides.
//!
//! The [`Color`] struct provides a command-line flag for controlling color output, giving the
//! user the ability to specify when color should be used (`auto`, `always`, or `never`). This crate
//! also re-exports the [`ColorChoice`] type from `clap` for convenience if clients need to work
//! with the user choice directly.
//!

use crate::{
    CommandOnce,
    clap::{Args, ColorChoice},
};
use colorchoice::ColorChoice as GlobalColorChoice;
use std::{convert::Infallible, process::ExitCode};
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Command-Line mixin that provides the `--color` flag.
///
#[derive(Debug, Default, Args)]
#[command(about = None, long_about = None)]
pub struct Color {
    /// Controls when to use color.
    #[arg(long, default_value_t = ColorChoice::Auto, value_name = "WHEN", value_enum, global = true)]
    color: ColorChoice,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl CommandOnce for Color {
    type Output = ExitCode;
    type Error = Infallible;

    fn execute_once(self) -> Result<Self::Output, Self::Error> {
        trace!("Setting color globals to `{:?}`", self.color);
        let global_choice = match self.color {
            ColorChoice::Auto => GlobalColorChoice::Auto,
            ColorChoice::Always => GlobalColorChoice::Always,
            ColorChoice::Never => GlobalColorChoice::Never,
        };
        global_choice.write_global();
        match self.color {
            ColorChoice::Always => colored::control::set_override(true),
            ColorChoice::Never => colored::control::set_override(false),
            ColorChoice::Auto => {}
        }
        Ok(ExitCode::SUCCESS)
    }
}

impl Color {
    /// Returns the color choice set by this command.
    pub fn to_color_choice(&self) -> ColorChoice {
        self.color
    }
}
