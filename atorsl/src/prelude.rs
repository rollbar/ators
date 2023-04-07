pub(crate) trait IsSomeAnd<T> {
    #[must_use]
    fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> IsSomeAnd<T> for Option<T> {
    /// Returns `true` if the option is a [`Some`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(is_some_and)]
    ///
    /// let x: Option<u32> = Some(2);
    /// assert_eq!(x.is_some_and(|x| x > 1), true);
    ///
    /// let x: Option<u32> = Some(0);
    /// assert_eq!(x.is_some_and(|x| x > 1), false);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.is_some_and(|x| x > 1), false);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(is_some_and)]
    ///
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.is_ok_and(|x| x > 1), true);
    ///
    /// let x: Result<u32, &str> = Ok(0);
    /// assert_eq!(x.is_ok_and(|x| x > 1), false);
    ///
    /// let x: Result<u32, &str> = Err("hey");
    /// assert_eq!(x.is_ok_and(|x| x > 1), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_ok_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Err(_) => false,
            Ok(x) => f(x),
        }
    }

    /// Returns `true` if the result is [`Err`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(is_some_and)]
    /// use std::io::{Error, ErrorKind};
    ///
    /// let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
    /// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), true);
    ///
    /// let x: Result<u32, Error> = Err(Error::new(ErrorKind::PermissionDenied, "!"));
    /// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);
    ///
    /// let x: Result<u32, Error> = Ok(123);
    /// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);
    /// ```
    #[must_use]
    #[inline]
    fn is_err_and(self, f: impl FnOnce(E) -> bool) -> bool {
        match self {
            Ok(_) => false,
            Err(e) => f(e),
        }
    }
}
