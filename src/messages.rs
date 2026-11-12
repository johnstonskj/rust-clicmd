//!
//! Provides structured message handling for the application, including severity levels, context,
//! and notes.
//!
//! Primarily used for creating and displaying structured messages with varying severity levels,
//! context, and additional notes for unhandled errors.
//!

use colored::Colorize;
use core::{error::Error, fmt::Display};
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};
use tracing::{error, info, warn};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    severity: Severity,
    message: String,
    context: Option<String>,
    notes: Vec<Note>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    content: String,
    kind: NoteKind,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, EnumIter)]
pub enum Severity {
    Information,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, EnumIter)]
pub enum NoteKind {
    Help,
    Note,
    #[strum(serialize = "See Also")]
    SeeAlso,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (icon, severity, message) = match self.severity {
            Severity::Information => ("💡", self.severity.to_string(), self.message.to_string()),
            Severity::Warning => (
                "⚠️",
                self.severity.to_string().bold().yellow().to_string(),
                self.message.to_string(),
            ),
            Severity::Error => (
                "🛑",
                self.severity.to_string().bold().red().to_string(),
                self.message.to_string().bold().to_string(),
            ),
        };
        writeln!(f, "{} {}: {}", icon, severity, message)?;
        if let Some(context) = &self.context {
            writeln!(
                f,
                "{} {}",
                if self.notes.is_empty() {
                    "   └── 🔎"
                } else {
                    "   ├── 🔎"
                },
                context.blue()
            )?;
        }
        for (note, is_last) in self
            .notes
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i == self.notes.len() - 1))
        {
            writeln!(
                f,
                "{} {}",
                if is_last {
                    "   └──"
                } else {
                    "   ├──"
                },
                format!(
                    "{}  {}: {}",
                    match note.kind {
                        NoteKind::Help => "ℹ️",
                        NoteKind::Note => "🗒️",
                        NoteKind::SeeAlso => "🔗",
                    },
                    note.kind,
                    note.content
                )
                .dimmed()
            )?;
        }
        Ok(())
    }
}

const RUST_VERSION: &str = env!("CARGO_PKG_RUST_VERSION");

impl Message {
    pub fn from_error<E: Error>(
        error: E,
        package_name: &str,
        package_version: &str,
        _repository: Option<&str>,
    ) -> Self {
        Self::error("an unhandled error has been detected, sorry about that")
            .with_context(format!("Error {error}"))
            .with_notes([
                Note::note("if this re-occurs, please file a bug"),
                Note::note(format!(
                    "Package: {package_name}@{package_version}, rust@{RUST_VERSION}"
                )),
                //Note::see_also("https://github.com/johnstonskj/rust-rfham/issues"),
            ])
    }

    pub fn new<S: Into<String>>(severity: Severity, message: S) -> Self {
        Self {
            severity,
            message: message.into(),
            context: Default::default(),
            notes: Default::default(),
        }
    }

    pub fn error<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Error, message)
    }

    #[allow(dead_code)]
    pub fn warning<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Warning, message)
    }

    #[allow(dead_code)]
    pub fn infomation<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Information, message)
    }

    pub fn with_context<S: Into<String>>(mut self, context: S) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn with_notes<I: IntoIterator<Item = Note>>(mut self, notes: I) -> Self {
        self.notes = notes.into_iter().collect();
        self
    }

    pub fn with_note(mut self, note: Note) -> Self {
        self.notes = vec![note];
        self
    }

    pub fn print(&self) {
        match self.severity {
            Severity::Information => {
                info!("{self}");
                println!("{self}")
            }
            Severity::Warning => {
                warn!("{self}");
                println!("{self}")
            }
            Severity::Error => {
                error!("{self}");
                eprint!("{self}")
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Note {
    pub fn new<S: Into<String>>(kind: NoteKind, content: S) -> Self {
        Self {
            kind,
            content: content.into(),
        }
    }

    pub fn help<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::Help, content)
    }

    #[allow(clippy::self_named_constructors)]
    pub fn note<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::Note, content)
    }

    pub fn see_also<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::SeeAlso, content)
    }
}
