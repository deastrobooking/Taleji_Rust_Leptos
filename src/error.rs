use thiserror::Error;
use leptos::ServerFnError;

/// Application-specific error types for better error handling
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl From<AppError> for ServerFnError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::NotFound(msg) => ServerFnError::new(format!("Not found: {}", msg)),
            AppError::Unauthorized => ServerFnError::new("Unauthorized access".to_string()),
            AppError::Forbidden(msg) => ServerFnError::new(format!("Forbidden: {}", msg)),
            AppError::Validation(msg) => ServerFnError::new(format!("Validation error: {}", msg)),
            AppError::RateLimitExceeded => ServerFnError::new("Rate limit exceeded".to_string()),
            _ => ServerFnError::new("Internal server error".to_string()),
        }
    }
}

/// Result type alias for convenience
pub type AppResult<T> = Result<T, AppError>;

/// Helper function to log errors with context
pub fn log_error(error: &AppError, context: &str) {
    tracing::error!(
        error = %error,
        context = context,
        "Application error occurred"
    );
}

/// Validation helpers
pub mod validation {
    use super::AppError;
    use validator::{Validate, ValidationErrors};

    pub fn validate_input<T: Validate>(input: &T) -> Result<(), AppError> {
        input.validate().map_err(|e| {
            let errors = format_validation_errors(e);
            AppError::Validation(errors)
        })
    }

    fn format_validation_errors(errors: ValidationErrors) -> String {
        errors
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                let field_errors: Vec<String> = errs
                    .iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                    .collect();
                format!("{}: {}", field, field_errors.join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ")
    }
}