pub use std::error;
pub use super::Error;

/// A basic error triggered by something the user has done.
///
/// Constructs a new [Error] describing a failure which was the result of an
/// action that the user has taken. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
///
/// # Examples
/// ```
/// use human_errors;
/// 
/// human_errors::user(
///   "We could not open the config file you provided.",
///   "Make sure that the file exists and is readable by the application.",
/// );
/// ```
pub fn user(description: &str, advice: &str) -> Error {
    Error::UserError(description.to_string(), advice.to_string(), None, None)
}

/// An error triggered by something the user has done, with a deeper cause.
///
/// Constructs a new [Error] describing a failure which was the result of an
/// action that the user has taken. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
/// It also includes the details of another error which resulted in this failure,
/// as well as any advice that error may provide.
///
/// # Examples
/// ```
/// use human_errors;
/// 
/// human_errors::user_with_cause(
///   "We could not open the config file you provided.",
///   "Make sure that you've specified a valid config file with the --config option.",
///   human_errors::user(
///     "We could not find a file at /home/user/.config/demo.yml",
///     "Make sure that the file exists and is readable by the application."
///   )
/// );
/// ```
pub fn user_with_cause(description: &str, advice: &str, cause: Error) -> Error {
    Error::UserError(
        description.to_string(),
        advice.to_string(),
        Some(Box::from(cause)),
        None,
    )
}

/// An error triggered by something the user has done, with a deeper cause.
///
/// Constructs a new [Error] describing a failure which was the result of an
/// action that the user has taken. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
/// It also includes the details of another error which resulted in this failure.
///
/// **NOTE**: The internal error may be any type which may be converted into a [Box<std::error::Error>].
///
/// # Examples
/// ```
/// use human_errors;
/// 
/// human_errors::user_with_internal(
///   "We could not open the config file you provided.",
///   "Make sure that the file exists and is readable by the application.",
///   human_errors::detailed_message("ENOENT 2: No such file or directory")
/// );
/// ```
pub fn user_with_internal<T>(description: &str, advice: &str, internal: T) -> Error
where
    T: Into<Box<dyn error::Error + Send + Sync>>,
{
    Error::UserError(
        description.to_string(),
        advice.to_string(),
        None,
        Some(internal.into()),
    )
}

/// An error triggered by the system rather than the user.
///
/// Constructs a new [Error] describing a failure which was the result of a failure
/// in the system, rather than a user's action. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
///
/// # Examples
/// ```
/// use human_errors;
/// 
/// human_errors::system(
///   "We could not open the config file you provided.",
///   "Make sure that the file exists and is readable by the application."
/// );
/// ```
pub fn system(description: &str, advice: &str) -> Error {
    Error::SystemError(description.to_string(), advice.to_string(), None, None)
}

/// An error triggered by the system rather than the user, with a deeper cause.
///
/// Constructs a new [Error] describing a failure which was the result of a failure
/// in the system, rather than a user's action. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
/// It also includes the details of another error which resulted in this failure,
/// as well as any advice that error may provide.
///
/// # Examples
/// ```
/// use human_errors;
/// 
/// human_errors::system_with_cause(
///   "We could not open the config file you provided.",
///   "Make sure that you've specified a valid config file with the --config option.",
///   human_errors::system(
///     "We could not find a file at /home/user/.config/demo.yml",
///     "Make sure that the file exists and is readable by the application."
///   )
/// );
/// ```
pub fn system_with_cause(description: &str, advice: &str, cause: Error) -> Error {
    Error::SystemError(
        description.to_string(),
        advice.to_string(),
        Some(Box::from(cause)),
        None,
    )
}

/// An error triggered by the system rather than the user, with a deeper cause.
///
/// Constructs a new [Error] describing a failure which was the result of a failure
/// in the system, rather than a user's action. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
/// It also includes the details of another error which resulted in this failure.
///
/// **NOTE**: The internal error may be any type which may be converted into a [Box<std::error::Error>].
///
/// # Examples
/// ```
/// use human_errors;
/// 
/// human_errors::system_with_internal(
///   "We could not open the config file you provided.",
///   "Make sure that the file exists and is readable by the application.",
///   human_errors::detailed_message("ENOENT 2: No such file or directory")
/// );
/// ```
pub fn system_with_internal<T>(description: &str, advice: &str, internal: T) -> Error
where
    T: Into<Box<dyn error::Error + Send + Sync>>,
{
    Error::SystemError(
        description.to_string(),
        advice.to_string(),
        None,
        Some(internal.into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description() {
        assert_eq!(
            user(
                "Something bad happened",
                "Avoid bad things happening in future"
            )
            .description(),
            "Something bad happened"
        );

        assert_eq!(
            system(
                "Something bad happened",
                "Avoid bad things happening in future"
            )
            .description(),
            "Something bad happened"
        );
    }

    #[test]
    fn test_message_basic() {
        assert_eq!(
            user(
                "Something bad happened.",
                "Avoid bad things happening in future"
            )
            .message(),
            "Oh no! Something bad happened.\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );

        assert_eq!(
            system(
                "Something bad happened.",
                "Avoid bad things happening in future"
            )
            .message(),
            "Whoops! Something bad happened. (This isn't your fault)\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );
    }

    #[test]
    fn test_message_cause() {
        assert_eq!(
            user_with_cause(
                "Something bad happened.",
                "Avoid bad things happening in future",
                user("You got rate limited by GitHub.", "Wait a few minutes and try again.")
            )
            .message(),
            "Oh no! Something bad happened.\n\nThis was caused by:\n - You got rate limited by GitHub.\n\nTo try and fix this, you can:\n - Wait a few minutes and try again.\n - Avoid bad things happening in future"
        );

        assert_eq!(
            system_with_cause(
                "Something bad happened.",
                "Avoid bad things happening in future",
                system("You got rate limited by GitHub.", "Wait a few minutes and try again.")
            )
            .message(),
            "Whoops! Something bad happened. (This isn't your fault)\n\nThis was caused by:\n - You got rate limited by GitHub.\n\nTo try and fix this, you can:\n - Wait a few minutes and try again.\n - Avoid bad things happening in future"
        );
    }
}
