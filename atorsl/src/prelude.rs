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
