use std::{error, fmt};
use super::Kind;

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

/// The fundamental error type used by this library.
///
/// An error type which encapsulates information about whether an error
/// is the result of something the user did, or a system failure outside
/// their control. These errors include a description of what occurred,
/// advice on how to proceed and references to the causal chain which led
/// to this failure.
///
/// # Examples
/// ```
/// use human_errors;
///
/// let err = human_errors::user(
///   "We could not open the config file you provided.",
///   &["Make sure that the file exists and is readable by the application."],
/// );
///
/// // Prints the error and any advice for the user.
/// println!("{}", err)
/// ```
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: Kind,
    pub(crate) error: Box<dyn error::Error + Send + Sync>,
    pub(crate) advice: &'static [&'static str],
}

impl Error {
    /// Constructs a new [Error].
    ///
    /// # Examples
    /// ```
    /// use human_errors;
    /// let err = human_errors::Error::new(
    ///     "Low-level IO error details",
    ///     human_errors::Kind::System,
    ///     &["Try restarting the application", "If the problem persists, contact support"]
    /// );
    /// ```
    pub fn new<E: Into<Box<dyn error::Error + Send + Sync>>>(
        error: E,
        kind: Kind,
        advice: &'static [&'static str],
    ) -> Self {
        Self {
            error: error.into(),
            kind,
            advice,
        }
    }

    /// Checks if this error is of a specific kind.
    ///
    /// Returns `true` if this error matches the provided [Kind],
    /// otherwise `false`.
    ///
    /// # Examples
    /// ```
    /// use human_errors;
    ///
    /// let err = human_errors::user(
    ///   "We could not open the config file you provided.",
    ///   &["Make sure that the file exists and is readable by the application."],
    /// );
    ///
    /// // Prints "is_user?: true"
    /// println!("is_user?: {}", err.is(human_errors::Kind::User));
    /// ```
    pub fn is(&self, kind: Kind) -> bool {
        self.kind == kind
    }

    /// Gets the description message from this error.
    ///
    /// Gets the description which was provided as the first argument when constructing
    /// this error.
    ///
    /// # Examples
    /// ```
    /// use human_errors;
    ///
    /// let err = human_errors::user(
    ///   "We could not open the config file you provided.",
    ///   &["Make sure that the file exists and is readable by the application."],
    /// );
    ///
    /// // Prints: "We could not open the config file you provided."
    /// println!("{}", err.description())
    /// ```
    pub fn description(&self) -> String {
        match self.error.downcast_ref::<Error>() {
            Some(err) => err.description(),
            None => format!("{}", self.error),
        }
    }

    /// Gets the advice associated with this error and its causes.
    /// 
    /// Gathers all advice from this error and any causal errors it wraps,
    /// returning a deduplicated list of suggestions for how a user should
    /// deal with this error.
    /// 
    /// # Examples
    /// ```
    /// use human_errors;
    /// 
    /// let err = human_errors::wrap_user(
    ///   human_errors::user(
    ///     "We could not find a file at /home/user/.config/demo.yml",
    ///     &["Make sure that the file exists and is readable by the application."]
    ///   ),
    ///   "We could not open the config file you provided.",
    ///   &["Make sure that you've specified a valid config file with the --config option."],
    /// );
    /// 
    /// // Prints:
    /// // - Make sure that the file exists and is readable by the application.
    /// // - Make sure that you've specified a valid config file with the --config option.
    /// for tip in err.advice() {
    ///     println!("- {}", tip);
    /// }
    /// ``````
    pub fn advice(&self) -> Vec<&'static str> {
        let mut advice = self.advice.to_vec();

        let mut cause: Option<&(dyn std::error::Error + 'static)> = Some(self.error.as_ref());
        while let Some(err) = cause {
            if let Some(err) = err.downcast_ref::<Error>() {
                advice.extend_from_slice(err.advice);
            }
            
            cause = err.source();
        }

        advice.reverse();

        let mut seen = std::collections::HashSet::new();
        advice.retain(|item| seen.insert(*item));

        advice
    }

    /// Gets the formatted error and its advice.
    ///
    /// Generates a string containing the description of the error and any causes,
    /// as well as a list of suggestions for how a user should
    /// deal with this error. The "deepest" error's advice is presented first, with
    /// successively higher errors appearing lower in the list. This is done because
    /// the most specific error is the one most likely to have the best advice on how
    /// to resolve the problem.
    ///
    /// # Examples
    /// ```
    /// use human_errors;
    ///
    /// let err = human_errors::wrap_user(
    ///   human_errors::user(
    ///     "We could not find a file at /home/user/.config/demo.yml",
    ///     &["Make sure that the file exists and is readable by the application."]
    ///   ),
    ///   "We could not open the config file you provided.",
    ///   &["Make sure that you've specified a valid config file with the --config option."],
    /// );
    ///
    /// // Prints a message like the following:
    /// // We could not open the config file you provided. (User error)
    /// //
    /// // This was caused by:
    /// // We could not find a file at /home/user/.config/demo.yml
    /// //
    /// // To try and fix this, you can:
    /// //  - Make sure that the file exists and is readable by the application.
    /// //  - Make sure that you've specified a valid config file with the --config option.
    /// println!("{}", err.message());
    /// ```
    pub fn message(&self) -> String {
        let description = self.description();
        let hero_message = self.kind.format_description(&description);

        match (self.caused_by(), self.advice()) {
            (cause, advice) if !cause.is_empty() && !advice.is_empty() => {
                format!(
                    "{}\n\nThis was caused by:\n - {}\n\nTo try and fix this, you can:\n - {}",
                    hero_message,
                    cause.join("\n - "),
                    advice.join("\n - ")
                )
            }
            (cause, _) if !cause.is_empty() => {
                format!(
                    "{}\n\nThis was caused by:\n - {}",
                    hero_message,
                    cause.join("\n - ")
                )
            }
            (_, advice) if !advice.is_empty() => {
                format!(
                    "{}\n\nTo try and fix this, you can:\n - {}",
                    hero_message,
                    advice.join("\n - ")
                )
            }
            _ => hero_message,
        }
    }

    fn caused_by(&self) -> Vec<String> {
        let mut causes = Vec::new();
        let mut current_error: &dyn error::Error = self.error.as_ref();
        while let Some(err) = current_error.source() {
            if let Some(err) = err.downcast_ref::<Error>() {
                causes.push(err.description());
            } else {
                causes.push(format!("{}", err));
            }

            current_error = err;
        }

        causes
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.error.source()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 3)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("advice", &self.advice())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_user_error() {
        let err = Error::new(
            "Something bad happened.",
            Kind::User,
            &["Avoid bad things happening in future"],
        );

        assert!(err.is(Kind::User));
        assert_eq!(err.description(), "Something bad happened.");
        assert_eq!(
            err.message(),
            "Something bad happened. (User error)\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );
    }

    #[test]
    fn test_basic_system_error() {
        let err = Error::new(
            "Something bad happened.",
            Kind::System,
            &["Avoid bad things happening in future"],
        );

        assert!(err.is(Kind::System));
        assert_eq!(err.description(), "Something bad happened.");
        assert_eq!(
            err.message(),
            "Something bad happened. (System failure)\n\nTo try and fix this, you can:\n - Avoid bad things happening in future"
        );
    }

    #[test]
    fn test_advice_aggregation() {
        let low_level_err = Error::new(
            "Low-level failure.",
            Kind::System,
            &["Check low-level systems"],
        );

        let high_level_err = Error::new(
            low_level_err,
            Kind::User,
            &["Check high-level configuration"],
        );

        assert_eq!(
            high_level_err.advice(),
            vec!["Check low-level systems", "Check high-level configuration"]
        );
    }
}