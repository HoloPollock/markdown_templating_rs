use std::error;
use std::fmt;
use std::io;

#[derive(Debug, Clone)]
pub struct RegexError;

impl fmt::Display for RegexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to find item in regex")
    }
}

impl error::Error for RegexError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub enum SnippitError {
    IoError(io::Error),
    Regex(RegexError),
}

impl From<io::Error> for SnippitError {
    fn from(error: io::Error) -> Self {
        SnippitError::IoError(error)
    }
}

impl fmt::Display for SnippitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SnippitError::IoError(ref e) => e.fmt(f),
            SnippitError::Regex(ref e) => e.fmt(f),
        }
    }
}

impl From<RegexError> for SnippitError {
    fn from(error: RegexError) -> Self {
        SnippitError::Regex(error)
    }
}

pub enum ConvertError {
    IoError(io::Error),
    FromUtfError(std::string::FromUtf8Error),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConvertError::IoError(ref e) => e.fmt(f),
            ConvertError::FromUtfError(ref e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for ConvertError {
    fn from(error: io::Error) -> Self {
        ConvertError::IoError(error)
    }
}

impl From<std::string::FromUtf8Error> for ConvertError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ConvertError::FromUtfError(error)
    }
}

pub enum ToHtmlError {
    Snippet(SnippitError),
    IoError(io::Error),
    Convert(ConvertError),
}

impl From<io::Error> for ToHtmlError {
    fn from(error: io::Error) -> Self {
        ToHtmlError::IoError(error)
    }
}

impl fmt::Display for ToHtmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ToHtmlError::IoError(ref e) => e.fmt(f),
            ToHtmlError::Snippet(ref e) => e.fmt(f),
            ToHtmlError::Convert(ref e) => e.fmt(f),
        }
    }
}
