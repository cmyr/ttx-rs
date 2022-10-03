//! Error handling.

use std::path::PathBuf;

/// An error that occurs in the ttx process.
pub enum Error {
    /// An input file could not be read
    Load(std::io::Error),
    /// An error was reported from the read-fonts crate
    Runtime(read_fonts::ReadError),
    /// A provided path was not well formed
    InvalidPath(PathBuf),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Load(inner) => write!(f, "Error loading file: '{inner}'"),
            Error::Runtime(inner) => inner.fmt(f),
            Error::InvalidPath(inner) => write!(f, "Path '{}' is invalid", inner.display()),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(src: std::io::Error) -> Error {
        Error::Load(src)
    }
}

impl From<read_fonts::ReadError> for Error {
    fn from(src: read_fonts::ReadError) -> Error {
        Error::Runtime(src)
    }
}
