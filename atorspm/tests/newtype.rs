#![allow(clippy::no_effect, unused_must_use)]

use atorspm::{Deref, DerefMut, From, Newtype};

struct B<T>(T);

#[derive(Deref, DerefMut, From)]
struct A(B<()>);

#[derive(Newtype)]
struct AA(B<()>);

// [todo] Newtype generics support
// #[derive(Newtype)]
// struct AAA<T>(B<T>);

// [todo] test proper erroring
// error: proc-macro derive panicked
//    = help: message: Only trivial tuple structs wrapping over a single field can be derived.
//#[derive(Newtype)]
//struct AAA();
//struct AAA(B<()>, ());
//struct AAA { a: B<()>, }

fn main() {
    // Deref
    A(B(())).0 .0 as ();
    (*A(B(()))).0 as ();

    fn f<T>(_: &B<T>) {}
    fn g(_: &A) {}
    fn h(_: A) {}
    f(&A(B(())));
    g(&A(B(())));
    h(A(B(())));

    // From
    A::from(B(()));
    let _: A = B(()).into();
    <B<()> as Into<A>>::into(B(()));

    // Deref
    AA(B(())).0 .0 as ();
    (*AA(B(()))).0 as ();

    fn ff<T>(_: &B<T>) {}
    fn gg(_: &AA) {}
    fn hh(_: AA) {}
    ff(&AA(B(())));
    gg(&AA(B(())));
    hh(AA(B(())));

    // From
    AA::from(B(()));
    let _: AA = B(()).into();
    <B<()> as Into<AA>>::into(B(()));
}
