pub(crate) trait IsSomeAnd<T> {
    #[must_use]
    fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> IsSomeAnd<T> for Option<T> {
    /// Returns `true` if the option is a [`Some`] and the value inside of it matches a predicate.
    #[must_use]
    #[inline]
    fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            None => false,
            Some(x) => f(x),
        }
    }
}

pub(crate) trait IsOkAnd<T, E> {
    #[must_use]
    fn is_ok_and(self, f: impl FnOnce(T) -> bool) -> bool;

    #[must_use]
    fn is_err_and(self, f: impl FnOnce(E) -> bool) -> bool;
}

impl<T, E> IsOkAnd<T, E> for Result<T, E> {
    /// Returns `true` if the result is [`Ok`] and the value inside of it matches a predicate.
    #[must_use]
    #[inline]
    fn is_ok_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Err(_) => false,
            Ok(x) => f(x),
        }
    }

    /// Returns `true` if the result is [`Err`] and the value inside of it matches a predicate.
    #[must_use]
    #[inline]
    fn is_err_and(self, f: impl FnOnce(E) -> bool) -> bool {
        match self {
            Ok(_) => false,
            Err(e) => f(e),
        }
    }
}
