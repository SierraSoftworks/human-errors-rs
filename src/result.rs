use std::borrow::Cow;

use super::*;

/// Extension trait for `Result` to convert errors into user-friendly or
/// system-friendly `Error` types.
///
/// # Examples
/// ```
/// use human_errors::ResultExt;
///
/// // Converts any error into a user-caused error with the provided advice.
/// "0.not a number".parse::<i32>()
///     .map_err_as_user(&["Please provide a valid integer input."]);
///
/// // Converts any error into a system-caused error with the provided advice.
/// "0.not a number".parse::<i32>()
///     .map_err_as_system(&["Please check your system configuration."]);
///
/// // Wraps any error into a user-caused error with a custom message and advice.
/// "0.not a number".parse::<i32>()
///     .wrap_err_as_user(
///         "Failed to parse the provided input as an integer.",
///         &["Please provide a valid integer input."],
///     );
///
/// // Wraps any error into a system-caused error with a custom message and advice.
/// "0.not a number".parse::<i32>()
///     .wrap_err_as_system(
///         "Failed to parse the provided input as an integer.",
///         &["Please check your system configuration."],
///     );
/// ```
pub trait ResultExt<T> {
    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a user-facing error with the provided advice.
    ///
    /// # Examples
    /// ```
    /// use human_errors::ResultExt;
    ///
    /// "0.not a number".parse::<i32>()
    ///     .or_user_err(&["Please provide a valid integer input."]);
    /// ```
    fn or_user_err(self, advice: &'static [&'static str]) -> Result<T, Error>;

    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a user-facing error with the provided description and advice.
    ///
    /// # Examples
    /// ```
    /// use human_errors::ResultExt;
    ///
    /// "0.not a number".parse::<i32>()
    ///     .wrap_user_err(
    ///         "Failed to parse the provided input as an integer.",
    ///         &["Please provide a valid integer input."],
    ///     );
    /// ```
    fn wrap_user_err<S: Into<Cow<'static, str>> + 'static>(
        self,
        message: S,
        advice: &'static [&'static str],
    ) -> Result<T, Error>;

    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a system-facing error with the provided advice.
    /// 
    /// # Examples
    /// ```
    /// use human_errors::ResultExt;
    /// 
    /// "0.not a number".parse::<i32>()
    ///    .or_system_err(&["Please report this issue to the dev team."]);
    /// ```
    fn or_system_err(self, advice: &'static [&'static str]) -> Result<T, Error>;

    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a system-facing error with the provided description and advice.
    /// 
    /// # Examples
    /// ```
    /// use human_errors::ResultExt;
    ///
    /// "0.not a number".parse::<i32>()
    ///     .wrap_system_err(
    ///         "Failed to parse the provided input as an integer.",
    ///         &["Please report this issue to the dev team."],
    ///     );
    /// ```
    fn wrap_system_err<S: Into<Cow<'static, str>> + 'static>(
        self,
        message: S,
        advice: &'static [&'static str],
    ) -> Result<T, Error>;

    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a user-facing error with the provided advice.
    ///
    /// # Examples
    /// ```
    /// use human_errors::ResultExt;
    ///
    /// "0.not a number".parse::<i32>()
    ///     .map_err_as_user(&["Please provide a valid integer input."]);
    /// ```
    #[deprecated(
        since = "0.2.3",
        note = "We are updating the interface to match the Rust stdlib style. Please use `or_user_err` method instead."
    )]
    fn map_err_as_user(self, advice: &'static [&'static str]) -> Result<T, Error>;

    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a user-facing error with the provided description and advice.
    ///
    /// # Examples
    /// ```
    /// use human_errors::ResultExt;
    ///
    /// "0.not a number".parse::<i32>()
    ///     .wrap_err_as_user(
    ///         "Failed to parse the provided input as an integer.",
    ///         &["Please provide a valid integer input."],
    ///     );
    /// ```
    #[deprecated(
        since = "0.2.3",
        note = "We are updating the interface to match the Rust stdlib style. Please use `wrap_user_err` method instead."
    )]
    fn wrap_err_as_user<S: Into<Cow<'static, str>> + 'static>(
        self,
        message: S,
        advice: &'static [&'static str],
    ) -> Result<T, Error>;

    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a system-facing error with the provided advice.
    ///
    /// # Examples
    /// ```
    /// use human_errors::ResultExt;
    ///
    /// "0.not a number".parse::<i32>()
    ///     .map_err_as_system(&["Please report this issue to the dev team."]);
    /// ```
    #[deprecated(
        since = "0.2.3",
        note = "We are updating the interface to match the Rust stdlib style. Please use `or_system_err` method instead."
    )]
    fn map_err_as_system(self, advice: &'static [&'static str]) -> Result<T, Error>;

    /// Converts a `Result<T, E>` into a `Result<T, Error>`, wrapping any
    /// error in a system-facing error with the provided description and advice.
    /// 
    /// # Examples
    /// ```
    /// use human_errors::ResultExt;
    ///
    /// "0.not a number".parse::<i32>()
    ///     .wrap_err_as_system(
    ///         "Failed to parse the provided input as an integer.",
    ///         &["Please report this issue to the dev team."],
    ///     );
    /// ```
    #[deprecated(
        since = "0.2.3",
        note = "We are updating the interface to match the Rust stdlib style. Please use `wrap_system_err` method instead."
    )]
    fn wrap_err_as_system<S: Into<Cow<'static, str>> + 'static>(
        self,
        message: S,
        advice: &'static [&'static str],
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
{
    fn or_user_err(self, advice: &'static [&'static str]) -> Result<T, Error> {
        self.map_err(|e| user(e, advice))
    }

    fn wrap_user_err<S: Into<Cow<'static, str>> + 'static>(
        self,
        message: S,
        advice: &'static [&'static str],
    ) -> Result<T, Error> {
        self.map_err(|e| wrap_user(e, message, advice))
    }

    fn or_system_err(self, advice: &'static [&'static str]) -> Result<T, Error> {
        self.map_err(|e| system(e, advice))
    }

    fn wrap_system_err<S: Into<Cow<'static, str>> + 'static>(
            self,
            message: S,
            advice: &'static [&'static str],
        ) -> Result<T, Error> {
        self.map_err(|e| wrap_system(e, message, advice))
    }

    fn map_err_as_user(self, advice: &'static [&'static str]) -> Result<T, Error> {
        self.map_err(|e| user(e, advice))
    }

    fn wrap_err_as_user<S: Into<Cow<'static, str>> + 'static>(
        self,
        message: S,
        advice: &'static [&'static str],
    ) -> Result<T, Error> {
        self.map_err(|e| user(wrap_user(e, message, advice), advice))
    }

    fn map_err_as_system(self, advice: &'static [&'static str]) -> Result<T, Error> {
        self.map_err(|e| system(e, advice))
    }

    fn wrap_err_as_system<S: Into<Cow<'static, str>> + 'static>(
        self,
        message: S,
        advice: &'static [&'static str],
    ) -> Result<T, Error> {
        self.map_err(|e| system(wrap_system(e, message, advice), advice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or_user_error() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::other("underlying error"));

        let user_error = result
            .or_user_err(&["Please check your input and try again."])
            .err()
            .unwrap();

        assert!(user_error.is(Kind::User));
    }

    #[test]
    fn test_wrap_user_error() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::other("underlying error"));

        let user_error = result
            .wrap_user_err(
                "Failed to process the input.",
                &["Please check your input and try again."],
            )
            .err()
            .unwrap();

        assert!(user_error.is(Kind::User));
        assert_eq!(user_error.message(), "Failed to process the input.");
    }

    #[test]
    fn test_or_system_error() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::other("underlying error"));

        let system_error = result
            .or_system_err(&["Please check your input and try again."])
            .err()
            .unwrap();

        assert!(system_error.is(Kind::System));
    }

    #[test]
    fn test_wrap_system_error() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::other("underlying error"));
        let system_error = result
            .wrap_system_err(
                "Failed to process the input.",
                &["Please check your input and try again."],
            )
            .err()
            .unwrap();
        assert!(system_error.is(Kind::System));
        assert_eq!(system_error.message(), "Failed to process the input.");
    }
}
