use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error{
    FileReadingError
}

impl Display for Error{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileReadingError=> write!(f, "Error reading file"),
            _ => write!(f, "Generic Error")
        }
    }
}