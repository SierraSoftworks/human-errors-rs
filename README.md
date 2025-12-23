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
        Err(err) => eprintln!("{}", human_errors::pretty(err)),
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

The easiest way to construct a human-errors `Error` is to use the `user`
and `system` helper functions. These allow you to quickly create errors
of either kind with a message and advice for how to resolve the issue.

```rust
use human_errors;

human_errors::user(
    "We could not find the configuration file.",
    &["Make sure that you've specified a valid config file with the --config option."]
);

human_errors::system(
    "A low-level IO error occurred.",
    &["Please try again later or contact support if the issue persists."]
);
```

These methods can also be used to wrap existing errors, adding advice on how to best
deal with them.

```rust
use std::fs;

fn read_config() -> Result<String, human_errors::Error> {
    fs::read_to_string("config.toml").map_err(|err| {
        human_errors::user(
            err,
            &["Make sure that you've specified a valid config file with the --config option."]
        )
    })
}
```

If you find yourself wanting to add a better error message, you can use the `wrap_user`
and `wrap_system` methods to add additional context to an existing error.

```rust
use std::fs;
use human_errors;

fn read_config() -> Result<String, human_errors::Error> {
    fs::read_to_string("config.toml").map_err(|err| {
        human_errors::wrap_user(
            err,
            "We could not read the 'config.toml' configuration file.",
            &[
                "Make sure that the 'config.toml' configuration file exists.",
                "Ensure that you have permission to read the 'config.toml' file.",
            ]
        )
    })
}
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

## Pretty Printing

Errors produced by this library implement the `Display` trait to provide a
human-friendly rendering of the error message and its advice. However, if you
want to customize the rendering further, you can use the `pretty` function
to get a pre-rendered string representation of the error.

**NOTE**: By default, this function is the same as using the `Display` implementation,
but when the `cli` feature is enabled, it will format the error using coloured output
and unicode box-drawing characters for an improved terminal experience.

```rust
use human_errors;

let err = human_errors::user(
    "We could not connect to the database.",
    &["Check that the database server is running.", "Verify your network connection."]
);

eprintln!("{}", human_errors::pretty(err));
```
