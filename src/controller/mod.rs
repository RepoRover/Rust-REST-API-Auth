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

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidEnvFile(String),
    MissingEnvVars(String),
    ExtraEnvVars(String),
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

impl PartialEq for AppError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && match (&self.original_err, &other.original_err) {
                (Some(a), Some(b)) => a.to_string() == b.to_string(),
                (None, None) => true,
                _ => false,
            }
    }
}

impl Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fmt;

//     #[test]
//     fn test_app_error_new() {
//         // Test instantiation with each variant of ErrorKind
//         let invalid_env_file = AppError::new(
//             ErrorKind::InvalidEnvFile("Invalid .env format".to_string()),
//             None,
//         );
//         assert!(matches!(
//             invalid_env_file.kind,
//             ErrorKind::InvalidEnvFile(_)
//         ));

//         let missing_env_vars =
//             AppError::new(ErrorKind::MissingEnvVars("API_KEY".to_string()), None);
//         assert!(matches!(
//             missing_env_vars.kind,
//             ErrorKind::MissingEnvVars(_)
//         ));

//         let extra_env_vars =
//             AppError::new(ErrorKind::ExtraEnvVars("UNNEEDED_VAR".to_string()), None);
//         assert!(matches!(extra_env_vars.kind, ErrorKind::ExtraEnvVars(_)));

//         let http_server_fail = AppError::new(ErrorKind::HttpServerFail, None);
//         assert!(matches!(http_server_fail.kind, ErrorKind::HttpServerFail));

//         let address_binding_fail = AppError::new(
//             ErrorKind::AddressBindingFail("127.0.0.1:8000".to_string()),
//             None,
//         );
//         assert!(matches!(
//             address_binding_fail.kind,
//             ErrorKind::AddressBindingFail(_)
//         ));
//     }

//     #[test]
//     fn test_app_error_display_without_original_error() {
//         let error = AppError::new(ErrorKind::HttpServerFail, None);
//         let formatted_error = format!("{}", error);
//         assert_eq!(formatted_error, "HttpServerFail");
//     }

//     #[test]
//     fn test_app_error_display_with_original_error() {
//         let original_error = std::io::Error::new(std::io::ErrorKind::Other, "IO error");
//         let error = AppError::new(
//             ErrorKind::InvalidEnvFile("Bad format".to_string()),
//             Some(Box::new(original_error)),
//         );
//         let formatted_error = format!("{}", error);
//         assert!(formatted_error.contains("InvalidEnvFile"));
//         assert!(formatted_error.contains("IO error"));
//     }

//     #[test]
//     fn test_app_error_as_error_trait() {
//         // Check that `AppError` conforms to `std::error::Error`
//         let result: Result<()> = Err(AppError::new(ErrorKind::HttpServerFail, None));
//         match result {
//             Err(e) => {
//                 assert!(e.source().is_none()); // No source error if original_err is None
//             }
//             _ => panic!("Expected an error"),
//         }

//         let original_error = std::io::Error::new(std::io::ErrorKind::Other, "Wrapped IO error");
//         let result_with_source: Result<()> = Err(AppError::new(
//             ErrorKind::MissingEnvVars("API_KEY".to_string()),
//             Some(Box::new(original_error)),
//         ));
//         match result_with_source {
//             Err(e) => {
//                 assert!(e.source().is_some()); // source should be present if original_err exists
//             }
//             _ => panic!("Expected an error"),
//         }
//     }
// }
