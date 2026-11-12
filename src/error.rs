//!
//! Provides the error types for this crate.
//!

use std::{path::PathBuf, process::ExitStatus};
use thiserror::Error;
use tracing_subscriber::filter::FromEnvError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum CliError {
    #[error(
        "Could not retrieve value from environment variable for command-line argument; error: {0}"
    )]
    EnvError(#[from] FromEnvError),

    #[error("An error occured initializing the global state; value name: {0:?}")]
    InitGlobalFailed(&'static str),

    #[error("Sub-process failed with exit status {0:?}")]
    SubProcessFailure(ExitStatus),

    #[error("Sub-process executable `{0:?}` is missing")]
    SubProcessExecutableMissing(PathBuf),
}
