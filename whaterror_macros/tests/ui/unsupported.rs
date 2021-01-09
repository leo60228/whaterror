#![feature(c_variadic)]

use whaterror_macros::whaterror;

#[whaterror(())]
const fn const_fn() {}

#[whaterror(())]
async fn async_fn() {}

#[whaterror(())]
fn generic_fn<T>() {}

#[whaterror(())]
fn arguments(_x: i32) {}

#[whaterror(())]
unsafe extern "C" fn variadic(_arg: i32, _args: ...) {}

#[whaterror(())]
#[warn(warnings)]
fn attrs() {}

#[whaterror(())]
async fn generic_async_args<T>(_x: T) {}

fn main() {}
