use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Errors {
    MessageError(String),
    ParseNumberError,
    OverflowError,
}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::MessageError(message) => write!(f, "Error: {message}"),
            Errors::ParseNumberError => write!(f, "Error parsing number"),
            Errors::OverflowError => write!(f, "Number overflow error"),
        }
    }
}

