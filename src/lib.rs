use std::fmt::Display;

pub mod util;

#[derive(Debug, Clone)]
pub enum AsciifyError {
    EmptyGrayScale,
}

impl std::error::Error for AsciifyError {}

impl Display for AsciifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyGrayScale => write!(f, "gray scale is empty"),
        }
    }
}
