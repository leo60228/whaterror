//! whaterror

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
