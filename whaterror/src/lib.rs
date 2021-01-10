mod option;
mod result;
mod unit;

#[doc(transparent)]
pub use whaterror_macros::*;

pub trait FatalError<T> {
    fn handle(self, handler: T);
}

pub trait Termination<T> {
    type Err: FatalError<T>;

    fn into_result(self) -> Result<(), Self::Err>;
}
