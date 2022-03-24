// run-rustfix
#![warn(clippy::empty_drop)]

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {}
}

fn main() {
    let s = Foo;
    // s goes out of scope.
}
