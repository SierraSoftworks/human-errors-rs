use std::borrow::Cow;

use super::*;

/// Extension trait for `Option` to convert `None` values into user-friendly 
/// error types.
/// 
/// # Examples
/// ```
/// use human_errors::OptionExt;
/// 
/// // Converts a `None` value into a user-caused error with the provided message and advice.
/// let value = None::<i32>.ok_or_user_err(
///    "No value was provided.",
///    &["Please provide a valid integer value."],
/// );
/// 
/// // Converts a `None` value into a system-caused error with the provided message and advice.
/// let value = None::<i32>.ok_or_system_err(
///   "No value was provided.",
///   &["Please check your system configuration."],
/// );
/// ```
pub trait OptionExt<T> {
    /// Converts an `Option<T>` into a `Result<T, Error>`, returning a user-caused
    /// error with the provided message and advice if the option is `None`.
    ///
    /// # Examples
    /// ```
    /// use human_errors::OptionExt;
    /// 
    /// let value = None::<i32>.ok_or_user_err(
    ///   "No value was provided.",
    ///   &["Please provide a valid integer value."],
    /// );
    /// ```
    fn ok_or_user_err<S: Into<Cow<'static, str>>>(self, msg: S, advice: &'static [&'static str]) -> Result<T, Error>;

    /// Converts an `Option<T>` into a `Result<T, Error>`, returning a system-caused
    /// error with the provided message and advice if the option is `None`.
    /// 
    /// # Examples
    /// ```
    /// use human_errors::OptionExt;
    /// 
    /// let value = None::<i32>.ok_or_system_err(
    ///   "No value was provided.",
    ///   &["Please check your system configuration."], 
    /// );
    /// ```
    fn ok_or_system_err<S: Into<Cow<'static, str>>>(self, msg: S, advice: &'static [&'static str]) -> Result<T, Error>;
}

impl <T> OptionExt<T> for Option<T> {
    fn ok_or_user_err<S: Into<Cow<'static, str>>>(self, msg: S, advice: &'static [&'static str]) -> Result<T, Error> {
        match self {
            Some(value) => Ok(value),
            None => Err(user(msg.into(), advice)),
        }
    }

    fn ok_or_system_err<S: Into<Cow<'static, str>>>(self, msg: S, advice: &'static [&'static str]) -> Result<T, Error> {
        match self {
            Some(value) => Ok(value),
            None => Err(system(msg.into(), advice)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_or_user_err_some() {
        let value = Some(42).ok_or_user_err("No value", &["Provide a value"]).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_ok_or_user_err_none() {
        let err = None::<i32>.ok_or_user_err("No value", &["Provide a value"]).unwrap_err();
        assert!(err.is(Kind::User));
        assert_eq!(err.message(), "No value");
    }

    #[test]
    fn test_ok_or_system_err_some() {
        let value = Some(42).ok_or_system_err("No value", &["Check system"]).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_ok_or_system_err_none() {
        let err = None::<i32>.ok_or_system_err("No value", &["Check system"]).unwrap_err();
        assert!(err.is(Kind::System));
        assert_eq!(err.message(), "No value");
    }
}