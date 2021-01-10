# `whaterror`

<!-- cargo-sync-readme start -->

*There is nothing at all in your return type, useful or otherwise.*

Proc macro attribute for a more customizable alternative to the standard library's
`Termination` trait.

As a simple example, this replicates the standard behavior for `fn main()` returning
`Result<T, E>`:
```rust
use whaterror::whaterror;

#[whaterror(|err| eprintln!("Error: {:#?}", err))]
fn main() -> Result<(), Error> {
    Err(Error::Failed)
}
```

If your handler returns, `whaterror` will automatically exit with code 1 (or panic if inside a
test).

This also works for `Option<T>`:
```rust
use whaterror::whaterror;

#[whaterror(|| eprintln!("returned None"))]
fn main() -> Option<()> {
    None
}
```

The `||` isn't necessary in this case, since there are no arguments:
```rust
use whaterror::whaterror;

#[whaterror(unreachable!())]
fn main() -> Option<()> {
    Some(())
}
```

This works for non-`()` types just like you would expect. Non-`main` functions are technically
allowed, but currently have very strict limitations.
These limitations are planned to be lifted in the future.

<!-- cargo-sync-readme end -->
