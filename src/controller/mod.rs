use std::{error::Error, fmt};

#[derive(Debug)]
pub struct AppError {
    kind: ErrorKind,
    original_err: Option<Box<dyn Error>>,
}

impl AppError {
    pub fn new(kind: ErrorKind, original_err: Option<Box<dyn Error>>) -> Self {
        Self { kind, original_err }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidEnvFile(String),
    MissingEnvVars(String),
    HttpServerFail,
    AddressBindingFail(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.original_err {
            Some(err) => write!(f, "{:?}: {}", self.kind, err),
            None => write!(f, "{:?}", self.kind),
        }
    }
}

impl Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
