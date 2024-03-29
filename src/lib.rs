//! Errors which make your users' lives easier.
//!
//! Provides a framework through which you can expose error chains
//! which include advice for how users can respond to (and hopefully
//! resolve) a failure. Designed to make you treat recovery from failure
//! as a fundamental part of the design process in your application.

mod description;
mod error;
mod from;
mod helpers;
#[macro_use]
mod extend;

pub use description::*;
pub use error::*;
pub use helpers::*;
