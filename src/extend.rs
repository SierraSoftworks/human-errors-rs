/// Create a shim error type between [`human_errors::Error`] and other error types.
///
/// # Examples
/// ```
/// human_errors::error_shim!(MyError);
///
/// impl From<std::num::ParseIntError> for MyError {
///   fn from(err: std::num::ParseIntError) -> Self {
///     user_with_internal(
///       "We could not parse the number you provided.",
///       "Make sure that you're providing a number in the form 12345 or -12345.",
///       err,
///     )
///   }    
/// }
/// ```
#[macro_export]
macro_rules! error_shim {
    ($type:ident) => {
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
        #[allow(dead_code)]
        pub fn user(description: &str, advice: &str) -> $type {
            $crate::user(description, advice).into()
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
        #[allow(dead_code)]
        pub fn user_with_cause(description: &str, advice: &str, cause: $type) -> $type {
            $crate::user_with_cause(description, advice, cause.into()).into()
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
        #[allow(dead_code)]
        pub fn user_with_internal<T>(description: &str, advice: &str, internal: T) -> $type
        where
            T: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
            $crate::user_with_internal(description, advice, internal).into()
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
        #[allow(dead_code)]
        pub fn system(description: &str, advice: &str) -> $type {
            $crate::system(description, advice).into()
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
        #[allow(dead_code)]
        pub fn system_with_cause(description: &str, advice: &str, cause: $type) -> $type {
            $crate::system_with_cause(description, advice, cause.into()).into()
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
        #[allow(dead_code)]
        pub fn system_with_internal<T>(description: &str, advice: &str, internal: T) -> $type
        where
            T: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
            $crate::system_with_internal(description, advice, internal).into()
        }

        /// The fundamental error type used by this library.
        ///
        /// An error type which encapsulates information about whether an error
        /// is the result of something the user did, or a system failure outside
        /// of their control. These errors include a description of what occurred,
        /// advice on how to proceed and references to the causal chain which led
        /// to this failure.
        ///
        /// # Examples
        /// ```
        /// let err = human_errors::user(
        ///   "We could not open the config file you provided.",
        ///   "Make sure that the file exists and is readable by the application.",
        /// );
        ///
        /// // Prints the error and any advice for the user.
        /// println!("{}", err)
        /// ```
        #[derive(Debug)]
        pub struct $type($crate::Error);

        impl From<$crate::Error> for $type {
            fn from(err: $crate::Error) -> Self {
                Self(err)
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<$crate::Error> for $type {
            fn into(self) -> $crate::Error {
                self.0
            }
        }

        #[allow(dead_code)]
        impl $type {
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
                self.0.description()
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
                self.0.message()
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
                self.0.is_user()
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
                self.0.is_system()
            }
        }

        impl std::error::Error for $type {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.0.source()
            }
        }

        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    error_shim!(MyError);

    impl From<std::num::ParseIntError> for MyError {
        fn from(err: std::num::ParseIntError) -> Self {
            user_with_internal(
                "We could not parse the number you provided.",
                "Make sure that you're providing a number in the form 12345 or -12345.",
                err,
            )
        }
    }

    #[test]
    fn test_error_conversion() {
        let err = user("Something exploded.", "Don't blow it up in future.");

        assert_eq!(err.description(), "Something exploded.");
    }
}
