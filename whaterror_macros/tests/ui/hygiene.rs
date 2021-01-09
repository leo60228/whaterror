mod local_hygiene {
    mod whaterror {
        pub const CONSTANT: i32 = 5;
    }

    #[whaterror_macros::whaterror(())]
    fn local_hygiene() {
        let _ = inner;
        let _ = handle;
        let _ = whaterror::CONSTANT;
    }
}

mod naming_conflicts {
    #[whaterror_macros::whaterror(())]
    fn whaterror() {}

    #[whaterror_macros::whaterror(())]
    fn inner() {}

    #[whaterror_macros::whaterror(())]
    fn handle() {}
}

fn main() {}
