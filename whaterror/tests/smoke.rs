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

macro_rules! group {
    (#[$attr:meta] mod $group:ident { $($name:ident : $test:ident $args:tt;)* }) => {
        mod $group {
            $(
                #[test]
                #[$attr]
                fn $name() {
                    #[allow(unused)]
                    use super::*;
                    let _: () = super::$test $args;
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
    }

    #[should_panic]
    mod should_fail {
        failing_option: failing_option();
    }
}
