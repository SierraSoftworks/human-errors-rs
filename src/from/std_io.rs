use crate::{Error, wrap_system, wrap_user};
use std::io;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => wrap_user(
                err,
                "Could not find the requested file.",
                &["Check that the file path you provided is correct and try again."],
            ),
            io::ErrorKind::PermissionDenied => wrap_user(
                err,
                "Permission denied when trying to access the requested resource.",
                &["Check the file permissions and ensure that the application has access to the resource."],
            ),
            io::ErrorKind::AlreadyExists => wrap_user(
                err,
                "The file or directory you are trying to create already exists.",
                &["Choose a different file name or delete the existing file and try again."],
            ),
            io::ErrorKind::AddrInUse => wrap_user(
                err,
                "The network address you are trying to bind to is already in use.",
                &["Make sure no other application is using the same address and try again."],
            ),
            io::ErrorKind::DirectoryNotEmpty => wrap_user(
                err,
                "The directory you are trying to remove is not empty.",
                &["Delete all files and subdirectories within the directory before attempting to remove it."],
            ),
            _ => wrap_system(
                err,
                "An internal error occurred which we could not recover from.",
                &["Please read the internal error below and decide if there is something you can do to fix the problem, or report it to us on GitHub."],
            ),
        }
    }
}
