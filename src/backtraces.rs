//! Parsing and simplification of captured backtraces for display.
//!
//! This module is only compiled when the `backtraces` feature is enabled. It is
//! responsible for collecting the backtraces recorded by an [`Error`] and its
//! causal chain, and for turning each captured [`std::backtrace::Backtrace`]
//! into a concise, readable rendering by trimming noise frames and stripping
//! redundant file paths.

use super::Error;

/// Symbol prefixes whose frames are dropped while they appear contiguously at
/// the top (innermost end) of a backtrace.
///
/// These frames belong to the machinery which captures the backtrace and to
/// this crate's own error constructors, none of which are useful when
/// diagnosing where an error actually originated.
const INITIAL_SKIP_PREFIXES: &[&str] = &["std::backtrace", "human_errors"];

/// Symbol prefixes whose frames retain their function name but have their file
/// path (the `at <path>` line) removed to reduce noise.
const HIDE_PATH_PREFIXES: &[&str] = &["core::", "std::"];

/// The standard library symbol which marks the boundary between user code and
/// the runtime/OS bootstrap that invoked it. Frames from this point toward the
/// bottom (outermost end) of the stack are dropped.
const RUNTIME_BOUNDARY_MARKER: &str = "std::sys::backtrace::__rust_begin_short_backtrace";

/// A single frame parsed from a backtrace's textual rendering.
struct BacktraceFrame<'a> {
    index: usize,
    symbol: &'a str,
    location: Option<&'a str>,
}

/// Collects the captured backtraces for an error and its causal chain.
///
/// Walks the error and each causal [`Error`] which recorded a backtrace,
/// pairing every captured backtrace with the description of the error it
/// belongs to. Errors without a successfully captured backtrace are skipped.
/// Each backtrace is simplified for readability (see [`simplify`]).
pub(crate) fn collect(error: &Error) -> Vec<(String, String)> {
    let mut backtraces = Vec::new();

    if let Some(backtrace) = captured(error) {
        backtraces.push((error.description(), simplify(backtrace)));
    }

    let mut cause: Option<&(dyn std::error::Error + 'static)> = Some(error.error.as_ref());
    while let Some(err) = cause {
        if let Some(err) = err.downcast_ref::<Error>() {
            if let Some(backtrace) = captured(err) {
                backtraces.push((err.description(), simplify(backtrace)));
            }
        }

        cause = err.source();
    }

    backtraces
}

/// Returns the backtrace captured for `error`, but only when a stack trace was
/// actually recorded (rather than being disabled or unsupported).
fn captured(error: &Error) -> Option<&std::backtrace::Backtrace> {
    error
        .backtrace
        .as_ref()
        .filter(|backtrace| backtrace.status() == std::backtrace::BacktraceStatus::Captured)
}

/// Simplifies a captured [backtrace](std::backtrace::Backtrace) for display.
///
/// The formatted output of the backtrace is reduced by:
///
/// - Skipping the backtrace-capture (`std::backtrace*`) and `human_errors::*`
///   frames at the top of the stack, so the trace starts at the caller which
///   created the error.
/// - Dropping the runtime and OS bootstrap frames at the bottom of the stack,
///   so the trace ends where user code begins executing.
/// - Removing the file path information from `core::*` and `std::*` frames,
///   which are rarely useful and add significant noise.
///
/// Wherever frames are skipped, a `... skipped N frames ...` annotation is
/// emitted in their place. Remaining frames retain their original offsets. If
/// nothing survives the filtering, the original (unfiltered) rendering is
/// returned.
fn simplify(backtrace: &std::backtrace::Backtrace) -> String {
    simplify_text(&backtrace.to_string())
}

/// Applies the backtrace simplification rules to the textual rendering of a
/// backtrace. See [`simplify`] for the rules which are applied.
fn simplify_text(raw: &str) -> String {
    let frames = parse_frames(raw);
    if frames.is_empty() {
        return raw.to_string();
    }

    // Drop the runtime/OS bootstrap frames at the bottom of the stack: everything
    // from the point where the standard library begins executing user code.
    let end = frames
        .iter()
        .position(|frame| frame.symbol.contains(RUNTIME_BOUNDARY_MARKER))
        .unwrap_or(frames.len());
    let bottom_skipped = frames.len() - end;

    // Drop the backtrace-capture and `human_errors::*` frames at the top of the
    // stack, so the trace starts at the caller which created the error.
    let start = frames[..end]
        .iter()
        .position(|frame| !starts_with_any(frame.symbol, INITIAL_SKIP_PREFIXES))
        .unwrap_or(end);
    let top_skipped = start;

    let frames = &frames[start..end];
    if frames.is_empty() {
        return raw.to_string();
    }

    let mut output = String::new();

    if top_skipped > 0 {
        output.push_str(&format!("    ... skipped {top_skipped} frames ...\n"));
    }

    for frame in frames {
        output.push_str(&format!("{:>2}: {}\n", frame.index, frame.symbol));

        if let Some(location) = frame.location {
            if !starts_with_any(frame.symbol, HIDE_PATH_PREFIXES) {
                output.push_str(&format!("    {location}\n"));
            }
        }
    }

    if bottom_skipped > 0 {
        output.push_str(&format!("    ... skipped {bottom_skipped} frames ...\n"));
    }

    output
}

/// Parses the textual rendering of a backtrace into its individual frames.
fn parse_frames(raw: &str) -> Vec<BacktraceFrame<'_>> {
    let mut frames = Vec::new();
    let mut lines = raw.lines().peekable();

    while let Some(line) = lines.next() {
        let Some((index, symbol)) = parse_frame(line.trim_start()) else {
            continue;
        };

        // A frame's source location is rendered on the following `at <path>` line.
        let location = match lines.peek() {
            Some(next) if next.trim_start().starts_with("at ") => {
                Some(lines.next().unwrap().trim_start())
            }
            _ => None,
        };

        frames.push(BacktraceFrame {
            index,
            symbol,
            location,
        });
    }

    frames
}

/// Parses a backtrace frame header line of the form `<index>: <symbol>` into its
/// original frame offset and demangled symbol, returning `None` for other lines.
fn parse_frame(line: &str) -> Option<(usize, &str)> {
    let (index, symbol) = line.split_once(": ")?;
    Some((index.parse().ok()?, symbol))
}

/// Returns `true` if `symbol` starts with any of the provided prefixes.
fn starts_with_any(symbol: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| symbol.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_text() {
        let raw = "\
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/library/std/src/backtrace.rs:1
   1: std::backtrace::Backtrace::create
             at /rustc/library/std/src/backtrace.rs:2
   2: human_errors::error::Error::new
             at ./src/error.rs:57
   3: human_errors::helpers::wrap_system
             at ./src/helpers.rs:110
   4: my_app::do_work
             at ./src/main.rs:20
   5: my_app::main
             at ./src/main.rs:10
   6: core::ops::function::FnOnce::call_once
             at /rustc/library/core/src/ops/function.rs:250
   7: std::rt::lang_start_internal
             at /rustc/library/std/src/rt.rs:175
   8: main";

        let simplified = simplify_text(raw);

        // The capture machinery and human_errors frames at the top are dropped
        // and replaced with a single annotation, while surviving frames keep
        // their original offsets.
        assert!(!simplified.contains("std::backtrace"));
        assert!(!simplified.contains("human_errors::"));
        assert!(simplified.contains("... skipped 4 frames ..."));
        assert!(simplified.contains(" 4: my_app::do_work"));

        // Application frames keep their file paths.
        assert!(simplified.contains("at ./src/main.rs:20"));

        // core::* and std::* frames keep their symbol but lose their file path.
        assert!(simplified.contains("core::ops::function::FnOnce::call_once"));
        assert!(simplified.contains("std::rt::lang_start_internal"));
        assert!(!simplified.contains("at /rustc/library/core/src/ops/function.rs:250"));
        assert!(!simplified.contains("at /rustc/library/std/src/rt.rs:175"));

        // A frame without a location line is still preserved.
        assert!(simplified.contains("main\n"));
    }

    #[test]
    fn test_simplify_text_strips_runtime_frames() {
        let raw = "\
   0: human_errors::error::Error::new
             at ./src/error.rs:57
   1: my_app::main
             at ./src/main.rs:10
   2: core::ops::function::FnOnce::call_once
             at /rustc/library/core/src/ops/function.rs:250
   3: std::sys::backtrace::__rust_begin_short_backtrace
             at /rustc/library/std/src/sys/backtrace.rs:154
   4: std::rt::lang_start_internal
             at /rustc/library/std/src/rt.rs:175
   5: main
   6: BaseThreadInitThunk
   7: RtlUserThreadStart";

        let simplified = simplify_text(raw);

        // The user's code is retained with its original offset.
        assert!(simplified.contains(" 1: my_app::main"));
        assert!(simplified.contains("core::ops::function::FnOnce::call_once"));

        // Everything from the runtime boundary toward the bottom is dropped and
        // replaced with an annotation (5 frames: the marker plus 4 below it).
        assert!(!simplified.contains("__rust_begin_short_backtrace"));
        assert!(!simplified.contains("std::rt::lang_start_internal"));
        assert!(!simplified.contains("BaseThreadInitThunk"));
        assert!(!simplified.contains("RtlUserThreadStart"));
        assert!(simplified.contains("... skipped 5 frames ..."));
    }

    #[test]
    fn test_simplify_text_falls_back_when_empty() {
        let raw = "not a recognisable backtrace";
        assert_eq!(simplify_text(raw), raw);
    }
}
