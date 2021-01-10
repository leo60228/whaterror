use whaterror_macros::whaterror;

#[whaterror(())]
const NOT_A_FUNCTION: () = ();

#[whaterror]
fn no_handler() {}

#[whaterror]
const NON_FUNCTION_WITHOUT_HANDLER: () = ();

#[whaterror]
const fn const_no_handler() {}

fn main() {}
