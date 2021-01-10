//! *There is nothing at all in your return type, useful or otherwise.*
//!
//! Proc macro attribute for a more customizable alternative to the standard library's
//! `Termination` trait.
//!
//! As a simple example, this replicates the standard behavior for `fn main()` returning
//! `Result<T, E>`:
//! ```should_panic
//! # #[derive(Debug)]
//! # enum Error {
//! #     Failed,
//! # }
//! use whaterror::whaterror;
//!
//! #[whaterror(|err| eprintln!("Error: {:#?}", err))]
//! fn main() -> Result<(), Error> {
//!     Err(Error::Failed)
//! }
//! ```
//!
//! If your handler returns, `whaterror` will automatically exit with code 1 (or panic if inside a
//! test).
//!
//! This also works for `Option<T>`:
//! ```should_panic
//! use whaterror::whaterror;
//!
//! #[whaterror(|| eprintln!("returned None"))]
//! fn main() -> Option<()> {
//!     None
//! }
//! ```
//!
//! The `||` isn't necessary in this case, since there are no arguments:
//! ```
//! use whaterror::whaterror;
//!
//! #[whaterror(unreachable!())]
//! fn main() -> Option<()> {
//!     Some(())
//! }
//! ```
//!
//! This works for non-`()` types just like you would expect. Non-`main` functions are technically
//! allowed, but currently have very strict limitations.
//! These limitations are planned to be lifted in the future.

#![warn(missing_docs)]

pub mod option;
pub mod result;
mod unit;

#[doc(transparent)]
pub use whaterror_macros::*;

/// Represents an error to be handled by `whaterror`.
pub trait FatalError<T> {
    /// Handle the error using the provided handler.
    fn handle(self, handler: T);
}

/// Represents a return value to be handled by `whaterror`.
pub trait Termination {
    /// Type for a successful result.
    type Ok;

    /// Type for an unsuccessful result. This should implement FatalError<T> for at least one type.
    type Err;

    /// Convert to a Result<T> for handling.
    fn into_result(self) -> Result<Self::Ok, Self::Err>;
}

/// USED BY MACRO, NOT PUBLIC API
#[doc(hidden)]
pub fn terminate(test: bool) -> ! {
    if test {
        panic!()
    } else {
        std::process::exit(1)
    }
}
