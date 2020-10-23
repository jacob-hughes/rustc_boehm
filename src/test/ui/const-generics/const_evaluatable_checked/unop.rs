// run-pass
#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]

struct Foo<const B: bool>;

fn test<const N: usize>() -> Foo<{ !(N > 10) }> where Foo<{ !(N > 10) }>: Sized {
    Foo
}

fn main() {
    let _: Foo<false> = test::<12>();
    let _: Foo<true> = test::<9>();
}
