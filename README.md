# Human Errors
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
use human_errors::{user_with_internal, Error};

fn main() {
    match read_file() {
        Ok(content) => println!("{}", content),
        Err(err) => eprintln!("{}", err),
    }
}

fn read_file() -> Result<String, Error> {
    fs::read_to_string("example.txt").map_err(|err| user_with_internal(
        "We could not read the contents of the example.txt file.",
        "Check that the file exists and that you have permission to access it.",
        err
    ))?
}
```

The above code might result in an error which, when printed, shows the following:

```
The above code might result in an error which, when printed, shows the following:

```
Oh no! We could not read the contents of the example.txt file.

This was caused by:
File Not Found

To try and fix this, you can:
 - Check that the file exists and that you have permission to access it.
```