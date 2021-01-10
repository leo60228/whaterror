//! Implementations for `Option<T>`.

use super::{FatalError, Termination};

/// Error used for a `None` value.
pub struct NoneError;

impl FatalError<()> for NoneError {
    fn handle(self, _handler: ()) {}
}

impl<F> FatalError<F> for NoneError
where
    F: FnOnce(),
{
    fn handle(self, handler: F) {
        handler();
    }
}

impl<T> Termination for Option<T> {
    type Ok = T;
    type Err = NoneError;

    fn into_result(self) -> Result<Self::Ok, Self::Err> {
        self.ok_or(NoneError)
    }
}
