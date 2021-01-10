use super::{FatalError, Termination};

pub struct Error<T>(T);

impl<T, F> FatalError<F> for Error<T>
where
    F: FnOnce(T),
{
    fn handle(self, handler: F) {
        handler(self.0);
        std::process::exit(1);
    }
}

impl<T, F> Termination<F> for Result<(), T>
where
    F: FnOnce(T),
{
    type Err = Error<T>;

    fn into_result(self) -> Result<(), Error<T>> {
        self.map_err(Error)
    }
}
