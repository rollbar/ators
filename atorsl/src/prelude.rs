#![allow(unstable_name_collisions)]

pub(crate) trait IsSomeAnd<T> {
    #[must_use]
    fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> IsSomeAnd<T> for Option<T> {
    #[must_use]
    #[inline]
    fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            None => false,
            Some(x) => f(x),
        }
    }
}
