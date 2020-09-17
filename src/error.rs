use std::convert::From;
use std::error;
use std::fmt;
use std::io::Error;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum ProgramError {
    ParseError(ParseError),
    InputOutput(Error),
    PlotError(Box<dyn std::error::Error>),
    UnknowStatus(usize),
}

#[derive(Debug)]
pub enum ParseError {
    Int(ParseIntError),
    Float(ParseFloatError),
    MissingToken(usize),
}

#[derive(Debug)]
pub enum FormatError {
    MissingFormat(String),
    UnknownExtension(String),
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        Self::InputOutput(e)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        Self::Int(e)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(e: ParseFloatError) -> Self {
        Self::Float(e)
    }
}

impl From<ParseError> for ProgramError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl std::convert::From<Box<dyn std::error::Error>> for ProgramError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Self::PlotError(e)
    }
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOutput(io_err) => write!(f, "IO Error: {}", io_err),
            Self::ParseError(parse_err) => write!(f, "Parse Error: {}", parse_err),
            Self::UnknowStatus(s) => write!(
                f,
                "Read Unknown Status Value: {}\nExpect: ``, `0`, `1`, `2`",
                s
            ),
            Self::PlotError(plot_err) => write!(f, "Plot Error: {}", plot_err),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(f_err) => write!(f, "{}", f_err),
            Self::Int(i_err) => write!(f, "{}", i_err),
            Self::MissingToken(i) => write!(f, "Missing Value on column {}", i),
        }
    }
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFormat(path) => write!(
                f,
                "Given file:`{}`\ndoes not have an extension\ncannot understand output format",
                path
            ),
            Self::UnknownExtension(ext) => write!(
                f,
                "Given extension: `{}` is unknow\ncannot understand output format",
                ext
            ),
        }
    }
}

impl error::Error for FormatError {}
