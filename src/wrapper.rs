use std::{borrow::Cow, fmt};

/// Wraps an existing error with a basic message.
///
/// Generates a [std::error::Error] compatible error which wraps
/// the provided inner error with the given message. Can be used
/// as the internal error for a [crate::Error].
///
/// # Examples
/// ```
/// use human_errors;
///
/// human_errors::wrap(
///   "ENOENT 2: No such file or directory",
///   "We could not open the config file you provided."
/// );
/// ```
pub fn wrap<
    S: Into<Cow<'static, str>>,
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
>(
    inner: E,
    message: S,
) -> impl std::error::Error {
    let message = message.into();
    ErrorWithMessage {
        message,
        inner: Some(inner.into()),
    }
}

#[derive(Debug)]
struct ErrorWithMessage {
    message: Cow<'static, str>,
    inner: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::error::Error for ErrorWithMessage {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.inner {
            Some(inner) => Some(&**inner),
            None => None,
        }
    }
}

impl fmt::Display for ErrorWithMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_message_internal() {
        assert_eq!(
            user(
                wrap("You got rate limited", "Something bad happened."),
                &["Avoid bad things happening in future"],
            )
            .message(),
            "Oh no! Something bad happened.\n\nThis was caused by:\n - You got rate limited\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );

        assert_eq!(
            system(
                wrap("You got rate limited", "Something bad happened."),
                &["Avoid bad things happening in future"],
            )
            .message(),
            "Whoops! Something bad happened. (This isn't your fault)\n\nThis was caused by:\n - You got rate limited\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );
    }
}
