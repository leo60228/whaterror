pub mod option;
pub mod result;
mod unit;

#[doc(transparent)]
pub use whaterror_macros::*;

pub trait FatalError<T> {
    fn handle(self, handler: T);
}

pub trait Termination {
    type Ok;
    type Err;

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
