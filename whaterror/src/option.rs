use super::{FatalError, Termination};

pub struct NoneError;

impl FatalError<()> for NoneError {
    fn handle(self, _handler: ()) {}
}

impl<T> FatalError<T> for NoneError
where
    T: FnOnce(),
{
    fn handle(self, handler: T) {
        handler();
    }
}

macro_rules! imp {
    () => {
        type Err = NoneError;

        fn into_result(self) -> Result<(), Self::Err> {
            self.ok_or(NoneError)
        }
    };
}

impl<T> Termination<T> for Option<()>
where
    T: FnOnce(),
{
    imp!();
}

impl Termination<()> for Option<()> {
    imp!();
}
