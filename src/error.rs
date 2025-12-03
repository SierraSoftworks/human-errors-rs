use std::{error, fmt};

/// The kind of error which occurred.
///
/// Distinguishes between errors which were the result of user actions
/// and those which were the result of system failures. Conceptually
/// similar to HTTP status codes in that 4xx errors are user-caused
/// and 5xx errors are system-caused.
#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    /// An error which was the result of actions that the user took.
    ///
    /// These errors are usually things which a user can easily resolve by
    /// changing how they interact with the system. Advice should be used
    /// to guide the user to the correct interaction paths and help them
    /// self-mitigate without needing to open support tickets.
    ///
    /// These errors are usually generated with [`crate::user`], [`crate::user_with_cause`]
    /// and [`crate::user_with_internal`].
    User,

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
    System,
}

impl Kind {
    fn format_description(&self, description: &str) -> String {
        match self {
            Kind::User => format!("Oh no! {description}"),
            Kind::System => format!("Whoops! {description} (This isn't your fault)"),
        }
    }
}

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
/// let err = human_errors::user_error(
///   "We could not open the config file you provided.",
///   &["Make sure that the file exists and is readable by the application."],
/// );
///
/// // Prints the error and any advice for the user.
/// println!("{}", err)
/// ```
#[derive(Debug)]
pub struct Error {
    kind: Kind,
    error: Box<dyn error::Error + Send + Sync>,
    advice: &'static [&'static str],
}

impl Error {
    /// Constructs a new [Error].
    ///
    /// # Examples
    /// ```
    /// use human_errors;
    /// let internal_error = human_errors::basic("Low-level IO error details");
    /// let err = human_errors::Error::new(
    ///     internal_error,
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

    /// Gets the description message from this error.
    ///
    /// Gets the description which was provided as the first argument when constructing
    /// this error.
    ///
    /// # Examples
    /// ```
    /// use human_errors;
    ///
    /// let err = human_errors::user_error(
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
    ///   human_errors::user_error(
    ///     "We could not find a file at /home/user/.config/demo.yml",
    ///     &["Make sure that the file exists and is readable by the application."]
    ///   ),
    ///   "We could not open the config file you provided.",
    ///   &["Make sure that you've specified a valid config file with the --config option."],
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

    fn advice(&self) -> Vec<&'static str> {
        let mut advice = self.advice.to_vec();

        let mut cause = self.error.as_ref();
        while let Some(err) = cause.downcast_ref::<Error>() {
            advice.extend_from_slice(&err.advice);
            cause = err.error.as_ref();
        }

        advice.reverse();

        advice
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
    /// let err = human_errors::user_error(
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
