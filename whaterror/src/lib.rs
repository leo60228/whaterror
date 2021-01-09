use std::convert::Infallible;

pub trait FatalError<T> {
    fn handle(self, handler: T);
}

pub trait Termination<T> {
    type Err: FatalError<T>;

    fn into_result(self) -> Result<(), Self::Err>;
}

impl<T> FatalError<T> for Infallible {
    fn handle(self, _handler: T) {
        match self {}
    }
}

impl<T> Termination<T> for () {
    type Err = Infallible;

    fn into_result(self) -> Result<(), Self::Err> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
