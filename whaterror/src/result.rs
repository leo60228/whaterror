use super::{FatalError, Termination};

pub struct Error<E>(E);

impl<E, F> FatalError<F> for Error<E>
where
    F: FnOnce(E),
{
    fn handle(self, handler: F) {
        handler(self.0);
    }
}

impl<T, E> Termination for Result<T, E> {
    type Ok = T;
    type Err = Error<E>;

    fn into_result(self) -> Result<T, Error<E>> {
        self.map_err(Error)
    }
}
