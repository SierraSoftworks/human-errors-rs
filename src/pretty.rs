use std::fmt::Display;

use super::*;

/// Returns a displayable representation of the given error.
///
/// Depending on whether the `cli` feature is enabled, this will
/// either present a simple error message or a more complex,
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
/// println!("{}", human_errors::pretty(&err));
/// ```
pub fn pretty(err: &Error) -> impl Display {
    Renderer {
        error: err,
        width: 80,
        backtraces: false,
    }
}

/// Returns a displayable representation of the given error, including backtraces.
///
/// Behaves like [`pretty`], but additionally renders the backtrace captured for
/// the error as well as the backtrace of each causal error which recorded one.
///
/// Backtraces are only rendered when the `backtraces` (or `force_backtraces`)
/// feature is enabled and a backtrace was successfully captured (which usually
/// requires the `RUST_BACKTRACE` environment variable to be set). When no
/// backtraces are available, this produces the same output as [`pretty`].
///
/// # Examples
/// ```no_run
/// use human_errors;
///
/// let err = human_errors::system(
///   "We could not connect to the database.",
///   &["Check that the database is running and reachable."],
/// );
///
/// println!("{}", human_errors::pretty_with_backtraces(&err));
/// ```
pub fn pretty_with_backtraces(err: &Error) -> impl Display {
    Renderer {
        error: err,
        width: 120,
        backtraces: true,
    }
}

struct Renderer<'a> {
    error: &'a Error,
    width: usize,
    backtraces: bool,
}

impl Display for Renderer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        use std::error::Error;

        write!(f, "error({}):    ", format_kind(&self.error.kind))?;
        write_wrapped(
            f,
            self.error.description(),
            self.width - 14,
            0,
            " ".repeat(14),
            ("", ""),
            (&format!("{}{}", "│".bright_black(), ""), ""),
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
                self.width - 14,
                0,
                " ".repeat(14),
                ("".bright_black().as_ref(), ""),
                (&format!("{}{}", "│".bright_black(), "").bright_black(), ""),
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
                self.width,
            )?;
        }

        if self.backtraces {
            #[cfg(feature = "backtraces")]
            for (description, backtrace) in crate::backtraces::collect(self.error) {
                writeln!(f)?;
                write_box(
                    f,
                    "Backtrace",
                    format!("{description}\n\n{backtrace}"),
                    cli_boxes::BoxChars::ROUND,
                    self.width,
                )?;
            }
        }

        Ok(())
    }
}

fn format_kind(kind: &Kind) -> colored::ColoredString {
    use colored::Colorize;

    match kind {
        Kind::System => "sys".red(),
        Kind::User => "usr".yellow(),
    }
}

fn write_wrapped<D: Display + Copy>(
    f: &mut std::fmt::Formatter<'_>,
    content: impl AsRef<str>,
    width: usize,
    padding: usize,
    indent: impl AsRef<str>,
    first_line: (D, D),
    other_lines: (D, D),
) -> std::fmt::Result {
    use colored::Colorize;

    let mut first = true;
    let padding_str = " ".repeat(padding);
    for chunk in textwrap::wrap(
        content.as_ref(),
        textwrap::Options::new(width - 2 * padding)
            .break_words(true)
            .subsequent_indent(indent.as_ref()),
    ) {
        let (prefix, suffix) = if first {
            first = false;
            first_line
        } else {
            other_lines
        };
        writeln!(
            f,
            "{prefix}{padding_str}{}{}{padding_str}{suffix}",
            chunk.bright_white(),
            " ".repeat(width.saturating_sub(chunk.chars().count())),
        )?;
    }

    Ok(())
}

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
            width - 4,
            1,
            "  ",
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

        let user_rendered = format!("{}", pretty(&user_error));
        let system_rendered = format!("{}", pretty(&system_error));

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

        let rendered = format!("{}", pretty(&root_error));

        println!("{}", rendered);

        assert!(rendered.contains("Failed to read configuration file."));
        assert!(rendered.contains("underlying IO error"));
        assert!(rendered.contains("Ensure the file exists and is readable."));
        assert!(rendered.contains("Check your configuration settings."));
    }

    #[test]
    fn test_renderer_with_backtraces() {
        let err = wrap_system(
            system("Inner failure.", &["Check inner systems"]),
            "Outer failure.",
            &["Check outer configuration"],
        );

        let rendered = format!("{}", pretty_with_backtraces(&err));

        println!("{}", rendered);

        assert!(rendered.contains("Outer failure."));

        // The default renderer never emits backtraces.
        let plain = format!("{}", pretty(&err));
        assert!(!plain.contains("Backtrace"));

        #[cfg(feature = "force_backtraces")]
        {
            // A backtrace box is rendered for both the outer and inner errors,
            // adding two boxes on top of whatever the plain renderer produced.
            assert_eq!(
                rendered.matches('╭').count(),
                plain.matches('╭').count() + 2
            );
        }
    }
}
