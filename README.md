# Human Errors [![crate](https://img.shields.io/crates/v/human-errors)](https://crates.io/crates/human-errors) [![docs](https://docs.rs/human-errors/badge.svg)](https://docs.rs/human-errors)

**Errors which make your users' lives easier**

This crate provides an `Error` type which has been designed to make errors
something which help guide your users through your application rather than
blocking their progress. It has fundamentally been designed with the expectation
that any failure can be mitigated (even if that means cutting a GitHub issue)
and that explaining to your user how to do so is the fastest way to get them
moving again.

## Features

- **Advice** on how to resolve a problem is a fundamental requirement for the creation of an error,
   making your developers think about the user experience at the point they write the code.
- **Wrapping** allows you to expose a causal chain which may incorporate advice from multiple layers
   in the stack - giving users a better sense of what failed and how to fix it.
- **Integration** with the `std::error::Error` type allows you to wrap any `Box`-able error in the
   causal chain and provide additional context.

## Example

```rust
use std::fs;
use human_errors::{Error, ResultExt};

fn main() {
    match read_file() {
        Ok(content) => println!("{}", content),
        Err(err) => eprintln!("{}", err),
    }
}

fn read_file() -> Result<String, Error> {
    fs::read_to_string("example.txt").wrap_err_as_user(
        "We could not read the contents of the example.txt file.",
        &["Check that the file exists and that you have permission to access it."]
    ))?
}
```

The above code might result in an error which, when printed, shows the following:

```txt
Oh no! We could not read the contents of the example.txt file.

This was caused by:
File Not Found

To try and fix this, you can:
 - Check that the file exists and that you have permission to access it.
```

## Getting Started

The easiest way to construct a human-errors `Error` is to use the `user_error`
and `system_error` helper functions. These allow you to quickly create errors
of either kind with a message and advice for how to resolve the issue.

```rust
use human_errors;

human_errors::user_error(
    "We could not find the configuration file.",
    &["Make sure that you've specified a valid config file with the --config option."]
);

human_errors::system_error(
    "A low-level IO error occurred.",
    &["Please try again later or contact support if the issue persists."]
);
```

## Wrapping Other Errors

When working with errors from other crates and the standard library, you will
often find yourself needing to convert those errors into `human_errors` error
types. We provide several helper methods which assist with this, including the
`user` and `system` functions for wrapping existing errors, and their siblings
`wrap_user` and `wrap_system` which allow you to add an additional error message
to provide more context if needed.

```rust
use human_errors;

let err = "0.not-a-number".parse::<u32>().unwrap_err();

human_errors::user(
    err,
    &["Make sure that you've provided a valid unsigned integer."]
)

human_errors::wrap_user(
    err,
    "We could not parse the user ID you provided.",
    &["Make sure that you've provided a valid unsigned integer."]
)
```

### Working with Results

To make working with `Result` types easier, we provide the `ResultExt` trait
which adds several helper methods to the standard `Result` type. These include
`wrap_err_as_user` and `wrap_err_as_system` which allow you to easily convert
any error in a `Result` into a human-errors `Error` of the desired kind.

```rust
use std::fs;
use human_errors::{Error, ResultExt};

fn read_file() -> Result<String, Error> {
    fs::read_to_string("example.txt").wrap_err_as_user(
        "We could not read the contents of the example.txt file.",
        &["Check that the file exists and that you have permission to access it."]
    ))
}
```
