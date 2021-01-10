use whaterror::whaterror;

#[whaterror(())]
fn simple() {}

#[whaterror(unreachable!())]
fn void_with_handler() {}

#[whaterror(())]
fn failing_option() -> Option<()> {
    None
}

#[test]
fn should_pass() {
    let _: () = simple();
    let _: () = void_with_handler();
}

#[test]
#[should_panic]
fn should_fail() {
    let _: () = failing_option();
}
