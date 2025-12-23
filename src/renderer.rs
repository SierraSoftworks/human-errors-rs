use std::fmt::Display;

use super::*;

/// Print the given error to stdout using the appropriate renderer.
///
/// Depending on whether the `cli` feature is enabled, this will
/// either print a simple error message or a more complex,
/// formatted error message suitable for CLI applications.
///
/// # Examples
/// ```no_run
/// use human_errors;
///
/// let err = human_errors::user(
///   "We could not open the config file you provided.",
///   &["Make sure that the file exists and is readable by the application."],
/// );
///
/// human_errors::println(&err);
/// ```
pub fn println(err: &Error) {
    println!("{}", Renderer { error: err })
}

/// Print the given error to stderr using the appropriate renderer.
///
/// Depending on whether the `cli` feature is enabled, this will
/// either print a simple error message or a more complex,
/// formatted error message suitable for CLI applications.
///
/// # Examples
/// ```no_run
/// use human_errors;
///
/// let err = human_errors::user(
///   "We could not open the config file you provided.",
///   &["Make sure that the file exists and is readable by the application."],
/// );
///
/// human_errors::eprintln(&err);
/// ```
pub fn eprintln(err: &Error) {
    eprintln!("{}", Renderer { error: err })
}

struct Renderer<'a> {
    error: &'a Error,
}

impl Display for Renderer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "cli"))]
        {
            write!(f, "{}", self.error)
        }

        #[cfg(feature = "cli")]
        {
            use colored::Colorize;
            use std::error::Error;

            const WIDTH: usize = 80;

            write!(f, "error({}):    ", format_kind(&self.error.kind))?;
            write_wrapped(
                f,
                self.error.description(),
                WIDTH - 14,
                ("", ""),
                (&format!("{}{}", "│".bright_black(), " ".repeat(14)), ""),
            )?;

            let mut source = self.error.source();
            while let Some(cause) = source {
                writeln!(f, "{}", "│".bright_black())?;

                source = cause.source();
                let prefix = if source.is_some() { "├─" } else { "╰─" };
                let description = if let Some(err) = cause.downcast_ref::<super::Error>() {
                    write!(
                        f,
                        "{} cause({}): ",
                        prefix.bright_black(),
                        format_kind(&err.kind)
                    )?;
                    err.description()
                } else {
                    write!(
                        f,
                        "{}{} cause: ",
                        prefix.bright_black(),
                        "─".repeat(5).bright_black()
                    )?;
                    cause.to_string()
                };
                write_wrapped(
                    f,
                    description,
                    WIDTH - 14,
                    ("".bright_black().as_ref(), ""),
                    (
                        &format!("{}{}", "│".bright_black(), " ".repeat(13)).bright_black(),
                        "",
                    ),
                )?;
            }

            let advice = self.error.advice();

            if !advice.is_empty() {
                writeln!(f)?;
                write_box(
                    f,
                    "Advice",
                    format!(" • {}", advice.join("\n • ")),
                    cli_boxes::BoxChars::ROUND,
                    WIDTH,
                )?;
            }

            Ok(())
        }
    }
}

#[cfg(feature = "cli")]
fn format_kind(kind: &Kind) -> colored::ColoredString {
    use colored::Colorize;

    match kind {
        Kind::System => "sys".red(),
        Kind::User => "usr".yellow(),
    }
}

#[cfg(feature = "cli")]
fn write_wrapped<D: Display + Copy>(
    f: &mut std::fmt::Formatter<'_>,
    content: impl AsRef<str>,
    width: usize,
    first_line: (D, D),
    other_lines: (D, D),
) -> std::fmt::Result {
    use colored::Colorize;

    let mut first = true;
    for chunk in textwrap::wrap(content.as_ref(), width) {
        let (prefix, suffix) = if first {
            first = false;
            first_line
        } else {
            other_lines
        };
        writeln!(
            f,
            "{}{}{}{}",
            prefix,
            chunk.bright_white(),
            " ".repeat(width.saturating_sub(chunk.len())),
            suffix
        )?;
    }

    Ok(())
}

#[cfg(feature = "cli")]
fn write_box(
    f: &mut std::fmt::Formatter<'_>,
    title: &str,
    content: impl AsRef<str>,
    box_chars: cli_boxes::BoxChars,
    width: usize,
) -> std::fmt::Result {
    use colored::Colorize;

    {
        let title_padding = vec![box_chars.top; width - title.len() - 5]
            .into_iter()
            .collect::<String>();
        writeln!(
            f,
            "{}{} {} {}{}",
            box_chars.top_left,
            box_chars.top,
            title.blue(),
            title_padding,
            box_chars.top_right,
        )?;
    }

    for line in content.as_ref().lines() {
        write_wrapped(
            f,
            line,
            width,
            (&box_chars.left, &box_chars.right),
            (&box_chars.left, &box_chars.right),
        )?;
    }

    {
        let bottom_padding = vec![box_chars.bottom; width - 2]
            .into_iter()
            .collect::<String>();
        writeln!(
            f,
            "{}{}{}",
            box_chars.bottom_left, bottom_padding, box_chars.bottom_right,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_basic() {
        let user_error = user(
            "Something bad happened.",
            &["Avoid bad things happening in future"],
        );

        let system_error = system(
            "Something bad happened.",
            &["Avoid bad things happening in future"],
        );

        let user_rendered = format!("{}", Renderer { error: &user_error });
        let system_rendered = format!(
            "{}",
            Renderer {
                error: &system_error
            }
        );

        println!("{}", user_rendered);

        assert!(user_rendered.contains("Something bad happened."));
        assert!(user_rendered.contains("Avoid bad things happening in future"));

        println!("{}", system_rendered);
        assert!(system_rendered.contains("Something bad happened."));
        assert!(system_rendered.contains("Avoid bad things happening in future"));
    }

    #[test]
    fn test_renderer_with_cause() {
        let underlying_error = std::io::Error::other("underlying IO error");
        let wrapped_error = wrap_user(
            underlying_error,
            "Failed to read configuration file.",
            &["Ensure the file exists and is readable."],
        );

        let root_error = wrap_user(
            wrapped_error,
            "Could not start application due to a problem which resulted in an extremely long error message which we'd like to wrap nicely if possible because otherwise it's going to result in weird and broken formatting on some systems.",
            &["Check your configuration settings."],
        );

        let rendered = format!("{}", Renderer { error: &root_error });

        println!("{}", rendered);

        assert!(rendered.contains("Failed to read configuration file."));
        assert!(rendered.contains("underlying IO error"));
        assert!(rendered.contains("Ensure the file exists and is readable."));
        assert!(rendered.contains("Check your configuration settings."));
    }
}
