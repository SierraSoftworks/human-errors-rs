use crate::{Error, user_with_internal};
use std::string::FromUtf8Error;

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        user_with_internal(
            "We could not parse the UTF-8 content you provided.",
            "Make sure that you are providing us with content which is valid UTF-8.",
            err,
        )
    }
}
