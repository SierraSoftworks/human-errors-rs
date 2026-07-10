//! Previews both the default and pretty output formats with backtraces.
//!
//! Requires the `pretty` and `force_backtraces` features. Run with:
//!
//! ```sh
//! cargo run --example backtraces --features pretty,force_backtraces
//! ```

fn main() {
    let err = build_error();

    println!("== Default output with backtraces ==\n");
    println!("{}\n", err.message_with_backtrace());

    println!("== Pretty output with backtraces ==\n");
    println!("{}", human_errors::pretty_with_backtraces(&err));
}

/// Builds a representative, multi-layered error for demonstration purposes.
///
/// Each layer captures its own backtrace (when the `force_backtraces` feature
/// is enabled), which the output formats render recursively.
fn build_error() -> human_errors::Error {
    human_errors::wrap_user(
        human_errors::wrap_system(
            std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "os error 13: permission denied",
            ),
            "We were unable to read the configuration file at /etc/demo/config.yml due to an I/O failure.",
            &["Ensure the application has permission to read the file and that the file exists."],
        ),
        "We could not load your configuration.",
        &["Check that the --config option points to a readable file."],
    )
}
