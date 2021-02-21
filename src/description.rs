use std::fmt;

/// Generates an error with the given `message`.
///
/// Generates a [std::error::Error] compatible error for the given
/// message. Can be used as the internal error for an [Error].
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
pub fn detailed_message(message: &str) -> BasicInternalError {
    message.into()
}

#[derive(Debug)]
pub struct BasicInternalError {
    message: String,
}

impl From<&str> for BasicInternalError {
    fn from(s: &str) -> Self {
        Self {
            message: s.to_string(),
        }
    }
}

impl std::error::Error for BasicInternalError {}

impl fmt::Display for BasicInternalError {
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
            user_with_internal(
                "Something bad happened.",
                "Avoid bad things happening in future",
                detailed_message("You got rate limited")
            )
            .message(),
            "Oh no! Something bad happened.\n\nThis was caused by:\n - You got rate limited\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );

        assert_eq!(
            system_with_internal(
                "Something bad happened.",
                "Avoid bad things happening in future",
                detailed_message("You got rate limited")
            )
            .message(),
            "Whoops! Something bad happened. (This isn't your fault)\n\nThis was caused by:\n - You got rate limited\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );
    }
}
