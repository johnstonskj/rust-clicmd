//!
//! Provides tracing initialization for the application using `tracing_subscriber`.
//!
//! Rather than creating a new command type, the `Command` trait is implemented directly for the
//! `LevelFilter` type.
//!

use crate::Command;
use std::process::ExitCode;
use tracing_subscriber::filter::{EnvFilter, FromEnvError, LevelFilter};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Return a default command instance.
///
#[inline(always)]
pub fn default() -> LevelFilter {
    LevelFilter::INFO
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Command for LevelFilter {
    type Output = ExitCode;
    type Error = FromEnvError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(self.into())
                    .from_env()?,
            )
            .init();

        tracing::info!("Telemetry initialized with level filter: {:?}", self);

        Ok(ExitCode::SUCCESS)
    }
}
