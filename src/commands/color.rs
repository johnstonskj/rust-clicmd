//!
//! Initialize color settings for the application, including global color overrides.
//!
//! This module wraps the `colorchoice_clap` crate to use the command style, and as part of the
//! command execution it sets the global color choice accordingly. Rather than create a new command
//! type [`Command`] is implemented directly for the `Color` type.
//!

use crate::Command;
use colorchoice::ColorChoice;
use std::{convert::Infallible, process::ExitCode};
use tracing::trace;

// re-export as this is our Command type.
pub use colorchoice_clap::Color;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Return a default command instance.
///
#[inline(always)]
pub fn default() -> Color {
    Color::default()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Command for Color {
    type Output = ExitCode;
    type Error = Infallible;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        trace!("Setting color globals to `{:?}`", self.as_choice());
        self.write_global();
        match self.as_choice() {
            ColorChoice::Always | ColorChoice::AlwaysAnsi => colored::control::set_override(true),
            ColorChoice::Never => colored::control::set_override(false),
            ColorChoice::Auto => {}
        }
        Ok(ExitCode::SUCCESS)
    }
}
