//! Example demonstrating the error handling system in ani-rs.
//!
//! This example shows:
//! 1. Using built-in `Status` enum for standard errors
//! 2. Defining custom error types for business logic
//! 3. Throwing errors that will be caught as JavaScript exceptions

use ani::prelude::*;
use ani_derive::ani;

// ===========================================================================
// Example 1: Using built-in Status for standard errors
// ===========================================================================

/// A function that returns a standard error using the built-in Status enum.
/// This is the simplest way to return errors.
#[ani]
fn divide(a: f64, b: f64) -> Result<f64> {
    if b == 0.0 {
        // Use Error::new with Status for standard errors
        return Err(Error::new(Status::InvalidArgs, "Cannot divide by zero"));
    }
    Ok(a / b)
}

/// A function that validates input and returns an error if invalid.
#[ani]
fn validate_age(age: i32) -> Result<String> {
    if age < 0 {
        return Err(Error::new(Status::InvalidArgs, "Age cannot be negative"));
    }
    if age > 150 {
        return Err(Error::new(
            Status::InvalidArgs,
            "Age seems unrealistic (> 150)",
        ));
    }
    Ok(format!("Age {} is valid", age))
}

// ===========================================================================
// Example 2: Custom error types for business logic
// ===========================================================================

/// A custom error enum for authentication-related operations.
/// By implementing `AsRef<str>`, it can be used with `Error<AuthError>`.
#[derive(Debug, Clone, Copy)]
pub enum AuthError {
    InvalidCredentials,
    TokenExpired,
    InsufficientPermissions,
    AccountLocked,
}

impl AsRef<str> for AuthError {
    fn as_ref(&self) -> &str {
        match self {
            AuthError::InvalidCredentials => "InvalidCredentials",
            AuthError::TokenExpired => "TokenExpired",
            AuthError::InsufficientPermissions => "InsufficientPermissions",
            AuthError::AccountLocked => "AccountLocked",
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AuthError::InvalidCredentials => "Invalid username or password",
            AuthError::TokenExpired => "Authentication token has expired",
            AuthError::InsufficientPermissions => {
                "You don't have permission to perform this action"
            }
            AuthError::AccountLocked => "Account has been locked due to too many failed attempts",
        };
        write!(f, "{}", msg)
    }
}

/// Simulates a login function that uses custom AuthError.
/// The error will be converted to a JavaScript exception when thrown.
fn authenticate(username: &str, password: &str) -> std::result::Result<String, Error<AuthError>> {
    if username.is_empty() || password.is_empty() {
        return Err(Error::new(
            AuthError::InvalidCredentials,
            "Username and password are required",
        ));
    }

    // Simulate authentication logic
    if username == "admin" && password == "secret" {
        Ok("auth_token_12345".to_string())
    } else if username == "locked_user" {
        Err(Error::new(
            AuthError::AccountLocked,
            "This account has been locked. Please contact support.",
        ))
    } else {
        Err(Error::new(
            AuthError::InvalidCredentials,
            "The username or password you entered is incorrect",
        ))
    }
}

/// A wrapper function that can be exported to ArkTS.
/// It converts the custom error to a standard Result<T>.
#[ani]
fn login(username: String, password: String) -> Result<String> {
    authenticate(&username, &password).map_err(|e| {
        // Convert custom error to standard Error<Status>
        // Access status and reason as public fields
        Error::new(
            Status::Error,
            format!("{}: {}", e.status.as_ref(), e.reason),
        )
    })
}

// ===========================================================================
// Example 3: Error with cause chain
// ===========================================================================

/// Demonstrates creating errors with a cause chain.
#[ani]
fn read_config(path: String) -> Result<String> {
    // Simulate a file read error with a cause chain
    if path.is_empty() {
        return Err(Error::new(Status::InvalidArgs, "Path cannot be empty"));
    }

    if !path.ends_with(".json") {
        // Create an error with a cause using the with_cause constructor
        let io_error =
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file extension");
        let cause = Error::new(Status::Error, io_error.to_string());

        return Err(Error::with_cause(
            Status::Error,
            "Failed to read config file",
            cause,
        ));
    }

    // Simulate successful read
    Ok(format!("{{\"path\": \"{}\"}}", path))
}

// ===========================================================================
// Example 4: Using BusinessError for direct exception throwing
// ===========================================================================

/// Demonstrates using BusinessError for throwing exceptions.
/// All ANI errors inherit from escompat.BusinessError.
/// Note: The error will be converted to BusinessError when the function returns Err.
#[ani]
fn check_array_bounds(index: i32, length: i32) -> Result<()> {
    if index < 0 {
        // Return an error for negative index
        return Err(Error::new(Status::OutOfRange, "Index cannot be negative"));
    }

    if index >= length {
        return Err(Error::new(
            Status::OutOfRange,
            format!(
                "Index {} is out of bounds for array of length {}",
                index, length
            ),
        ));
    }

    Ok(())
}

/// Demonstrates returning a type error.
#[ani]
fn expect_string_type(value: String) -> Result<String> {
    if value.starts_with("__invalid__") {
        return Err(Error::new(
            Status::InvalidType,
            "Expected a valid string, got invalid marker",
        ));
    }
    Ok(value.to_uppercase())
}

#[ani]
fn throw_existing_error(env: &Env<'_>, error: AniError<'_>) -> Result<()> {
    env.throw_error(&error)
}

#[ani]
fn reject_with_error_handle(
    env: &Env<'_>,
    resolver: AniResolver,
    error: AniError<'_>,
) -> Result<()> {
    env.promise_reject(&resolver, &error)
}

// ===========================================================================
// Example 5: Result type aliases for cleaner code
// ===========================================================================

/// Type alias for operations that can fail with AuthError
pub type AuthResult<T> = std::result::Result<T, Error<AuthError>>;

/// Using the type alias for cleaner function signatures
fn verify_token(token: &str) -> AuthResult<bool> {
    if token.is_empty() {
        return Err(Error::new(AuthError::InvalidCredentials, "Token is empty"));
    }

    if token.starts_with("expired_") {
        return Err(Error::new(AuthError::TokenExpired, "Please login again"));
    }

    Ok(true)
}

// ===========================================================================
// Example 6: Using anyhow (with error_anyhow feature)
// ===========================================================================

/// Example using anyhow for flexible error handling.
/// Enable with: `ani = { features = ["error_anyhow"] }`
#[cfg(feature = "anyhow_example")]
mod anyhow_example {
    use ani::prelude::*;
    use ani_derive::ani;
    use anyhow::{anyhow, Context};

    /// Internal function that returns anyhow::Result.
    /// Useful for operations that may fail in many ways.
    fn load_config_internal(path: &str) -> anyhow::Result<String> {
        if path.is_empty() {
            return Err(anyhow!("Config path cannot be empty"));
        }

        // Simulate file operations with context
        if !path.ends_with(".json") {
            return Err(anyhow!("Invalid extension")).context("Config file must be a JSON file");
        }

        // Simulate successful read
        Ok(format!("{{\"loaded_from\": \"{}\"}}", path))
    }

    /// Exported function using anyhow for error handling.
    /// anyhow::Error automatically converts to ani::Error.
    #[ani]
    fn load_config(path: String) -> Result<String> {
        let config = load_config_internal(&path)
            .with_context(|| format!("Failed to load config from '{}'", path))?;
        Ok(config)
    }

    /// Example showing how to combine multiple fallible operations.
    #[ani]
    fn process_data(input: String) -> Result<String> {
        // Validate input is not empty
        if input.is_empty() {
            return Err(anyhow!("Input cannot be empty").into());
        }

        // Check for required format
        if !input.contains(':') {
            return Err(anyhow!("Input must contain key:value format")
                .context("Invalid input format")
                .into());
        }

        let parts: Vec<&str> = input.splitn(2, ':').collect();
        let key = parts[0].trim();
        let value = parts[1].trim();

        if key.is_empty() {
            return Err(anyhow!("Key cannot be empty").into());
        }

        Ok(format!("Processed {} = {}", key, value))
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_error() {
        let result = authenticate("wrong", "wrong");
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.status.as_ref(), "InvalidCredentials");
    }

    #[test]
    fn test_successful_auth() {
        let result = authenticate("admin", "secret");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "auth_token_12345");
    }

    #[test]
    fn test_error_with_cause() {
        // Create an error with a cause using with_cause constructor
        let cause = Error::new(Status::Error, "Underlying error");
        let error = Error::with_cause(Status::Error, "High level error", cause);

        assert!(error.cause.is_some());
        assert_eq!(error.reason, "High level error");
        assert_eq!(error.cause.as_ref().unwrap().reason, "Underlying error");
    }

    #[test]
    fn test_verify_token() {
        assert!(verify_token("valid_token").is_ok());
        assert!(verify_token("").is_err());
        assert!(verify_token("expired_token").is_err());

        let err = verify_token("expired_token").unwrap_err();
        assert_eq!(err.status.as_ref(), "TokenExpired");
    }

    #[test]
    fn test_handle_surface_exports_compile() {
        let _ = throw_existing_error;
        let _ = reject_with_error_handle;
    }
}
