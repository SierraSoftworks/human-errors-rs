//! Previews the pretty (CLI-formatted) error output.
//!
//! Requires the `pretty` feature. Run with:
//!
//! ```sh
//! cargo run --example pretty --features pretty
//! ```

fn main() {
    let err = build_error();

    // `pretty` renders a richly formatted view of the error, its causal chain
    // and the aggregated advice for the user.
    println!("{}", human_errors::pretty(&err));
}

/// Builds a representative, multi-layered error for demonstration purposes.
fn build_error() -> human_errors::Error {
    human_errors::wrap_user(
        human_errors::wrap_system(
            std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "os error 13: permission denied",
            ),
            "Failed to read the configuration file at /etc/demo/config.yml.",
            &["Ensure the application has permission to read the file."],
        ),
        "We could not load your configuration.",
        &["Check that the --config option points to a readable file."],
    )
}
