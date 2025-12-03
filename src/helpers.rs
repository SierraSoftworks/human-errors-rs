pub use super::{Error, Kind, basic};
use std::{borrow::Cow, error};

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
/// human_errors::user_error(
///   "We could not open the config file you provided.",
///   &["Make sure that the file exists and is readable by the application."],
/// );
/// ```
pub fn user_error<S: Into<Cow<'static, str>>>(description: S, advice: &'static [&'static str]) -> Error {
    Error::new(
        basic(description.into()),
        Kind::User,
        advice
    )
}

/// An error triggered by something the user has done, with a deeper cause.
///
/// Constructs a new [Error] describing a failure which was the result of an
/// action that the user has taken. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
/// It also includes the details of another error which resulted in this failure.
///
/// **NOTE**: The internal error may be any type which may be converted into a [Box<dyn error::Error>].
///
/// # Examples
/// ```
/// use human_errors;
///
/// human_errors::user(
///   human_errors::basic("ENOENT 2: No such file or directory"),
///   &["Make sure that the file exists and is readable by the application."],
/// );
/// ```
pub fn user<T>(error: T, advice: &'static [&'static str]) -> Error
where
    T: Into<Box<dyn error::Error + Send + Sync>>,
{
    Error::new(
        error.into(),
        Kind::User,
        advice,
    )
}

/// An error triggered by something the user has done, with a deeper cause.
/// 
/// Constructs a new [Error] describing a failure which was the result of an
/// action that the user has taken. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
/// It also includes the details of another error which resulted in this failure.
/// 
/// **NOTE**: The internal error may be any type which may be converted into a [Box<dyn error::Error>].
/// 
/// # Examples
/// ```
/// use human_errors;
/// 
/// human_errors::wrap_user(
///  human_errors::system_error("The configuration file was not found.", &["Make sure that the file exists and try again."]),
///  "We could not open the config file you provided.",
///  &["Make sure that the file exists and is readable by the application."],
/// );
/// ```
pub fn wrap_user<S: Into<Cow<'static, str>> + 'static, E: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + 'static>(
    inner: E,
    message: S,
    advice: &'static [&'static str],
) -> Error {
    Error::new(
        super::wrap(message, inner),
        Kind::User,
        advice,
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
/// human_errors::system_error(
///   "We could not open the config file you provided.",
///   &["Make sure that the file exists and is readable by the application."]
/// );
/// ```
pub fn system_error<S: Into<Cow<'static, str>>>(description: S, advice: &'static [&'static str]) -> Error {
    Error::new(
        basic(description.into()),
        Kind::System,
        advice
    )
}

/// An error triggered by the system rather than the user, with a deeper cause.
///
/// Constructs a new [Error] describing a failure which was the result of a failure
/// in the system, rather than a user's action. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
/// It also includes the details of another error which resulted in this failure.
///
/// **NOTE**: The internal error may be any type which may be converted into a [Box<dyn error::Error>].
///
/// # Examples
/// ```
/// use human_errors;
///
/// human_errors::system(
///   human_errors::basic("ENOENT 2: No such file or directory"),
///   &["Make sure that the file exists and is readable by the application."],
/// );
/// ```
pub fn system<T>(error: T, advice: &'static [&'static str]) -> Error
where
    T: Into<Box<dyn error::Error + Send + Sync>>,
{
    Error::new(
        error.into(),
        Kind::System,
        advice,
    )
}

/// An error triggered by the system rather than the user, with a deeper cause.
/// 
/// Constructs a new [Error] describing a failure which was the result of a failure
/// in the system, rather than a user's action. This error includes a description of what
/// occurred, as well as some advice for the user to try to mitigate the problem.
/// It also includes the details of another error which resulted in this failure.
/// 
/// **NOTE**: The internal error may be any type which may be converted into a [Box<dyn error::Error>].
/// 
/// # Examples
/// ```
/// use human_errors;
/// human_errors::wrap_system(
///  human_errors::user_error("The configuration file was not found.", &["Make sure that the file exists and try again."]),
///  "We could not open the config file you provided.",
///  &["Make sure that the file exists and is readable by the application."],
/// );
/// ```
pub fn wrap_system<S: Into<Cow<'static, str>> + 'static, E: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + 'static>(
    inner: E,
    message: S,
    advice: &'static [&'static str],
) -> Error {
    Error::new(
        super::wrap(message, inner),
        Kind::System,
        advice,
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
                &["Avoid bad things happening in future"]
            )
            .description(),
            "Something bad happened"
        );

        assert_eq!(
            system(
                "Something bad happened",
                &["Avoid bad things happening in future"]
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
                &["Avoid bad things happening in future"]
            )
            .message(),
            "Oh no! Something bad happened.\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );

        assert_eq!(
            system(
                "Something bad happened.",
                &["Avoid bad things happening in future"]
            )
            .message(),
            "Whoops! Something bad happened. (This isn't your fault)\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );
    }

    #[test]
    fn test_message_wrapped() {
        assert_eq!(
            wrap_user(
                basic("You got rate limited"),
                "Something bad happened.",
                &["Avoid bad things happening in future"]
            )
            .message(),
            "Oh no! Something bad happened.\n\nThis was caused by:\n - You got rate limited\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );

        assert_eq!(
            wrap_system(
                basic("You got rate limited"),
                "Something bad happened.",
                &["Avoid bad things happening in future"]
            )
            .message(),
            "Whoops! Something bad happened. (This isn't your fault)\n\nThis was caused by:\n - You got rate limited\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );
    }
}
