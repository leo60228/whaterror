use std::panic::*;
use whaterror::whaterror;

struct PanicMarker;

fn ensure_panic_marker(f: impl FnOnce() + UnwindSafe) {
    let unwound = catch_unwind(f);
    let err = unwound.unwrap_err();
    if !err.is::<PanicMarker>() {
        resume_unwind(err);
    }
}

#[whaterror(())]
fn simple() {}

#[whaterror(unreachable!())]
fn void_with_handler() {}

#[whaterror(())]
fn failing_option() -> Option<()> {
    None
}

#[whaterror(panic!())]
fn impure_handler() -> Option<()> {
    Some(())
}

#[whaterror(panic!(PanicMarker))]
fn simple_handler() -> Option<()> {
    None
}

#[whaterror(|| panic!(PanicMarker))]
fn option_closure_handler() -> Option<()> {
    None
}

#[whaterror(|err| { assert_eq!(err, 123); panic!(PanicMarker); })]
fn result_failing() -> Result<(), i32> {
    Err(123)
}

#[whaterror(|_| ())]
fn result_passing() -> Result<(), ()> {
    Ok(())
}

#[whaterror(drop)]
fn named_handler() -> Result<(), ()> {
    Ok(())
}

#[whaterror(())]
fn no_panic_marker() -> Option<()> {
    None
}

#[whaterror(())]
fn option_return() -> Option<i32> {
    Some(413)
}

#[whaterror(|_| ())]
fn result_return() -> Result<i32, ()> {
    Ok(612)
}

macro_rules! group {
    (#[$attr:meta] mod $group:ident { $($name:ident : $test:expr;)* }) => {
        mod $group {
            $(
                #[test]
                #[$attr]
                fn $name() {
                    #[allow(unused)]
                    use super::*;
                    let _: () = $test;
                }
            )*
        }
    }
}

macro_rules! groups {
    ($(#[$attr:meta] mod $group:ident $body:tt)+) => {
        $(group!(#[$attr] mod $group $body);)+
    }
}

groups! {
    #[cfg(all())]
    mod should_pass {
        simple: simple();
        void_with_handler: void_with_handler();
        impure_handler: impure_handler();
        simple_handler: ensure_panic_marker(simple_handler);
        option_closure_handler: ensure_panic_marker(simple_handler);
        result_failing: ensure_panic_marker(simple_handler);
        result_passing: result_passing();
        named_handler: named_handler();
        option_return: assert_eq!(option_return(), 413);
        result_return: assert_eq!(result_return(), 612);
    }

    #[should_panic]
    mod should_fail {
        failing_option: failing_option();
        no_panic_marker: ensure_panic_marker(no_panic_marker);
    }
}
