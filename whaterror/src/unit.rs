use super::{FatalError, Termination};
use std::convert::Infallible;

impl<T> FatalError<T> for Infallible {
    fn handle(self, _handler: T) {
        match self {}
    }
}

impl Termination for () {
    type Ok = ();
    type Err = Infallible;

    fn into_result(self) -> Result<(), Self::Err> {
        Ok(self)
    }
}
