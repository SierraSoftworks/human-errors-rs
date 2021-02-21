# Human Errors
**Errors which make your users' lives easier**

This crate provides an `Error` type which has been designed to make errors
something which help guide your users through your application rather than
blocking their progress. It has fundamentally been designed with the expectation
that any failure can be mitigated (even if that means cutting a GitHub issue)
and that explaining to your user how to do so is the fastest way to get them
moving again.

## Example

```rust
use human_errors;

fn get_config() -> Result<(), human_errors::Error> {
    let config = read_config().map_err(|err| human_errors::user_with_cause(
        "We could not open the config file you provided.",
        "Make sure that you've specified a valid config file with the --config option.",
        err
    ))?;

    parse_config(&config)?
}

fn read_config() -> Result<String, human_errors::Error> {
    human_errors::user(
        "We could not find a file at /home/user/.config/demo.yml",
        "Make sure that the file exists and is readable by the application."
    )?;
}

fn parse_config(config: &str) -> Result<(), human_errors::Error> {
    human_errors::user_with_internal(
        "We could not parse the config file because it is not in a supported format.",
        "Make sure you're using the latest config file format or read our migration docs at https://example.com/migrate for help migrating.",
        human_errors::description("Found config version 1.7, but required 2.x"),
    )?;
}
```

The above code might result in an error which, when printed, shows the following:

```
Oh no! We could not open the config file you provided.

This was caused by:
We could not find a file at /home/user/.config/demo.yml

To try and fix this, you can:
 - Make sure that the file exists and is readable by the application.
 - Make sure that you've specified a valid config file with the --config option.
```