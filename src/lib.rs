use std::fmt::Display;

pub mod util;

#[derive(Debug, Clone)]
pub enum AsciificateError {
    EmptyGrayScale,
}

impl std::error::Error for AsciificateError {}

impl Display for AsciificateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyGrayScale => write!(f, "gray scale is empty"),
        }
    }
}
