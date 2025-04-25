use std::{error, fmt};

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
///   "Make sure that the file exists and is readable by the application.",
/// );
///
/// // Prints the error and any advice for the user.
/// println!("{}", err)
/// ```
#[derive(Debug)]
pub enum Error {
    /// An error which was the result of actions that the user took.
    ///
    /// These errors are usually things which a user can easily resolve by
    /// changing how they interact with the system. Advice should be used
    /// to guide the user to the correct interaction paths and help them
    /// self-mitigate without needing to open support tickets.
    ///
    /// These errors are usually generated with [`crate::user`], [`crate::user_with_cause`]
    /// and [`crate::user_with_internal`].
    UserError(
        String,
        String,
        Option<Box<Error>>,
        Option<Box<dyn error::Error + Send + Sync>>,
    ),

    /// An error which was the result of the system failing rather than the user's actions.
    ///
    /// These kinds of issues are usually the result of the system entering
    /// an unexpected state and/or violating an assumption on behalf of the
    /// developer. Often these issues cannot be resolved by the user directly,
    /// so the advice should guide them to the best way to raise a bug with you
    /// and provide you with information to help them fix the issue.
    ///
    /// These errors are usually generated with [`crate::system`], [`crate::system_with_cause`]
    /// and [`crate::system_with_internal`].
    SystemError(
        String,
        String,
        Option<Box<Error>>,
        Option<Box<dyn error::Error + Send + Sync>>,
    ),
}

impl Error {
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
    ///   "Make sure that the file exists and is readable by the application.",
    /// );
    ///
    /// // Prints: "We could not open the config file you provided."
    /// println!("{}", err.description())
    /// ```
    pub fn description(&self) -> String {
        match self {
            Error::UserError(description, ..) | Error::SystemError(description, ..) => {
                description.clone()
            }
        }
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
    /// let err = human_errors::user_with_cause(
    ///   "We could not open the config file you provided.",
    ///   "Make sure that you've specified a valid config file with the --config option.",
    ///   human_errors::user(
    ///     "We could not find a file at /home/user/.config/demo.yml",
    ///     "Make sure that the file exists and is readable by the application."
    ///   )
    /// );
    ///
    /// // Prints a message like the following:
    /// // Oh no! We could not open the config file you provided.
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
        let description = match self {
            Error::UserError(description, ..) | Error::SystemError(description, ..) => description,
        };

        let hero_message = match self {
            Error::UserError(_, _, _, _) => {
                format!("Oh no! {}", description)
            }
            Error::SystemError(_, _, _, _) => {
                format!("Whoops! {} (This isn't your fault)", description)
            }
        };

        match (self.caused_by(), self.advice()) {
            (Some(cause), Some(advice)) if !advice.is_empty() => {
                format!(
                    "{}\n\nThis was caused by:\n{}\n\nTo try and fix this, you can:\n{}",
                    hero_message, cause, advice
                )
            }
            (Some(cause), _) => {
                format!("{}\n\nThis was caused by:\n{}", hero_message, cause)
            }
            (None, Some(advice)) if !advice.is_empty() => {
                format!(
                    "{}\n\nTo try and fix this, you can:\n{}",
                    hero_message, advice
                )
            }
            _ => hero_message,
        }
    }

    fn caused_by(&self) -> Option<String> {
        match self {
            Error::UserError(.., Some(cause), _) | Error::SystemError(.., Some(cause), _) => {
                match cause.caused_by() {
                    Some(child_cause) => {
                        Some(format!(" - {}\n{}", cause.description(), child_cause))
                    }
                    None => Some(format!(" - {}", cause.description())),
                }
            }
            Error::UserError(.., Some(internal)) | Error::SystemError(.., Some(internal)) => {
                Some(self.internal_caused_by(internal.as_ref()))
            }
            _ => None,
        }
    }

    fn internal_caused_by(&self, error: &dyn error::Error) -> String {
        match error.source() {
            Some(source) => format!(" - {}\n{}", error, self.internal_caused_by(source)),
            None => format!(" - {}", error),
        }
    }

    fn advice(&self) -> Option<String> {
        let (advice, cause) = match self {
            Error::UserError(_, advice, cause, _) | Error::SystemError(_, advice, cause, _) => {
                (advice, cause)
            }
        };

        match cause {
            // We bias towards the most specific advice first (i.e. the lowest-level error) because that's most likely to be correct.
            Some(cause) => match (advice, cause.advice()) {
                (advice, Some(cause_advice)) if !advice.is_empty() && !cause_advice.is_empty() => {
                    Some(format!("{}\n - {}", cause_advice, advice))
                }
                (advice, _) if !advice.is_empty() => Some(format!(" - {}", advice)),
                (_, Some(cause_advice)) if !cause_advice.is_empty() => Some(cause_advice),
                _ => None,
            },
            None if !advice.is_empty() => Some(format!(" - {}", advice)),
            _ => None,
        }
    }

    /// Checks if this error is a user error.
    ///
    /// Returns `true` if this error is a [Error::UserError],
    /// otherwise `false`.
    ///
    /// # Examples
    /// ```
    /// use human_errors;
    ///
    /// let err = human_errors::user(
    ///   "We could not open the config file you provided.",
    ///   "Make sure that the file exists and is readable by the application.",
    /// );
    ///
    /// // Prints "is_user?: true"
    /// println!("is_user?: {}", err.is_user());
    /// ```
    pub fn is_user(&self) -> bool {
        matches!(self, Error::UserError(..))
    }

    /// Checks if this error is a system error.
    ///
    /// Returns `true` if this error is a [Error::SystemError],
    /// otherwise `false`.
    ///
    /// # Examples
    /// ```
    /// use human_errors;
    ///
    /// let err = human_errors::system(
    ///   "Failed to generate config file.",
    ///   "Please file an error report on GitHub."
    /// );
    ///
    /// // Prints "is_system?: true"
    /// println!("is_system?: {}", err.is_system());
    /// ```
    pub fn is_system(&self) -> bool {
        matches!(self, Error::SystemError(..))
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::UserError(.., Some(err)) | Error::SystemError(.., Some(err)) => {
                err.source()
            }
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
