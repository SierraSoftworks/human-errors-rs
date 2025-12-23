
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
    pub(crate) fn format_description(&self, description: &str) -> String {
        match self {
            Kind::User => format!("{description} (User error)"),
            Kind::System => format!("{description} (System failure)"),
        }
    }
}