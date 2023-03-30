#![allow(dead_code)]

#[inline(always)]
pub const fn id<T>(x: T) -> T {
    std::convert::identity(x)
}

#[inline(always)]
pub const fn r#const<T, U>(x: T) -> impl FnOnce(U) -> T {
    |_| x
}

#[inline(always)]
pub fn void<T>(_: T) -> () {
    ()
}
