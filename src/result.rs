use super::*;

/// Extension trait for `Result` to convert errors into user-friendly or
/// system-friendly `Error` types.
/// 
/// # Examples
/// ```
/// use human_errors::{ResultExt, user, system};
/// 
/// fn might_fail(i: i32) -> Result<i32, std::io::Error> {
///     if i % 2 == 0 {
///         Ok(i)
///     } else {
///         Err(std::io::Error::new(std::io::ErrorKind::Other, "odd number error"))
///     }
/// }
/// 
/// fn process_number(i: i32) -> Result<i32, human_errors::Error> {
///     might_fail(i).into_user_error(
///         "Failed to process the number.",
///         "Ensure the number is even."
///     )
/// }
/// 
/// fn main() {
///     match process_number(3) {
///         Ok(n) => println!("Processed number: {}", n),
///         Err(e) => println!("{}", e.message()),
///     }
/// }
/// ```
pub trait ResultExt<T> {
    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a user-facing error with the provided description and advice.
    fn into_user_error(
        self,
        description: &str,
        advice: &str,
    ) -> Result<T, Error>;

    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a system-facing error with the provided description and advice.
    fn into_system_error(
        self,
        description: &str,
        advice: &str,
    ) -> Result<T, Error>;
}

impl <T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn into_user_error(
        self,
        description: &str,
        advice: &str,
    ) -> Result<T, Error> {
        self.map_err(|e| user_with_internal(description, advice, e))
    }

    fn into_system_error(
        self,
        description: &str,
        advice: &str,
    ) -> Result<T, Error> {
        self.map_err(|e| system_with_internal(description, advice, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_user_error() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "underlying error",
        ));

        let user_error = result
            .into_user_error(
                "Failed to perform operation.",
                "Please check your input and try again.",
            )
            .err()
            .unwrap();

        assert!(user_error.is_user());
    }

    #[test]
    fn test_into_system_error() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "underlying error",
        ));

        let system_error = result
            .into_system_error(
                "Failed to perform operation.",
                "Please check your input and try again.",
            )
            .err()
            .unwrap();

        assert!(system_error.is_system());
    }
}